mod close_button;
mod maximize_button;
mod minimize_button;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{render::Canvas, video::Window};

use self::{
    close_button::CloseButton, maximize_button::MaximizeButton, minimize_button::MinimizeButton,
};
use crate::{
    ui::components::{
        rectangle::Rectangle, MouseDownParam, MouseMotionParam, MouseUpParam, TControl,
    },
    util::error::SuperError,
};

const TITLEBAR_HEIGHT: u32 = 40;

pub struct TitleBar {
    inner: Rectangle,
    close_button: CloseButton,
    maxmize_button: MaximizeButton,
    minimize_button: MinimizeButton,
    /// Indicate if user is operating on this control
    op_flag: bool,
}

impl TitleBar {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Result<Self, SuperError> {
        let (window_width, _) = canvas.clone().borrow().output_size()?;
        let x = 0;
        let y = 0;
        let width = window_width;
        let height = TITLEBAR_HEIGHT;

        let inner = Rectangle::new(x, y, width, height, canvas.clone())?;

        let btn_width: u32 = 40;
        let btn_height: u32 = 30;
        let btn_y = inner.center().1 - btn_height as i32 / 2;

        let close_btn_x = (window_width - btn_width) as i32;
        let max_btn_x = close_btn_x - btn_width as i32 - 5;
        let mini_button_x = max_btn_x - btn_width as i32 - 5;

        let close_button =
            CloseButton::new(close_btn_x, btn_y, btn_width, btn_height, canvas.clone())?;
        let maximize_button =
            MaximizeButton::new(max_btn_x, btn_y, btn_width, btn_height, canvas.clone())?;
        let minimize_button =
            MinimizeButton::new(mini_button_x, btn_y, btn_width, btn_height, canvas.clone())?;

        Ok(Self {
            inner,
            close_button,
            maxmize_button: maximize_button,
            minimize_button,
            op_flag: false,
        })
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

        // Adjust the position of controls within
        if let Some(width) = width {
            // Adjust position of close button since the titlebar size has been changed
            let clbtn_x = self.position().0 + width as i32 - self.close_button.size().0 as i32;
            self.close_button.set_position(clbtn_x, None);

            // Adjust position of maxmize button since the titlebar size has been changed
            let mxbtn_x = clbtn_x - self.maxmize_button.size().0 as i32;
            self.maxmize_button.set_position(mxbtn_x, None);

            // Adjust position of minimize button since the titlebar size has been changed
            let mibtn_x = mxbtn_x - self.minimize_button.size().0 as i32;
            self.minimize_button.set_position(mibtn_x, None);
        }
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        // If user is currently operating on canvas, show sub components
        if self.op_flag {
            self.close_button.render()?;
            self.maxmize_button.render()?;
            self.minimize_button.render()?;
        }

        Ok(true)
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_motion(params)? {
            self.op_flag = false;
            return Ok(false);
        }

        self.op_flag = true;

        self.close_button.on_mouse_motion(params)?;
        self.maxmize_button.on_mouse_motion(params)?;
        self.minimize_button.on_mouse_motion(params)?;

        Ok(true)
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            self.op_flag = false;
            return Ok(false);
        }

        self.op_flag = true;

        self.close_button.on_mouse_up(params)?;
        self.maxmize_button.on_mouse_up(params)?;
        self.minimize_button.on_mouse_up(params)?;

        Ok(true)
    }

    pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_down(params)? {
            self.op_flag = false;
            return Ok(false);
        }

        self.op_flag = true;

        Ok(true)
    }
}

impl Deref for TitleBar {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TitleBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
