// Hide console window on Windows platform, never remove it
// #![windows_subsystem = "windows"]

mod app;
mod config;
mod entity;
mod filemanager;
mod media;
mod sound;
mod ui;
mod util;

use std::{sync::{
    atomic::{AtomicI16, AtomicI64},
    RwLock,
}, iter::Map};

use crossbeam::channel::{unbounded, Receiver, Sender};
use entity::EventMessage;
use media::decoder::{AudioBuffer, SubtitleBuffer, VideoBuffer};
use media::decoder::{AudioSummary, SubtitleSummary, VideoSummary};
use static_init::dynamic;

/**
 * Global const or static variables
 * Warning: Only global variables for the whole application are defined here,
 * please define the local const or static variables in their own modules,
 * don't put them here.
 */

const APP_NAME: &str = "NT Player";
const LOGO_PATH: &str = "./assets/logo.png";
const INIT_WIDTH: u32 = 1024;
const INIT_HEIGHT: u32 = 768;

/// Forward or rewind amount each time, Unit: milliseconds
const FR_STEP: i64 = 10000;

/// Global volume, modify this value will affect to the play volume
static VOLUME: AtomicI16 = AtomicI16::new(0);
static VOLUME_STEP: i16 = 10;
const MAX_VOLUME: i16 = 2000;
const VOLUME_BENCHMARK: f32 = 50.0;

/// Global play timestamp, unit milliseconds+
static AUDIO_PTS_MILLIS: AtomicI64 = AtomicI64::new(0);
static AUDIO_SUMMARY: RwLock<Option<AudioSummary>> = RwLock::new(None);
// It's bettrer to give more buffers for audio,
// becuase humans are more sensitive to sound than video.
// In other words, video frames can be exhausted before audio frames,
// but not vice versa.
#[dynamic]
static AUDIO_BUFFER: AudioBuffer = AudioBuffer::new(50);

/// Global video timestamp, unit milliseconds
static VIDEO_PTS_MILLIS: AtomicI64 = AtomicI64::new(0);
static VIDEO_SUMMARY: RwLock<Option<VideoSummary>> = RwLock::new(None);
#[dynamic]
static VIDEO_BUFFER: VideoBuffer = VideoBuffer::new(10);

static SUBTITLE_SUMMARY: RwLock<Option<SubtitleSummary>> = RwLock::new(None);
#[dynamic]
static SUBTITLE_BUFFER: SubtitleBuffer = SubtitleBuffer::new(5);

pub type EventSender = Sender<EventMessage>;
pub type EventReceiver = Receiver<EventMessage>;
#[dynamic]
static EVENT_CHANNEL: (EventSender, EventReceiver) = unbounded();

/// The path list for playing
static mut PLAY_LIST: Vec<(String, Vec<String>)> = Vec::new();

fn main() {
    use app::NtApp;
    use log::error;
    use ui::components::dialog::show_error;
    use util::{error::SuperError, log_builder};

    log_builder::init_logger(log::LevelFilter::Debug);

    let run = || -> Result<(), SuperError> {
        let mut app = NtApp::new()?;
        app.run()?;

        Ok(())
    };

    if let Err(err) = run() {
        error!("{}", err);
        show_error(err.to_string().as_str());
    }
}


