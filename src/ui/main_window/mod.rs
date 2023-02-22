mod global;
mod playbar;
mod playbox;
mod progressbar;
mod titlebar;

use std::borrow::{Borrow, BorrowMut};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Arc;

// use log::warn;
use rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_YUV420P;
use sdl2::image::LoadSurface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext, WindowPos};
use sdl2::VideoSubsystem;

use crate::global::{APP_NAME, DEFAULT_BACKGROUND_COLOR, INIT_HEIGHT, INIT_WIDTH, LOGO_PATH};
use crate::media::decoder::VideoFrame;
use crate::util::error::SuperError;

use self::global::{CANVAS, TEXTURE_CREATOR};
use self::playbar::PlayBar;
use self::playbox::PlayBox;
use self::progressbar::ProgressBar;
use self::titlebar::TitleBar;

pub struct MainWindow {
    titlebar: TitleBar,
    playbar: PlayBar,
    progressbar: ProgressBar,
    playbox: PlayBox,
}

impl MainWindow {
    pub fn new(sys: &VideoSubsystem) -> Result<Self, SuperError> {
        let wind = Self::prepare_window(sys)?;
        let canvas = Self::prepare_canvas(wind)?;
        let texture_creator = canvas.texture_creator();

        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();

            maybe_uninit.write(canvas);
        });

        TEXTURE_CREATOR.with(|t| unsafe {
            let tc_ptr = t.get();
            let maybe_uninit = tc_ptr.as_mut().unwrap();

            maybe_uninit.write(texture_creator);
        });

        let play_box = PlayBox::new(0, 0, INIT_WIDTH, INIT_HEIGHT)?;

        Ok(Self {
            titlebar: TitleBar,
            playbar: PlayBar,
            progressbar: ProgressBar,
            playbox: play_box,
        })
    }

    pub fn set_logo(&mut self, path: &str) -> Result<(), SuperError> {
        let logo = Surface::from_file(path)?;

        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();
            let canvas = maybe_uninit.assume_init_mut();
            let wind = canvas.window_mut();

            wind.set_icon(logo);
        });

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), SuperError> {
        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();
            let canvas = maybe_uninit.assume_init_mut();
            let wind = canvas.window_mut();

            wind.set_size(width, height)
        })?;

        self.playbox.resize(width, height)?;

        Ok(())
    }

    pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {
        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();
            let canvas = maybe_uninit.assume_init_mut();
            let wind = canvas.window_mut();

            wind.set_position(x, y)
        });
    }

    pub fn update_video_frame(&mut self, frame: Arc<VideoFrame>) {
        self.playbox.update_frame(frame);
    }

    pub fn redrawl(&mut self) -> Result<(), SuperError> {
        // Render content
        self.playbox.render()?;

        // Display on screen
        CANVAS.with(|c| unsafe {
            let canvas_ptr = c.get();
            let maybe_uninit = canvas_ptr.as_mut().unwrap();
            let canvas = maybe_uninit.assume_init_mut();

            canvas.present();
        });

        Ok(())
    }

    fn prepare_window(sys: &VideoSubsystem) -> Result<Window, SuperError> {
        let mut wind = sys
            .window("rust-sdl2 demo: Video", INIT_WIDTH, INIT_HEIGHT)
            .borderless()
            .allow_highdpi()
            .position_centered()
            .resizable()
            .opengl()
            .build()?;

        wind.set_title(APP_NAME)?;

        let logo = Surface::from_file(LOGO_PATH)?;
        wind.set_icon(logo);

        Ok(wind)
    }

    fn prepare_canvas(wind: Window) -> Result<Canvas<Window>, SuperError> {
        let mut canvas = wind.into_canvas().build()?;
        canvas.set_draw_color(DEFAULT_BACKGROUND_COLOR);

        Ok(canvas)
    }
}
