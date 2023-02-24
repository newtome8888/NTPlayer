mod playbar;
mod playbox;
mod progressbar;
mod titlebar;

use std::{cell::RefCell, rc::Rc, sync::Arc};

use sdl2::{
    image::LoadSurface,
    render::Canvas,
    surface::Surface,
    video::{Window, WindowPos},
    VideoSubsystem,
};

use crate::global::{APP_NAME, INIT_HEIGHT, INIT_WIDTH, LOGO_PATH, VIDEO_BACKGROUND_COLOR};
use crate::media::decoder::VideoFrame;
use crate::util::error::SuperError;

use self::playbar::PlayBar;
use self::playbox::PlayBox;
use self::progressbar::ProgressBar;
use self::titlebar::TitleBar;

pub type RefWindow = Rc<RefCell<Canvas<Window>>>;
pub struct VideoWindow {
    canvas: RefWindow,
    titlebar: TitleBar,
    playbar: PlayBar,
    progressbar: ProgressBar,
    playbox: PlayBox,
}

impl VideoWindow {
    pub fn new(sys: &VideoSubsystem) -> Result<Self, SuperError> {
        let wind = Self::prepare_window(sys)?;
        let canvas = Self::prepare_canvas(wind)?;
        let canvas = Rc::new(RefCell::new(canvas));
        let play_box = PlayBox::new(0, 0, INIT_WIDTH, INIT_HEIGHT, canvas.clone())?;

        Ok(Self {
            titlebar: TitleBar,
            playbar: PlayBar,
            progressbar: ProgressBar,
            playbox: play_box,
            canvas,
        })
    }

    pub fn show(&mut self) {
        let canvas = self.canvas.clone();
        canvas.borrow_mut().window_mut().show();
    }

    pub fn hide(&mut self) {
        let canvas = self.canvas.clone();
        canvas.borrow_mut().window_mut().hide();
    }

    pub fn set_logo(&mut self, path: &str) -> Result<(), SuperError> {
        let logo = Surface::from_file(path)?;
        let canvas = self.canvas.clone();
        canvas.borrow_mut().window_mut().set_icon(logo);

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), SuperError> {
        let canvas = self.canvas.clone();
        canvas.borrow_mut().window_mut().set_size(width, height)?;
        self.playbox.resize(width, height)?;

        Ok(())
    }

    pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {
        let canvas = self.canvas.clone();
        canvas.borrow_mut().window_mut().set_position(x, y);
    }

    pub fn update_video_frame(&mut self, frame: Arc<VideoFrame>) {
        self.playbox.update_frame(frame);
    }

    pub fn redrawl(&mut self) -> Result<(), SuperError> {
        // Render content
        self.playbox.render()?;
        // Display on screen
        let canvas = self.canvas.clone();
        canvas.borrow_mut().present();

        Ok(())
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
        canvas.set_draw_color(VIDEO_BACKGROUND_COLOR);

        Ok(canvas)
    }
}
