use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window,
};

use crate::{
    ui::components::{rectangle::button::Button, MouseMotionParam, MouseUpParam, TControl},
    util::error::SuperError, EVENT_CHANNEL, entity::EventMessage,
};

const ON_COLOR: Color = Color::WHITE;
const OFF_COLOR: Color = Color::GRAY;

pub struct RewindButton {
    inner: Button,
}

impl RewindButton {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = Button::new(x, y, width, height, canvas)?;

        Ok(Self { inner })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let canvas = self.canvas_mut();
        let (center_x, center_y) = self.center16();
        let (control_width, control_height) = self.size16();
        let (trg_width, trg_height) = (control_width * 2 / 5, control_height * 2 / 3);
        let color = if self.is_cursorin() {
            ON_COLOR
        } else {
            OFF_COLOR
        };

        let (trg1_x1, trg1_y1) = (center_x - trg_width as i16, center_y);
        let (trg1_x2, trg1_y2) = (center_x, center_y - trg_height as i16 / 2);
        let (trg1_x3, trg1_y3) = (center_x, center_y + trg_height as i16 / 2);

        let (trg2_x1, trg2_y1) = (center_x, center_y);
        let (trg2_x2, trg2_y2) = (center_x + trg_width as i16, center_y - trg_height as i16 / 2);
        let (trg2_x3, trg2_y3) = (trg2_x2, center_y + trg_height as i16 / 2);

        canvas.aa_trigon(trg1_x1, trg1_y1, trg1_x2, trg1_y2, trg1_x3, trg1_y3, color)?;
        canvas.aa_trigon(trg2_x1, trg2_y1, trg2_x2, trg2_y2, trg2_x3, trg2_y3, color)?;

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        EVENT_CHANNEL.0.send(EventMessage::Rewind)?;

        Ok(true)
    }
}

impl Deref for RewindButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RewindButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
