use std::{path::PathBuf, sync::Arc};

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

    // File
    FileOpened(FileOpenedData),
    DirOpened(DirOpenedData),

    // Rendering
    RenderVideo(Arc<VideoFrame>),
    RenderAudio(Arc<AudioFrame>),
    RenderSubtitle(Arc<SubtitleFrame>),
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

pub enum PlayerState {
    NONE,
    PLAYING,
    PAUSING,
    STOPPED,
}
