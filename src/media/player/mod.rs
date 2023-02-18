pub mod audio_player;
pub mod traits;
pub mod vedio_player;

use self::{audio_player::AudioPlayer, traits::Player, vedio_player::VideoPlayer};
use super::decoder::MediaSummary;
use crate::entity::PlayerState;

pub struct MediaPlayer {
    pub audio_player: AudioPlayer,
    pub video_player: VideoPlayer,
    pub state: PlayerState,
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

    pub fn start(&mut self, media_summary: &MediaSummary) {
        let ap = &mut self.audio_player;
        let vp = &mut self.video_player;

        let (audio_summary, video_summary, subtitle_summary) = media_summary;
        ap.start(audio_summary);
        vp.start(video_summary);
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
