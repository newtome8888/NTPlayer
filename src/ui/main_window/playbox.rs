use std::sync::Arc;

use log::{debug, warn};
use rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_YUV420P as AVPIXELFORMAT_AV_PIX_FMT_YUV420P;
use sdl2::{pixels::PixelFormatEnum, rect::Rect, render::Texture, video::WindowPos};
// use tracing::{info, warn};

use crate::{
    entity::EventMessage, global::EVENT_CHANNEL, media::decoder::VideoFrame,
    util::error::{SuperError, safe_send},
};

use super::global::{CANVAS, TEXTURE_CREATOR};

pub struct PlayBox {
    texture: Texture<'static>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    frame: Option<Arc<VideoFrame>>,
}

impl PlayBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Result<Self, SuperError> {
        let texture = TEXTURE_CREATOR.with(|tc| unsafe {
            let tc_ptr = tc.get();
            let maybe_uninit = tc_ptr.as_mut().unwrap();
            let tc = maybe_uninit.assume_init_ref();

            tc.create_texture_streaming(PixelFormatEnum::IYUV, width, height)
        })?;

        Ok(Self {
            texture,
            x,
            y,
            width,
            height,
            frame: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), SuperError> {
        debug!("resize window");
        let texture = TEXTURE_CREATOR.with(|tc| unsafe {
            let tc_ptr = tc.get();
            let maybe_uninit = tc_ptr.as_mut().unwrap();
            let tc = maybe_uninit.assume_init_ref();

            tc.create_texture_streaming(PixelFormatEnum::IYUV, width, height)
        })?;

        self.width = width;
        self.height = height;
        self.texture = texture;

        let result = EVENT_CHANNEL.0.send(EventMessage::SetPosition {
            x: WindowPos::Centered,
            y: WindowPos::Centered,
        });

        safe_send(result);

        Ok(())
    }

    pub fn update_frame(&mut self, frame: Arc<VideoFrame>) {
        self.frame = Some(frame);
    }

    pub fn render(&mut self) -> Result<(), SuperError> {
        if self.frame.is_none() {
            return Ok(());
        }

        let frame = self.frame.as_ref().unwrap();
        match frame.format {
            AVPIXELFORMAT_AV_PIX_FMT_YUV420P => {
                let data = &frame.data;
                let ypitch = frame.width;
                let upitch = frame.width / 2;
                let vpitch = frame.width / 2;

                self.texture
                    .update_yuv(None, &data[0], ypitch, &data[1], upitch, &data[2], vpitch)?;
            }
            _ => {
                warn!("unknown pixel format: {}", frame.format);
                return Ok(());
            }
        }

        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();
            let canvas = maybe_uninit.assume_init_mut();

            canvas.copy(
                &self.texture,
                None,
                Rect::new(self.x, self.y, self.width, self.height),
            )
        })?;

        Ok(())
    }
}
