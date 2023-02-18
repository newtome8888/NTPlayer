use crossbeam::channel::unbounded;

use sdl2::pixels::Color;
use static_init::dynamic;
use std::sync::atomic::AtomicI64;

use crate::{
    media::decoder::{AudioBuffer, SubtitleBuffer, VideoBuffer},
    EventMessage,
};
use crossbeam::channel::{Receiver, Sender};

//
//
//  No lazy static variables
//
//

// App related
pub const APP_NAME: &str = "NT Player";
pub const LOGO_PATH: &str = "./assets/logo.png";
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::RGB(40, 40, 40);
pub const INIT_WIDTH: u32 = 1024;
pub const INIT_HEIGHT: u32 = 768;

// Media related
/// Unit: milliseconds
pub const MEDIA_TIMESTAMP_SYNC_DIFF: u64 = 200;
/// Unit: milliseconds
pub const FORWARD_REWIND_AMOUNT: i64 = 10000;
const MEDIA_BUFFER_SIZE: usize = 50;
/// Global play timestamp, unit microseconds
pub static AUDIO_PTS: AtomicI64 = AtomicI64::new(0);

//
//
// Lazy static variables
//
//

pub type EventSender = Sender<EventMessage>;
pub type EventReceiver = Receiver<EventMessage>;
#[dynamic]
pub static EVENT_CHANNEL: (EventSender, EventReceiver) = unbounded();
#[dynamic]
pub static AUDIO_BUFFER: AudioBuffer = AudioBuffer::new(MEDIA_BUFFER_SIZE);
#[dynamic]
pub static VIDEO_BUFFER: VideoBuffer = VideoBuffer::new(MEDIA_BUFFER_SIZE);
#[dynamic]
pub static SUBTITLE_BUFFER: SubtitleBuffer = SubtitleBuffer::new(MEDIA_BUFFER_SIZE);
