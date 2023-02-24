mod play_button;

use std::{cell::RefCell, rc::Rc};

use sdl2::{
    image::LoadSurface,
    pixels::Color,
    render::{BlendMode, Canvas},
    surface::Surface,
    video::Window,
    VideoSubsystem,
};

use crate::{global::{APP_NAME, LOGO_PATH}, app::{MouseUpParameters, MouseMotionParameters}};
use crate::util::error::SuperError;

use self::play_button::PlayButton;

pub struct StartWindow {
    canvas: Rc<RefCell<Canvas<Window>>>,
    title_bar: TitleBar,
    play_button: PlayButton,
}

impl StartWindow {
    pub fn new(sys: &VideoSubsystem) -> Result<Self, SuperError> {
        let wind = Self::prepare_window(sys)?;
        let canvas = Self::prepare_canvas(wind)?;
        let canvas = Rc::new(RefCell::new(canvas));
        let play_button = PlayButton::default(canvas.clone())?
            .with_size(100, 100)
            .with_center();

        let mut inst = Self {
            title_bar: TitleBar,
            play_button,
            canvas: canvas,
        };

        inst.redrawl()?;

        Ok(inst)
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

    pub fn redrawl(&mut self) -> Result<(), SuperError> {
        // Render content
        self.play_button.render()?;

        // Display on screen
        let canvas = self.canvas.clone();
        canvas.borrow_mut().present();

        Ok(())
    }

    fn prepare_window(sys: &VideoSubsystem) -> Result<Window, SuperError> {
        let mut wind = sys
            .window("NT Player", 200, 200)
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
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.clear();
        canvas.present();

        Ok(canvas)
    }

    pub fn on_mouse_up(&mut self, params: MouseUpParameters) {
        self.play_button.on_mouse_up(params);
    }
    
    pub fn on_mouse_motion(&mut self, params: MouseMotionParameters){
        self.play_button.on_mouse_motion(params);
    }
}

struct TitleBar;
