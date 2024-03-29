use std::path::PathBuf;

use crate::media::decoder::{AudioFrame, SubtitleFrame, VideoFrame};

/// Message types for application related events
pub enum EventMessage {
    /// Exit application
    Quit,
    /// Exit video window and return back to the start window
    ExitVideoWindow,

    /// Show error dialog
    ShowError(String),

    // For media state control
    Play(PathBuf),
    Pause,
    Resume,
    Stop,
    Forward,
    Rewind,
    SeekTo(i64),

    // Indicate that forward or rewind operation has been completed
    SeekFinished,

    // File
    FileOpened(PathBuf),
    DirOpened(Vec<PathBuf>),

    // Rendering
    RenderVideo(VideoFrame),
    RenderAudio(AudioFrame),
    RenderSubtitle(SubtitleFrame),

    // UI layout
    Resize((u32, u32)),
    SetPosition (Option<i32>, Option<i32>),
    ToggleFullScreen,

    // Volume control
    UpVolume,
    DownVolume,
}

pub struct MediaSelectedData {
    pub path: PathBuf,
}

pub struct MediaItemDoubleClickedData {
    pub path: PathBuf,
}
