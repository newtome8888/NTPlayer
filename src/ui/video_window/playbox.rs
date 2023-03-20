use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    time::Instant,
};

use log::warn;
use rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_YUV420P as AVPIXELFORMAT_AV_PIX_FMT_YUV420P;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::Canvas,
    video::{Window, WindowPos},
};

use crate::{
    entity::EventMessage,
    media::decoder::VideoFrame,
    ui::components::{rectangle::Rectangle, MouseUpParam, TControl},
    util::error::{safe_send, SuperError},
    EVENT_CHANNEL,
};

const DOUBLE_CLICK_INTERVAL: u128 = 200;

pub struct PlayBox {
    inner: Rectangle,
    frame: Option<VideoFrame>,
    preclick: Instant,
}

impl PlayBox {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        Ok(Self {
            inner: Rectangle::new(x, y, width, height, canvas.clone())?,
            frame: None,
            preclick: Instant::now(),
        })
    }

    pub fn update_frame(&mut self, frame: VideoFrame) {
        self.frame = Some(frame);
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let mut canvas = self.canvas_mut();
        let texture_creator = canvas.texture_creator();

        if let Some(frame) = self.frame.as_ref() {
            let frame_width = frame.width as u32;
            let frame_height = frame.height as u32;

            let mut texture = texture_creator.create_texture_streaming(
                PixelFormatEnum::IYUV,
                frame_width,
                frame_height,
            )?;
            match frame.format {
                AVPIXELFORMAT_AV_PIX_FMT_YUV420P => {
                    let data = &frame.data;
                    let ypitch = frame.width;
                    let upitch = ypitch / 2;
                    let vpitch = ypitch / 2;

                    texture
                        .update_yuv(None, &data[0], ypitch, &data[1], upitch, &data[2], vpitch)?;
                }
                _ => {
                    warn!("unknown pixel format: {}", frame.format);
                    return Ok(false);
                }
            }

            let (width, height) = self.compute_render_size(frame_width, frame_height)?;
            let (x, y) = self.compute_render_position(width, height);
            canvas.copy(&texture, None, Rect::new(x, y, width, height))?;
        }

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        // Toggle fullscreen state while double clicked
        if self.preclick.elapsed().as_millis() < DOUBLE_CLICK_INTERVAL {
            EVENT_CHANNEL.0.send(EventMessage::ToggleFullScreen)?;
        }

        self.preclick = Instant::now();

        Ok(true)
    }

    fn compute_render_size(
        &self,
        frame_width: u32,
        frame_height: u32,
    ) -> Result<(u32, u32), SuperError> {
        let ratio = frame_width / frame_height;

        let width = self.size().0;
        let height = width / ratio;

        Ok((width, height))
    }

    fn compute_render_position(&self, width: u32, height: u32) -> (i32, i32) {
        let (center_x, center_y) = self.center();
        let render_x = center_x - width as i32 / 2;
        let render_y = center_y - height as i32 / 2;

        (render_x, render_y)
    }
}

impl Deref for PlayBox {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PlayBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
