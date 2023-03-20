use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    ui::components::{rectangle::button::Button, MouseMotionParam, MouseUpParam, TControl},
    util::error::SuperError,
};

const BACKGROUND_COLOR: Color = Color::RGB(51, 51, 255);
const FORE_COLOR: Color = Color::WHITE;

pub struct MinimizeButton {
    inner: Button,
    selected: bool,
}

impl MinimizeButton {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = Button::new(x, y, width, height, canvas)?;

        Ok(Self {
            inner,
            selected: false,
        })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let mut canvas = self.canvas_mut();
        let (x, y) = self.position();
        let (width, height) = self.size();
        let button_rect = Rect::new(x, y, width, height);
        // draw background
        if self.selected {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.fill_rect(button_rect)?;
        }

        // draw X shape, the size of X shape is const value 20X20
        let (center_x, center_y) = self.center();
        let step = 5;
        canvas.set_draw_color(FORE_COLOR);
        canvas.draw_line(
            Point::new(center_x - step, center_y),
            Point::new(center_x + step, center_y),
        )?;

        Ok(true)
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        self.selected = self.inner.on_mouse_motion(params)?;

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        self.canvas_mut().window_mut().minimize();

        Ok(true)
    }
}

impl Deref for MinimizeButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for MinimizeButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
