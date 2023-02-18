use crossbeam::atomic::AtomicCell;
use rodio::{source::Source, OutputStream, Sample, Sink};
use std::{
    cell::Cell,
    sync::{Arc},
    thread::{self, JoinHandle},
};

use super::traits::Player;
use crate::{
    media::decoder::AudioSummary,
    util::error::{handle_result, SuperError},
};

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

pub struct AudioPlayer {
    /// State of the audio player
    state: Arc<AtomicCell<State>>,
    /// Handle to the device that output sounds
    sink: Arc<Sink>,
    /// Thread id
    tid: Cell<Option<JoinHandle<()>>>,
}

impl AudioPlayer {
    /// `es`: System event sender
    /// `ts`: Benchmark timestamp sender
    pub fn new() -> Self {
        let sink = handle_result(Self::get_sink()).expect("Initialize audio context failed!");

        Self {
            sink: Arc::new(sink),
            state: Arc::new(AtomicCell::new(State::Stopped)),
            tid: Cell::new(None),
        }
    }

    /// Set the buffer queue which will be used for audio play
    pub fn start(&mut self, summary: &Option<AudioSummary>) {
        // If the thread is already running, stop it first
        if let Some(_tid) = self.tid.take() {
            if !_tid.is_finished() {
                self.state.store(State::ReadyToStop);
                _tid.join().expect("Join audio thread failed!");
            }
        }

        // Loop to play audio in new thread
        let state = self.state.clone();
        let sink = self.sink.clone();
        let tid = thread::spawn({
            state.store(State::Playing);
            move || loop {
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

                // Play audio
                // if let Some(audio_data) = buffer.pop() {
                //     // Update timestamp of playing
                //     ts.store(
                //         audio_data.pts,
                //         std::sync::atomic::Ordering::Release,
                //     );

                //     let timeout = 1 / audio_data.fps;
                // handle_result(Self::play_audio(&sink, audio_data), &es).unwrap();

                // thread::sleep(Duration::from_millis(timeout as u64));
                // }
            }
        });

        // self.tid.replace(Some(tid));
    }

    fn play_audio<S>(sink: &Arc<Sink>, source: S) -> Result<(), SuperError>
    where
        S: Source + Send + 'static,
        S::Item: Sample + Send,
    {
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }

    fn get_sink() -> Result<Sink, SuperError> {
        let (_, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(sink)
    }
}

impl Player for AudioPlayer {
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

#[cfg(test)]
mod tests {
    use rodio::{source::SineWave, Source};
    use std::{sync::Arc, time::Duration};

    use super::AudioPlayer;

    #[test]
    fn test_play_audio() {
        let sink = &Arc::new(AudioPlayer::get_sink().unwrap());
        let source = SineWave::new(200.0)
            .take_duration(Duration::from_secs_f32(3.0))
            .amplify(0.20);
        AudioPlayer::play_audio(sink, source).expect("failed to play audio");
    }
}
