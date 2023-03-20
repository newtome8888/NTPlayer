use std::{ops::{Deref, DerefMut}, rc::Rc, cell::RefCell};

use sdl2::{render::Canvas, video::Window};

use crate::util::error::SuperError;

use super::Rectangle;

pub struct Panel{
    inner: Rectangle
}

impl Panel{
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
        })
    }
}

impl Deref for Panel{
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl  DerefMut for Panel{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}