use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    path::PathBuf,
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::{
    entity::EventMessage,
    ui::components::{rectangle::button::Button, MouseMotionParam, MouseUpParam, TControl},
    util::error::SuperError,
    EVENT_CHANNEL,
};

const ON_COLOR: Color = Color::WHITE;
const OFF_COLOR: Color = Color::GRAY;

pub struct NextButton {
    inner: Button,
}

impl NextButton {
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
        let color = if self.is_cursorin() {
            ON_COLOR
        } else {
            OFF_COLOR
        };

        let line_width = 2;
        let trg_width = control_width / 3;
        let content_width = line_width + trg_width;
        let content_height = control_height * 2 / 3;

        // draw trigon at left side
        let (tx1, ty1) = (
            center_x - content_width as i16 / 2,
            center_y - content_height as i16 / 2,
        );
        let (tx2, ty2) = (tx1, center_y + content_height as i16 / 2);
        let (tx3, ty3) = (tx1 + trg_width as i16, center_y);

        canvas.aa_trigon(tx1, ty1, tx2, ty2, tx3, ty3, color)?;

        // draw vertical line at right side
        let (line_x1, line_y1) = (tx3, center_y - content_height as i16 / 2);
        let (x2, y2) = (
            line_x1 + line_width as i16,
            center_y + content_height as i16 / 2,
        );

        canvas.rectangle(line_x1, line_y1, x2, y2, color)?;

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        // EVENT_CHANNEL
        //     .0
        //     .send(EventMessage::Play(PathBuf::from("next path")))?;
        println!("play next");

        Ok(true)
    }
}

impl Deref for NextButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NextButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
