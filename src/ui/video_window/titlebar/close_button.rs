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
    entity::EventMessage,
    ui::components::{rectangle::button::Button, MouseMotionParam, MouseUpParam, TControl},
    util::error::{safe_send, SuperError},
    EVENT_CHANNEL,
};

const BACKGROUND_COLOR: Color = Color::RGB(220, 20, 60);
const FORE_COLOR: Color = Color::WHITE;

pub struct CloseButton {
    inner: Button,
    selected: bool,
}

impl CloseButton {
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
            Point::new(center_x - step, center_y - step),
            Point::new(center_x + step, center_y + step),
        )?;
        canvas.draw_line(
            Point::new(center_x - step, center_y + step),
            Point::new(center_x + step, center_y - step),
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

        safe_send(EVENT_CHANNEL.0.send(EventMessage::ExitVideoWindow));

        Ok(true)
    }
}

impl Deref for CloseButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for CloseButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
