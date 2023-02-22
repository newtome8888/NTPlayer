use crossbeam::atomic::AtomicCell;
use log::{debug, info};
// use tracing::{info, debug};
use std::{
    cell::Cell,
    sync::{atomic::Ordering, Arc},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{
    entity::EventMessage,
    global::{AUDIO_PTS_MILLIS, EVENT_CHANNEL, MEDIA_TIMESTAMP_SYNC_DIFF, VIDEO_BUFFER, VIDEO_SUMMARY},
    media::decoder::VideoSummary,
    util::error::{handle_send_result, SuperError},
};

use super::traits::Player;

pub struct VideoPlayer {
    /// State of the audio player
    state: Arc<AtomicCell<State>>,
    /// Thread id
    tid: Cell<Option<JoinHandle<()>>>,
}

impl VideoPlayer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(AtomicCell::new(State::Stopped)),
            tid: Cell::new(None),
        }
    }

    pub fn start(&mut self) -> Result<(), SuperError> {
        let summary = VIDEO_SUMMARY.read().unwrap();
        if summary.is_none() {
            return Ok(());
        }

        // If the thread is already running, stop it first
        if let Some(_tid) = self.tid.take() {
            if !_tid.is_finished() {
                _tid.join().expect("Join audio thread failed!");
            }
        }

        let sender = &EVENT_CHANNEL.0;
        let summary = summary.as_ref().unwrap();
        info!("Starting video player, summary: {:?}", summary);
        sender.send(EventMessage::Resize((summary.width, summary.height)))?;

        let state = self.state.clone();
        let sleep_duration = Duration::from_millis(summary.play_interval);

        let tid = thread::spawn({
            move || {
                state.store(State::Playing);
                loop {
                    // Check player state
                    match state.load() {
                        // Ready to stop or already stopped, exit thread
                        State::ReadyToStop => {
                            state.store(State::Stopped);
                            break;
                        }
                        State::Stopped => break,
                        // Ready to pause or already paused,
                        // do nothing and continue loop
                        State::ReadyToPause => {
                            state.store(State::Paused);
                            continue;
                        }
                        State::Paused => continue,
                        // Ready to play or already playing, just go on
                        State::ReadyToPlay | State::ReadyToResume => {
                            state.store(State::Playing);
                        }
                        State::Playing => {
                            // go on
                        }
                    }

                    // Play video
                    if let Some(frame) = VIDEO_BUFFER.pop() {
                        let frame = Arc::new(frame);
                        if (AUDIO_PTS_MILLIS.load(Ordering::Acquire) - frame.pts_millis)
                            > MEDIA_TIMESTAMP_SYNC_DIFF
                        {
                            // Audio pts > video pts, skip this frame to catch up the audio timestamp
                            debug!("Audio pts exceeded video timestamp out of range, skip current video frame");
                            // Here we want to skip to spcified position rapidly, so don't sleep
                            continue;
                        } else {
                            while (AUDIO_PTS_MILLIS.load(Ordering::Acquire) - frame.pts_millis)
                                < -MEDIA_TIMESTAMP_SYNC_DIFF
                            {
                                // Audio pts < video pts, repeate send current frame to wait for audio timestamp
                                debug!("Audio pts delay of video timestamp out of range, repeat current frame");
                                let result = sender.send(EventMessage::RenderVideo(frame.clone()));
                                handle_send_result(result);

                                thread::sleep(sleep_duration);
                            }
                        }

                        // Send video data to UI
                        let result = sender.send(EventMessage::RenderVideo(frame.clone()));
                        handle_send_result(result);

                        thread::sleep(sleep_duration);
                    }
                }
            }
        });

        self.tid.set(Some(tid));

        Ok(())
    }
}

impl Player for VideoPlayer {
    fn play(&mut self) {
        self.state.store(State::ReadyToPlay);
    }

    fn pause(&mut self) {
        self.state.store(State::ReadyToPause);
    }

    fn resume(&mut self) {
        self.state.store(State::ReadyToResume);
    }

    fn stop(&mut self) {
        self.state.store(State::ReadyToStop);
    }

    fn fast_forward(&mut self) {
        todo!();
    }

    fn fast_rewind(&mut self) {
        todo!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Playing,
    Paused,
    Stopped,

    ReadyToPlay,
    ReadyToPause,
    ReadyToResume,
    ReadyToStop,
}
