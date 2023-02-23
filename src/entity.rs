use std::{path::PathBuf, sync::Arc};
use rsmpeg::ffi::AVFrame;
use sdl2::video::WindowPos;

use crate::media::decoder::{VideoFrame, AudioFrame, SubtitleFrame};

/// Message types for application related events
pub enum EventMessage{
    /// Show error dialog
    ShowError(String),

    // For media state control
    Play(PlayData),
    Pause,
    Resume,
    Stop,
    Forward,
    Rewind,

    // Indicate that forward or rewind operation has been completed
    SeekFinished,

    // File
    FileOpened(FileOpenedData),
    DirOpened(DirOpenedData),

    // Rendering
    RenderVideo(Arc<VideoFrame>),
    RenderAudio(AudioFrame),
    RenderSubtitle(Arc<SubtitleFrame>),

    // UI layout
    Resize((u32, u32)),
    SetPosition{x:WindowPos, y:WindowPos},

    // Volume control
    UpVolume,
    DownVolume,
}

pub struct PlayData {
    pub path: &'static str,
}

pub struct MediaSelectedData {
    pub path: PathBuf,
}

pub struct MediaItemDoubleClickedData {
    pub path: PathBuf,
}

pub struct FileOpenedData{
    pub path: &'static str,
}

pub struct DirOpenedData {
    pub paths: Vec<&'static str>,
}
