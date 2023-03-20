use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{render::Canvas, video::Window};

use self::{playbar::PlayBar, statebar::StateBar};
use crate::{
    ui::components::{
        rectangle::Rectangle, MouseDownParam, MouseMotionParam, MouseUpParam, TControl,
    },
    util::error::SuperError,
};

mod playbar;
mod statebar;

const HEIGHT: u32 = 70;
const STATEBAR_HEIGHT: u32 = 20;
const BAR_MARGIN: u32 = 10;
const PLAYBAR_HEIGHT: u32 = HEIGHT - STATEBAR_HEIGHT - BAR_MARGIN;
const MARGIN_BOTTOM: u32 = 10;

pub struct ControlBar {
    inner: Rectangle,
    playbar: PlayBar,
    statebar: StateBar,
}

impl ControlBar {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Result<Self, SuperError> {
        let (window_width, window_height) = canvas.borrow_mut().window_mut().drawable_size();
        let x = 0;
        let y = window_height as i32 - HEIGHT as i32;

        let mut inner = Rectangle::new(x, y, window_width, HEIGHT, canvas.clone())?;
        inner.set_margin(2, 2, 2, MARGIN_BOTTOM);

        let mut statebar = StateBar::new(x, y, window_width, STATEBAR_HEIGHT, canvas.clone())?;
        statebar.set_padding(None, None, 8, 8);

        let playbar_y = y + STATEBAR_HEIGHT as i32 + inner.margin().3 as i32;
        let playbar = PlayBar::new(x, playbar_y, window_width, PLAYBAR_HEIGHT, canvas.clone())?;

        Ok(Self {
            inner,
            playbar,
            statebar,
        })
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        self.statebar.on_mouse_up(params)?;
        self.playbar.on_mouse_up(params)?;

        Ok(true)
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_motion(params)? {
            return Ok(false);
        }

        self.statebar.on_mouse_motion(params)?;
        self.playbar.on_mouse_motion(params)?;

        Ok(true)
    }

    pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_down(params)? {
            return Ok(false);
        }

        self.statebar.on_mouse_down(params)?;
        self.playbar.on_mouse_down(params)?;

        Ok(true)
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        if self.is_cursorin() {
            self.statebar.render()?;
            self.playbar.render()?;
        }

        Ok(true)
    }

    pub fn set_size<W, H>(&mut self, width: W, height: H)
    where
        W: Into<Option<u32>>,
        H: Into<Option<u32>>,
    {
        let width: Option<u32> = width.into();
        let height: Option<u32> = height.into();

        // Set size of current control
        self.inner.set_size(width, height);

        // Adjust width of controls within
        self.statebar.set_size(width, None);
        self.playbar.set_size(width, None);
    }

    pub fn set_position<X, Y>(&mut self, x: X, y: Y)
    where
        X: Into<Option<i32>>,
        Y: Into<Option<i32>>,
    {
        let x: Option<i32> = x.into();
        let y: Option<i32> = y.into();

        // Set control position
        self.inner.set_position(x, y);

        // Set position of sub controls within
        self.statebar.set_position(x, y);
        if let Some(y) = y {
            self.playbar.set_position(x, y + STATEBAR_HEIGHT as i32);
        }
    }
}

impl Deref for ControlBar {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ControlBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
