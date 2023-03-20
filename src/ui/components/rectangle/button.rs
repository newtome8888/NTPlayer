use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{image::LoadSurface, rect::Rect, render::Canvas, surface::Surface, video::Window};

use crate::{ui::components::TControl, util::error::SuperError};

use super::Rectangle;

#[allow(unused)]
pub struct Button {
    inner: Rectangle,
    /// The background image path.
    /// Only bmp and png format are supported currently.
    bacground_image: Option<&'static str>,
    /// The background image path while cursor in this control.
    /// Only bmp and png format are supported currently.
    cursonin_background_image: Option<&'static str>,
}

#[allow(unused)]
impl Button {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = Rectangle::new(x, y, width, height, canvas)?;

        Ok(Self {
            inner,
            bacground_image: None,
            cursonin_background_image: None,
        })
    }

    pub fn render(&mut self) -> Result<(), SuperError> {
        self.inner.render()?;

        let mut image_path = self.bacground_image;
        if self.is_cursorin && self.cursonin_background_image.is_some() {
            image_path = self.cursonin_background_image;
        }

        if let Some(image_path) = image_path {
            let mut canvas = self.canvas_mut();
            let sfs = Surface::from_file(image_path)?;
            let tc = canvas.texture_creator();
            let texture = tc.create_texture_from_surface(sfs)?;
            let dst_rect = Rect::new(self.position.0, self.position.1, self.size.0, self.size.1);
            canvas.copy(&texture, None, dst_rect)?;
        }

        Ok(())
    }

    pub fn set_background_image<T: Into<Option<&'static str>>>(&mut self, image: T) {
        self.cursonin_background_image = image.into();
    }

    pub fn set_cursorin_background_color<T: Into<Option<&'static str>>>(&mut self, image: T){
        self.cursonin_background_image = image.into();
    }
}

impl Deref for Button {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
