use crossbeam::channel::unbounded;

use sdl2::pixels::Color;
use static_init::dynamic;
use std::{sync::{atomic::{AtomicI64, AtomicI16}, RwLock}, mem::MaybeUninit};

use crate::{
    media::decoder::{AudioBuffer, SubtitleBuffer, VideoBuffer, AudioSummary, VideoSummary, SubtitleSummary},
    EventMessage,
};
use crossbeam::channel::{Receiver, Sender};

//
//
//  Normal static variables
//
//

//
// App related
//
pub const APP_NAME: &str = "NT Player";
pub const LOGO_PATH: &str = "./assets/logo.png";
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::RGB(40, 40, 40);
pub const INIT_WIDTH: u32 = 1024;
pub const INIT_HEIGHT: u32 = 608;

//
// Media related
//
/// Unit: milliseconds
pub const MEDIA_TIMESTAMP_SYNC_DIFF: i64 = 200;
/// Forward or rewind amount each time, Unit: milliseconds
pub const FR_STEP: i64 = 10000;
/// Maximum number of frames that can be hold in `AUDIO_SUMMARY` and `VIDEO_SUMMARY`
const MEDIA_BUFFER_SIZE: usize = 10;
/// Global volume, modify this value will affect to the play volume
pub static VOLUME: AtomicI16 = AtomicI16::new(50);
pub static VOLUME_STEP: i16 = 10;
pub const MAX_VOLUME: i16 = 5000;
pub const VOLUME_BENCHMARK: f32 = 50.0;
/// Global play timestamp, unit milliseconds+
pub static GLOBAL_PTS_MILLIS: AtomicI64 = AtomicI64::new(0);
pub static AUDIO_SUMMARY: RwLock<Option<AudioSummary>> = RwLock::new(None);
pub static VIDEO_SUMMARY: RwLock<Option<VideoSummary>> = RwLock::new(None);
pub static SUBTITLE_SUMMARY: RwLock<Option<SubtitleSummary>> = RwLock::new(None);

pub type EventSender = Sender<EventMessage>;
pub type EventReceiver = Receiver<EventMessage>;

//
//
// Lazy static variables
//
//

#[dynamic]
pub static EVENT_CHANNEL: (EventSender, EventReceiver) = unbounded();
#[dynamic]
pub static AUDIO_BUFFER: AudioBuffer = AudioBuffer::new(MEDIA_BUFFER_SIZE);
#[dynamic]
pub static VIDEO_BUFFER: VideoBuffer = VideoBuffer::new(MEDIA_BUFFER_SIZE);
#[dynamic]
pub static SUBTITLE_BUFFER: SubtitleBuffer = SubtitleBuffer::new(MEDIA_BUFFER_SIZE);
