use crossbeam::atomic::AtomicCell;
use log::debug;
use std::{
    cell::Cell,
    sync::{atomic::Ordering, Arc},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{
    entity::EventMessage,
    global_variables::{AUDIO_PTS, EVENT_CHANNEL, MEDIA_TIMESTAMP_SYNC_DIFF, VIDEO_BUFFER},
    media::decoder::VideoSummary, util::error::handle_send_result,
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

    pub fn start(&mut self, summary: &Option<VideoSummary>) {
        if summary.is_none() {
            return;
        }

        let summary = summary.as_ref().unwrap();
        let state = self.state.clone();

        // If the thread is already running, stop it first
        if let Some(_tid) = self.tid.take() {
            if !_tid.is_finished() {
                _tid.join().expect("Join audio thread failed!");
            }
        }

        //
        // Loop to play video in new thread
        //
        let sender = &EVENT_CHANNEL.0;

        let timebase = summary.time_base_num / summary.time_base_den;
        // Total duration of the video with unit seconds
        let total_duration = summary.duration * timebase;
        // Sleep interval with unit milliseconds
        let interval = total_duration * 1000 / summary.frames;
        let sleep_duration = Duration::from_millis(interval);
        let diff_benchmark = (MEDIA_TIMESTAMP_SYNC_DIFF / timebase) as i64;
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
                        if (AUDIO_PTS.load(Ordering::Acquire) - frame.pts) > diff_benchmark {
                            // Audio pts > video pts, skip this frame to catch up the audio timestamp
                            debug!("Audio pts exceeded video timestamp out of range, skip current video frame");
                            // Here we want to skip to spcified position rapidly, so don't sleep
                            continue;
                        } else {
                            while (AUDIO_PTS.load(Ordering::Acquire) - frame.pts) < -diff_benchmark
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

        self.tid.replace(Some(tid));
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
