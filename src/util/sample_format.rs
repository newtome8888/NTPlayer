use std::{slice, sync::atomic::Ordering};

use rsmpeg::avutil::AVFrame;

use crate::{media::decoder::AudioFrame, AUDIO_SUMMARY, VOLUME_BENCHMARK, VOLUME};

/// Parse ffmpeg audio frame to AudioFrame
pub fn parse_audio_frame(frame: &mut AVFrame) -> AudioFrame {
    let r = AUDIO_SUMMARY.read().unwrap();
    let summary = r.as_ref().unwrap();
    let pts_millis = 1000 * frame.pts * summary.timebase_num as i64 / summary.timebase_den as i64;

    let left_slice =
        unsafe { slice::from_raw_parts(frame.data[0] as *const f32, frame.nb_samples as usize) };
    let right_slice =
        unsafe { slice::from_raw_parts(frame.data[1] as *const f32, frame.nb_samples as usize) };

    // Convert planar data to mono data
    // Set volume to current frame
    let volume = VOLUME.load(Ordering::Acquire) as f32 / VOLUME_BENCHMARK;
    let mut data = vec![];
    for (left, right) in left_slice.iter().zip(right_slice.iter()) {
        data.push((*left) * volume);
        data.push((*right) * volume);
    }

    let audio_frame = AudioFrame {
        format: frame.format,
        data,
        pts: frame.pts,
        pts_millis,
        sample_rate: frame.sample_rate,
        channels: frame.channels as u8,
        channel_layout: frame.channel_layout as u8,
    };

    audio_frame
}
