use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window,
};

use crate::{
    entity::EventMessage,
    ui::components::{rectangle::button::Button, MouseUpParam, TControl},
    util::error::SuperError,
    EVENT_CHANNEL,
};

const ON_COLOR: Color = Color::WHITE;
const OFF_COLOR: Color = Color::GRAY;

pub struct PlayButton {
    inner: Button,
    paused: bool,
}

impl PlayButton {
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
            paused: false,
        })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let canvas = self.canvas_mut();
        let (center_x, center_y) = self.center16();
        let (control_width, control_height) = self.size16();
        let (width, height) = (control_width * 2 / 3, control_height * 2 / 3);
        let color = if self.is_cursorin() {
            ON_COLOR
        } else {
            OFF_COLOR
        };

        if self.paused {
            // Current state is paused, draw play button
            let x1 = center_x - width as i16 / 2;
            let x2 = center_x + width as i16 / 2;
            let y1 = center_y - height as i16 / 2;
            let y2 = center_y + height as i16 / 2;

            canvas.trigon(x1, y1, x1, y2, x2, center_y, color)?;
        } else {
            // Current state is playing,show pause button
            let margin: u16 = 5;
            let rect_width = (width - margin) / 2;

            let (rect1_x1, rect1_y1) = (center_x - width as i16 / 2, center_y - height as i16 / 2);
            let (rect1_x2, rect1_y2) = (rect1_x1 + rect_width as i16, rect1_y1 + height as i16);
            
            let (rect2_x1, rect2_y1) = (rect1_x2 + margin as i16, rect1_y1);
            let (rect2_x2, rect2_y2) = (rect2_x1 + rect_width as i16, rect1_y2);

            canvas.rectangle(rect1_x1, rect1_y1, rect1_x2, rect1_y2, color)?;
            canvas.rectangle(rect2_x1, rect2_y1, rect2_x2, rect2_y2, color)?;
        }

        Ok(true)
    }

    /// If button is clicked, toggle pause/resume state
    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        if self.paused {
            EVENT_CHANNEL.0.send(EventMessage::Resume)?;
        } else {
            EVENT_CHANNEL.0.send(EventMessage::Pause)?;
        }

        self.paused = !self.paused;

        Ok(true)
    }
}

impl Deref for PlayButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PlayButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
