use std::{
    cell::{RefCell},
    rc::Rc,
    sync::Arc,
};

use log::{warn};
use rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_YUV420P as AVPIXELFORMAT_AV_PIX_FMT_YUV420P;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::Canvas,
    video::{Window, WindowPos},
};

use crate::{
    entity::EventMessage,
    global::EVENT_CHANNEL,
    media::decoder::VideoFrame,
    util::error::{safe_send, SuperError},
};

use super::RefWindow;

pub struct PlayBox {
    canvas: Rc<RefCell<Canvas<Window>>>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    frame: Option<Arc<VideoFrame>>,
}

impl PlayBox {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: RefWindow,
    ) -> Result<Self, SuperError> {
        Ok(Self {
            canvas,
            x,
            y,
            width,
            height,
            frame: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), SuperError> {
        self.width = width;
        self.height = height;

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

        let canvas = self.canvas.clone();
        let texture_creator = canvas.borrow_mut().texture_creator();

        let frame = self.frame.as_ref().unwrap();
        let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::IYUV,
            frame.width as u32,
            frame.height as u32,
        )?;
        match frame.format {
            AVPIXELFORMAT_AV_PIX_FMT_YUV420P => {
                let data = &frame.data;
                let ypitch = frame.width;
                let upitch = frame.width / 2;
                let vpitch = frame.width / 2;

                texture.update_yuv(None, &data[0], ypitch, &data[1], upitch, &data[2], vpitch)?;
            }
            _ => {
                warn!("unknown pixel format: {}", frame.format);
                return Ok(());
            }
        }

        self.canvas.clone().borrow_mut().copy(
            &texture,
            None,
            Rect::new(self.x, self.y, self.width, self.height),
        )?;
        Ok(())
    }
}
