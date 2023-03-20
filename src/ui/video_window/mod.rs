mod controlbar;
mod playbox;
mod titlebar;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use log::error;
use sdl2::{
    image::LoadSurface,
    pixels::Color,
    render::Canvas,
    surface::Surface,
    video::{FullscreenType, Window, WindowPos},
    VideoSubsystem,
};

use crate::media::decoder::VideoFrame;
use crate::util::error::SuperError;
use crate::{APP_NAME, INIT_HEIGHT, INIT_WIDTH, LOGO_PATH};

use self::controlbar::ControlBar;
use self::playbox::PlayBox;
use self::titlebar::TitleBar;

use super::{
    components::{
        rectangle::Rectangle, MouseDownParam, MouseMotionParam, MouseUpParam, MouseWheelParam,
        TControl,
    },
    NTWindow,
};

pub const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

pub struct VideoWindow {
    pub id: u32,
    inner: Rectangle,
    titlebar: TitleBar,
    controlbar: ControlBar,
    playbox: PlayBox,
}

impl VideoWindow {
    pub fn new(sys: &VideoSubsystem) -> Result<Self, SuperError> {
        let wind = Self::prepare_window(sys)?;
        let window_id = wind.id();
        let (x, y) = wind.position();
        let (width, height) = wind.size();

        let canvas = Self::prepare_canvas(wind)?;
        let canvas = Rc::new(RefCell::new(canvas));
        let play_box = PlayBox::new(0, 0, INIT_WIDTH, INIT_HEIGHT, canvas.clone())?;

        Ok(Self {
            titlebar: TitleBar::new(canvas.clone())?,
            controlbar: ControlBar::new(canvas.clone())?,
            playbox: play_box,
            id: window_id,
            inner: Rectangle::new(x, y, width, height, canvas.clone())?,
        })
    }

    pub fn show(&mut self) {
        self.canvas_mut().window_mut().show();
    }

    pub fn hide(&mut self) {
        self.canvas_mut().window_mut().hide();
    }

    pub fn update_video_frame(&mut self, frame: VideoFrame) {
        self.playbox.update_frame(frame);
    }

    fn prepare_window(sys: &VideoSubsystem) -> Result<Window, SuperError> {
        let mut wind = sys
            .window("NT Player", INIT_WIDTH, INIT_HEIGHT)
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
        canvas.set_draw_color(BACKGROUND_COLOR);

        Ok(canvas)
    }

    pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if params.window_id != self.id {
            return Ok(false);
        }

        self.controlbar.on_mouse_down(params)?;
        self.titlebar.on_mouse_down(params)?;

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if params.window_id != self.id {
            return Ok(false);
        }

        self.playbox.on_mouse_up(params)?;
        self.titlebar.on_mouse_up(params)?;
        self.controlbar.on_mouse_up(params)?;

        Ok(true)
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if params.window_id != self.id {
            return Ok(false);
        }

        self.titlebar.on_mouse_motion(params)?;
        self.controlbar.on_mouse_motion(params)?;
        Ok(true)
    }

    pub fn on_mouse_wheel(&mut self, params: &MouseWheelParam) -> Result<bool, SuperError> {
        if params.window_id != self.id {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        // Update inner size first, it's important for later computing
        self.inner.set_size(width, height);

        let func = || -> Result<(), SuperError> {
            let mut canvas = self.canvas_mut();
            let wind = canvas.window_mut();

            wind.set_size(width, height)?;
            wind.set_position(WindowPos::Centered, WindowPos::Centered);

            Ok(())
        };
        match func() {
            Ok(_) => {
                self.on_resized(width, height);
            }
            Err(err) => error!("set window size failed: {:?}", err),
        }
    }

    pub fn on_resized(&mut self, width: u32, height: u32) {
        self.inner.set_size(width, height);

        // Adjust playbox size
        self.playbox.set_size(width, height);
        // Adjuist titlebar size
        self.titlebar.set_size(width, None);
        // Adjust controlbar size
        self.controlbar.set_size(width, None);

        // Adjust controlbar position        
        let y = height - self.controlbar.size().1 - self.controlbar.margin().3;
        self.controlbar.set_position(None, y as i32);
    }

    pub fn set_position<X, Y>(&mut self, x: X, y: Y)
    where
        X: Into<Option<i32>>,
        Y: Into<Option<i32>>,
    {
        println!("set window position");
        let x: Option<i32> = x.into();
        let y: Option<i32> = y.into();

        self.inner.set_position(x, y);

        // Set position of sdl window
        {
            let mut canvas = self.canvas_mut();
            let wind = canvas.window_mut();
            let (x, y) = self.inner.position();

            wind.set_position(WindowPos::Positioned(x), WindowPos::Positioned(y));
        }
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        self.canvas_mut().set_draw_color(Color::BLACK);
        self.canvas_mut().clear();

        // Render content
        self.playbox.render()?;
        self.titlebar.render()?;
        self.controlbar.render()?;

        // Display on screen
        self.canvas_mut().present();

        Ok(true)
    }

    pub fn set_fullscreen(&mut self, fs_type: FullscreenType) {
        if let Err(err) = self.canvas_mut().window_mut().set_fullscreen(fs_type) {
            error!("Failed to set fullscreen state, error: {:?}", err);
        }
    }
}

impl NTWindow for VideoWindow {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Deref for VideoWindow {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VideoWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
