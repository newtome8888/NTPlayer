use std::borrow::{Borrow, BorrowMut};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Arc;

use log::warn;
use rsmpeg::ffi::AVPixelFormat_AV_PIX_FMT_YUV420P;
use sdl2::image::LoadSurface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;

use crate::entity::PlayerState;
use crate::global_variables::{
    APP_NAME, DEFAULT_BACKGROUND_COLOR, INIT_HEIGHT, INIT_WIDTH, LOGO_PATH,
};
use crate::media::decoder::VideoFrame;
use crate::util::error::SuperError;

thread_local! {
    static CANVAS: UnsafeCell<MaybeUninit<Canvas<Window>>> = UnsafeCell::new(MaybeUninit::zeroed());
    static TEXTURE_CREATOR: UnsafeCell<MaybeUninit<TextureCreator<WindowContext>>> = UnsafeCell::new(MaybeUninit::zeroed());
}

pub struct MainWindow {
    title_bar: TitleBar,
    play_bar: PlayBar,
    progress_bar: ProgressBar,
    play_box: PlayBox,
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
            title_bar: TitleBar,
            play_bar: PlayBar,
            progress_bar: ProgressBar,
            play_box,
        })
    }

    pub fn set_state(&mut self, state: PlayerState) {
        match state {
            PlayerState::NONE => {
                todo!()
            }
            PlayerState::PLAYING => todo!(),
            PlayerState::PAUSING => todo!(),
            PlayerState::STOPPED => todo!(),
        }
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

    pub fn update_video_frame(&mut self, frame: Arc<VideoFrame>) {
        self.play_box.update_frame(frame);
    }

    pub fn redrawl(&mut self) -> Result<(), SuperError> {
        // Render content
        self.play_box.render()?;

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

struct TitleBar;
struct ProgressBar;
struct PlayBar;
struct PlayBox {
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

            tc.create_texture_target(PixelFormatEnum::IYUV, width, height)
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

    pub fn update_frame(&mut self, frame: Arc<VideoFrame>) {
        self.frame = Some(frame);
    }

    pub fn render(&mut self) -> Result<(), SuperError> {
        if self.frame.is_none() {
            return Ok(());
        }

        let frame = self.frame.as_ref().unwrap();
        match frame.format {
            AVPixelFormat_AV_PIX_FMT_YUV420P => {
                let data = frame.data;
                let ypitch = frame.width * frame.height;
                let upitch = ypitch / 2;
                let vpitch = upitch;

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
