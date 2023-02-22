pub mod audio_player;
pub mod traits;
pub mod vedio_player;

use crate::util::error::SuperError;

use self::{audio_player::AudioPlayer, traits::Player, vedio_player::VideoPlayer};

pub struct MediaPlayer {
    audio_player: AudioPlayer,
    video_player: VideoPlayer,
    state: PlayerState,
}

impl MediaPlayer {
    pub fn new() -> Self {
        let ap = AudioPlayer::new();
        let vp = VideoPlayer::new();

        Self {
            audio_player: ap,
            video_player: vp,
            state: PlayerState::NONE,
        }
    }

    pub fn start(&mut self)-> Result<(), SuperError>{
        let audio_player = &mut self.audio_player;
        let video_player = &mut self.video_player;

        audio_player.start();
        video_player.start()?;

        Ok(())
    }
}

impl Player for MediaPlayer {
    fn play(&mut self) {
        self.audio_player.play();
        self.video_player.play();
        self.state = PlayerState::PLAYING;
    }

    fn pause(&mut self) {
        self.audio_player.pause();
        self.video_player.pause();
        self.state = PlayerState::PAUSING;
    }

    fn resume(&mut self) {
        self.audio_player.resume();
        self.video_player.resume();
        self.state = PlayerState::PLAYING;
    }

    fn stop(&mut self) {
        self.audio_player.stop();
        self.video_player.stop();
        self.state = PlayerState::STOPPED;
    }

    fn fast_forward(&mut self) {
        self.audio_player.fast_forward();
        self.video_player.fast_forward();
        self.state = PlayerState::PLAYING;
    }

    fn fast_rewind(&mut self) {
        self.audio_player.fast_rewind();
        self.video_player.fast_rewind();
        self.state = PlayerState::PLAYING;
    }
}

enum PlayerState {
    NONE,
    PLAYING,
    PAUSING,
    STOPPED,
}