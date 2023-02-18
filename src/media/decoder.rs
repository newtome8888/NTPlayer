use std::{
    error::Error,
    ffi::CString,
    ops::{Deref, DerefMut},
    slice,
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crossbeam::queue::ArrayQueue;
use log::{debug, error, info, warn};

use rsmpeg::{
    avcodec::{AVCodec, AVCodecContext, AVPacket},
    avformat::AVFormatContextInput,
    avutil::AVFrame,
    ffi::{
        AVMediaType_AVMEDIA_TYPE_ATTACHMENT, AVMediaType_AVMEDIA_TYPE_AUDIO,
        AVMediaType_AVMEDIA_TYPE_DATA, AVMediaType_AVMEDIA_TYPE_NB,
        AVMediaType_AVMEDIA_TYPE_SUBTITLE, AVMediaType_AVMEDIA_TYPE_VIDEO,
        AVPixelFormat_AV_PIX_FMT_YUV420P,
    },
};

use crate::global_variables::{AUDIO_BUFFER, SUBTITLE_BUFFER, VIDEO_BUFFER};
const BUFFER_FULL_SLEEP_DURATION: Duration = Duration::from_millis(200);
pub type MediaSummary = (
    Option<AudioSummary>,
    Option<VideoSummary>,
    Option<SubtitleSummary>,
);

pub struct MediaDecoder {
    stop_flag: Arc<AtomicBool>,
    seek_to: Arc<AtomicI64>,
    pub media_summary: MediaSummary,
}

impl MediaDecoder {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let seek_to = Arc::new(AtomicI64::new(-1));

        let ctx = MediaDecoder::get_media_context(&path)?;
        let (streams, media_summary) = Self::get_streams(&ctx);

        Self::start_task(ctx, streams, &stop_flag, &seek_to);

        Ok(Self {
            stop_flag,
            seek_to,
            media_summary,
        })
    }

    /// Seek to the specified position
    /// `position` is the position to seek to, unit microseconds
    pub fn seek_to(&mut self, position: i64) {
        self.seek_to.store(position, Ordering::SeqCst);
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
    }

    fn start_task(
        ctx: AVFormatContextInput,
        streams: MediaStreams,
        stop_flag: &Arc<AtomicBool>,
        seek_to: &Arc<AtomicI64>,
    ) {
        let mut ctx = ctx;
        let stop_flag = stop_flag.clone();
        let seek_to = seek_to.clone();

        let audio_stream = streams.audio_stream;
        let mut video_stream = streams.video_stream;
        let subtitle_stream = streams.subtitle_stream;
        let data_stream = streams.data_stream;
        let attachment_stream = streams.attachment_stream;
        let nb_stream = streams.nb_stream;
        thread::spawn({
            move || {
                loop {
                    if stop_flag.load(std::sync::atomic::Ordering::SeqCst) {
                        debug!("stop flag setted, stop thread now");
                        break;
                    }

                    let position = seek_to.load(Ordering::SeqCst);
                    if position >= 0 {
                        debug!("seek to position {}", position);
                        // If seek_to is not negative,
                        // clear the buffers
                        while !AUDIO_BUFFER.is_empty() {
                            AUDIO_BUFFER.pop();
                        }

                        while !VIDEO_BUFFER.is_empty() {
                            VIDEO_BUFFER.pop();
                        }

                        while !SUBTITLE_BUFFER.is_empty() {
                            SUBTITLE_BUFFER.pop();
                        }

                        // Finally seek to the new position
                        // todo!

                        seek_to.store(-1, Ordering::SeqCst);
                    }

                    if AUDIO_BUFFER.is_full() || VIDEO_BUFFER.is_full() || SUBTITLE_BUFFER.is_full()
                    {
                        thread::sleep(BUFFER_FULL_SLEEP_DURATION);
                        continue;
                    }

                    let result = ctx.read_packet().unwrap();
                    match result {
                        Some(packet) => {
                            // Only process the data in correct stream, ignore others
                            let stream_index = Some(packet.stream_index);
                            if stream_index == audio_stream.index {
                                // Self::decode_audio(audio_decoder, packet, &vb);
                            } else if stream_index == video_stream.index {
                                video_stream.decoder_ctx = video_stream
                                    .decoder_ctx
                                    .and_then(|dctx| {
                                        let dctx = Self::decode_video(dctx, &packet);
                                        Some(dctx)
                                    })
                                    .or_else(|| {
                                        warn!("Video stream founded but no decoder!");
                                        None
                                    });
                            } else if stream_index == subtitle_stream.index {
                                //todo!
                            } else if stream_index == data_stream.index {
                                info!("data stream packet readed");
                            } else if stream_index == nb_stream.index {
                                info!("nb stream packet readed");
                            } else if stream_index == attachment_stream.index {
                                info!("attachment stream packet readed");
                            } else {
                                debug!("unknown type of packet");
                            }
                        }
                        None => {
                            debug!("no more packets, stop decoding");
                            break;
                        }
                    }
                }
            }
        });

        debug!("thread finished");
    }

    fn decode_video(dctx: AVCodecContext, packet: &AVPacket) -> AVCodecContext {
        let mut dctx = dctx;
        if let Err(err) = dctx.send_packet(Some(packet)) {
            debug!("send packet to context error: {}", err);
            return dctx;
        }

        match dctx.receive_frame() {
            Ok(frame) => {
                let mut vf = Self::parse_video_frame(&frame);
                // Push frame to buffer until succeeded
                while let Err(f) = VIDEO_BUFFER.push(vf) {
                    vf = f;
                    thread::sleep(BUFFER_FULL_SLEEP_DURATION);
                }
            }
            Err(err) => {
                debug!("{}", err);
            }
        }

        dctx
    }

    /// Notice! DemuxerWithStreamInfo do not support multiple threads, so you have to create
    /// a new object for every thread which `DemuxerWithStreamInfo` will be used
    pub fn get_media_context(path: &str) -> Result<AVFormatContextInput, Box<dyn Error>> {
        let path = CString::new(path).unwrap();
        let ctx = AVFormatContextInput::open(&path)?;

        Ok(ctx)
    }

    fn get_streams(ctx: &AVFormatContextInput) -> (MediaStreams, MediaSummary) {
        let streams = ctx.streams();

        let mut audio_stream = StreamInfo::default();
        let mut video_stream = StreamInfo::default();
        let mut subtitle_stream = StreamInfo::default();
        let mut data_stream = StreamInfo::default();
        let mut attachment_stream = StreamInfo::default();
        let mut nb_stream = StreamInfo::default();
        let mut unknown_streams = Vec::<StreamInfo>::new();

        let mut audio_summary = None;
        let mut video_summary = None;
        let mut subtitle_summary = None;

        for stream in streams {
            if stream.nb_frames <= 0 {
                continue;
            }

            let codecpar = stream.codecpar();
            let codec_type = stream.codecpar().codec_type;

            let mut decoder_name = String::default();
            let decoder_ctx = AVCodec::find_decoder(codecpar.codec_id).and_then(|d| {
                decoder_name = d.name().to_str().unwrap_or("unknown").to_string();
                let mut decoder_ctx = AVCodecContext::new(&d);

                if let Err(err) = decoder_ctx.apply_codecpar(&codecpar) {
                    error!("{}", err);
                }

                if let Err(err) = decoder_ctx.open(None) {
                    error!("{}", err);
                }

                Some(decoder_ctx)
            });
            let stream_info = StreamInfo {
                decoder_ctx,
                index: Some(stream.index),
            };
            match codec_type {
                AVMediaType_AVMEDIA_TYPE_AUDIO => {
                    audio_stream = stream_info;
                    audio_summary = Some(AudioSummary);
                }
                AVMediaType_AVMEDIA_TYPE_VIDEO => {
                    video_stream = stream_info;
                    video_summary = Some(VideoSummary {
                        decoder_name: decoder_name.to_string(),
                        duration: stream.duration as u64,
                        frames: stream.nb_frames as u64,
                        time_base_num: stream.time_base.num as u64,
                        time_base_den: stream.time_base.den as u64,
                        width: codecpar.width as u32,
                        height: codecpar.height as u32,
                    });
                }
                AVMediaType_AVMEDIA_TYPE_SUBTITLE => {
                    subtitle_stream = stream_info;
                    subtitle_summary = Some(SubtitleSummary);
                }
                AVMediaType_AVMEDIA_TYPE_ATTACHMENT => {
                    attachment_stream = stream_info;
                }
                AVMediaType_AVMEDIA_TYPE_DATA => {
                    data_stream = stream_info;
                }
                AVMediaType_AVMEDIA_TYPE_NB => {
                    nb_stream = stream_info;
                }
                _ => {
                    unknown_streams.push(stream_info);
                }
            }
        }

        (
            MediaStreams {
                audio_stream,
                video_stream,
                subtitle_stream,
                attachment_stream,
                nb_stream,
                data_stream,
                unknown_streams,
            },
            (audio_summary, video_summary, subtitle_summary),
        )
    }

    fn parse_video_frame(frame: &AVFrame) -> VideoFrame {
        let width = frame.width as usize;
        let height = frame.height as usize;

        match frame.format {
            AVPixelFormat_AV_PIX_FMT_YUV420P => {
                let y_size = width * height;
                let u_size = y_size / 4; // width/2 * height/2
                let v_size = y_size / 4; // width/2 * height/2

                let y_ptr = frame.data[0];
                let y = unsafe { slice::from_raw_parts(y_ptr, y_size) };

                let u_ptr = frame.data[1];
                let u = unsafe { slice::from_raw_parts(u_ptr, u_size) };

                let v_ptr = frame.data[2];
                let v = unsafe { slice::from_raw_parts(v_ptr, v_size) };

                VideoFrame {
                    format: frame.format,
                    data: [y, u, v, &[], &[], &[], &[], &[]],
                    width,
                    height,
                    pts: frame.pts,
                }
            }
            _ => {
                warn!(
                    "Un implemented pixel format: {}. It needs some time to finish the work.",
                    frame.format
                );
                VideoFrame {
                    format: frame.format,
                    data: [&[], &[], &[], &[], &[], &[], &[], &[]],
                    width,
                    height,
                    pts: frame.pts,
                }
            }
        }
    }

    fn parse_audio_frame(frame: &AVFrame) -> AudioFrame {
        todo!();
    }

    fn parse_subtitle_frame(frame: &AVFrame) -> SubtitleFrame {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoSummary {
    /// The name of decoder if any
    pub decoder_name: String,
    /// The duration of whole media, unit: second
    pub duration: u64,
    /// Number of frames in media
    pub frames: u64,
    /// Number of timebase
    pub time_base_num: u64,
    /// Denominator of timebase
    pub time_base_den: u64,
    /// Width of video
    pub width: u32,
    /// Height of video
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioSummary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleSummary;

struct StreamInfo {
    decoder_ctx: Option<AVCodecContext>,
    index: Option<i32>,
}

impl Default for StreamInfo {
    fn default() -> Self {
        Self {
            decoder_ctx: Default::default(),
            index: Default::default(),
        }
    }
}

struct MediaStreams {
    audio_stream: StreamInfo,
    video_stream: StreamInfo,
    subtitle_stream: StreamInfo,
    attachment_stream: StreamInfo,
    nb_stream: StreamInfo,
    data_stream: StreamInfo,
    unknown_streams: Vec<StreamInfo>,
}

trait MediaBuffer {
    type Item;
    fn pop(&self) -> Option<Self::Item>;
    fn push(&self, item: Self::Item) -> Result<(), Self::Item>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

pub struct AudioFrame {
    data: [&'static [u8]; 8],
}

pub struct AudioBuffer {
    inner: ArrayQueue<AudioFrame>,
}

impl AudioBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            inner: ArrayQueue::<AudioFrame>::new(size),
        }
    }
}

impl Deref for AudioBuffer {
    type Target = ArrayQueue<AudioFrame>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct VideoFrame {
    pub format: i32,
    pub data: [&'static [u8]; 8],
    pub width: usize,
    pub height: usize,
    pub pts: i64,
}

pub struct VideoBuffer {
    inner: ArrayQueue<VideoFrame>,
}

impl VideoBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            inner: ArrayQueue::<VideoFrame>::new(size),
        }
    }
}

impl Deref for VideoBuffer {
    type Target = ArrayQueue<VideoFrame>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VideoBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct SubtitleFrame {
    data: [&'static [u8]; 8],
}

pub struct SubtitleBuffer {
    inner: ArrayQueue<SubtitleFrame>,
}

impl SubtitleBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            inner: ArrayQueue::<SubtitleFrame>::new(size),
        }
    }
}

impl Deref for SubtitleBuffer {
    type Target = ArrayQueue<SubtitleFrame>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SubtitleBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
