use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};

use crate::{util::error::SuperError, ui::{components::TControl, TTF_CONTEXT, DEFAULT_FONT_PATH}};

use super::Rectangle;

pub struct Label {
    inner: Rectangle,
    text: &'static str,
    font_size: u16,
}

impl Label {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        text: &'static str,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let mut inner = Rectangle::new(x, y, width, height, canvas)?;
        // Set the default font color as black, which is usually adopted by user
        inner.foreground_color = Color::BLACK;

        Ok(Self { inner, text, font_size: 10 })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        self.inner.render()?;

        let mut canvas = self.canvas_mut();
        let ttf = TTF_CONTEXT.load_font(DEFAULT_FONT_PATH, self.font_size)?;
        let sfs = ttf.render(self.text).blended(self.foreground_color)?;
        let tc = canvas.texture_creator();
        let texture = tc.create_texture_from_surface(sfs)?;
        let (cx, cy) = self.content_position();
        let (cw, ch) = self.content_size();
        let dst_rect = Rect::new(cx, cy, cw, ch);

        canvas.copy(&texture, None, dst_rect)?;

        Ok(true)
    }


}

impl Deref for Label {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
