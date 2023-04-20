pub mod button;
pub mod label;
pub mod panel;
pub mod progressbar;
pub mod tree;

use std::{cell::RefCell, rc::Rc, time::Instant};

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use super::{ControlDistance, DistanceDirection, TControl};
use crate::util::error::SuperError;

/// The basic struct of Rectangle control,
/// All the controls with rectangle shape should extend this struct.
/// This struct provide the basic and common functionalities for rectangle controls.
pub struct Rectangle {
    /// The canvas which responsible for drawing
    canvas: Rc<RefCell<Canvas<Window>>>,
    /// The position of control
    /// # Format
    /// (x, y)
    position: (i32, i32),
    /// The size of control
    /// # Format
    /// (width, height)
    size: (u32, u32),
    /// The internal padding of control
    /// # Format
    /// (left, right, top, bottom)
    padding: (u32, u32, u32, u32),
    /// The outside margin of control
    /// # Format
    /// (top, bottom, left, right)
    margin: (u32, u32, u32, u32),
    /// The radius of control if the control is rounded cornor
    radius: u32,
    /// The background color of control
    background_color: Color,
    /// The background color of control while the control is cursor_in
    cursorin_background_color: Color,
    /// The color of content in the control
    foreground_color: Color,
    /// The foreground color of control while the control is cursor_in
    cursorin_foreground_color: Color,
    /// The border color of control
    border_color: Option<Color>,
    /// The border color of control while the control is cursor_in
    cursorin_border_color: Option<Color>,
    /// Indicates how the contents be alligned in control
    align: Align,
    /// Indicate if the cursor is currently in this control
    is_cursorin: bool,
    /// The time of pre-click
    preclick: Instant,
}

#[allow(unused)]
impl Rectangle {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        Ok(Self {
            position: (x, y),
            size: (width, height),
            canvas,
            padding: (5, 5, 5, 5),
            margin: (5, 5, 5, 5),
            radius: 0,
            background_color: Color::BLACK,
            foreground_color: Color::WHITE,
            align: Align::Left,
            border_color: None,
            cursorin_background_color: Color::BLACK,
            cursorin_foreground_color: Color::WHITE,
            cursorin_border_color: None,
            is_cursorin: false,
            preclick: Instant::now(),
        })
    }
    /// Handle control resized event
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    /// # Warning
    /// This method is not implemented yet, you have to implement it in your own way
    pub fn on_resized(&mut self, _width: u32, _height: u32) -> Result<bool, SuperError> {
        todo!()
    }
    /// Get the size of current control
    /// # Returns
    /// (width, height)
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
    pub fn size16(&self) -> (u16, u16) {
        (self.size.0 as u16, self.size.1 as u16)
    }
    /// Set the size of current control
    /// # Arguments
    /// * `width` - the new width of the control
    /// * `height` - the new height of the control
    pub fn set_size<W, H>(&mut self, width: W, height: H)
    where
        W: Into<Option<u32>>,
        H: Into<Option<u32>>,
    {
        if let Some(width) = width.into() {
            self.size.0 = width;
        }
        if let Some(height) = height.into() {
            self.size.1 = height;
        }
    }
    /// Get the position of current control
    /// # Returns
    /// (x, y)
    pub fn position(&self) -> (i32, i32) {
        self.position
    }
    pub fn position16(&self) -> (i16, i16) {
        (self.position.0 as i16, self.position.1 as i16)
    }
    /// Set the position of current control
    /// # Arguments
    /// * `x` - the new x position of the control
    /// * `y` - the new y position of the control
    pub fn set_position<X, Y>(&mut self, x: X, y: Y)
    where
        X: Into<Option<i32>>,
        Y: Into<Option<i32>>,
    {
        if let Some(x) = x.into() {
            self.position.0 = x;
        }
        if let Some(y) = y.into() {
            self.position.1 = y;
        }
    }
    /// Get the center position of current control
    /// # Returns
    /// (center_x, center_y)
    pub fn center(&self) -> (i32, i32) {
        let center_x = self.position.0 + self.size.0 as i32 / 2;
        let center_y = self.position.1 + self.size.1 as i32 / 2;

        (center_x, center_y)
    }
    pub fn center16(&self) -> (i16, i16) {
        let (center_x, center_y) = self.center();
        (center_x as i16, center_y as i16)
    }
    /// Set the center position of current control
    /// # Arguments
    /// * `center_x` - the x coordinate that will be set as center x
    /// * `center_y` - the y coordinate that will be set as center y
    pub fn set_center<X, Y>(&mut self, center_x: X, center_y: Y)
    where
        X: Into<Option<i32>>,
        Y: Into<Option<i32>>,
    {
        if let Some(center_x) = center_x.into() {
            let x = center_x - self.size.0 as i32 / 2;
            self.set_position(x, None);
        }
        if let Some(center_y) = center_y.into() {
            let y = center_y - self.size.1 as i32 / 2;
            self.set_position(None, y);
        }
    }
    pub fn radius(&self) -> u32 {
        self.radius
    }
    pub fn radius16(&self) -> u16 {
        self.radius as u16
    }
    /// Set the radius of current control
    pub fn set_radius(&mut self, radius: u32) {
        self.radius = radius;
    }
    /// Get the padding of current control
    /// # Returns
    /// (left, right, top, bottom)
    pub fn padding(&self) -> (u32, u32, u32, u32) {
        self.padding
    }
    pub fn padding16(&self) -> (u16, u16, u16, u16) {
        (
            self.padding.0 as u16,
            self.padding.1 as u16,
            self.padding.2 as u16,
            self.padding.3 as u16,
        )
    }
    /// Set the padding of current control
    pub fn set_padding<L, R, T, B>(&mut self, left: L, right: R, top: T, bottom: B)
    where
        L: Into<Option<u32>>,
        R: Into<Option<u32>>,
        T: Into<Option<u32>>,
        B: Into<Option<u32>>,
    {
        if let Some(left) = left.into() {
            self.padding.0 = left;
        }
        if let Some(right) = right.into() {
            self.padding.1 = right;
        }
        if let Some(top) = top.into() {
            self.padding.2 = top;
        }
        if let Some(bottom) = bottom.into() {
            self.padding.3 = bottom;
        }
    }
    /// Get the size of current control
    /// # Returns
    /// (left, right, top, bottom)
    pub fn margin(&self) -> (u32, u32, u32, u32) {
        self.margin
    }
    pub fn margin16(&self) -> (u16, u16, u16, u16) {
        (
            self.margin.0 as u16,
            self.margin.1 as u16,
            self.margin.2 as u16,
            self.margin.3 as u16,
        )
    }
    /// Set the margin of current control
    pub fn set_margin<L, R, T, B>(&mut self, left: L, right: R, top: T, bottom: B)
    where
        L: Into<Option<u32>>,
        R: Into<Option<u32>>,
        T: Into<Option<u32>>,
        B: Into<Option<u32>>,
    {
        if let Some(left) = left.into() {
            self.margin.0 = left;
        }
        if let Some(right) = right.into() {
            self.margin.1 = right;
        }
        if let Some(top) = top.into() {
            self.margin.2 = top;
        }
        if let Some(bottom) = bottom.into() {
            self.margin.3 = bottom;
        }
    }
    /// Get the background color of current control
    pub fn background_color(&self) -> Color {
        self.background_color
    }
    /// Set the background color of current control
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    /// Get the background color of current control while the control is cursor_in
    pub fn cursorin_background_color(&self) -> Color {
        self.cursorin_background_color
    }
    /// Set the background color of current control while the control is cursor_in
    pub fn set_cursorin_background_color(&mut self, color: Color) {
        self.cursorin_background_color = color;
    }
    /// Get the foreground color of current control
    pub fn foreground_color(&self) -> Color {
        self.foreground_color
    }
    /// Set the foreground color of current control
    pub fn set_foreground_color(&mut self, color: Color) {
        self.foreground_color = color;
    }
    /// Get the foreground color of current control while the control is cursor_in
    pub fn cursorin_foreground_color(&self) -> Color {
        self.cursorin_foreground_color
    }
    /// Set the foreground color of current control while the control is cursor_in
    pub fn set_cursorin_foreground_color(&mut self, color: Color) {
        self.cursorin_foreground_color = color
    }
    /// Get the alignment of current control
    pub fn align(&self) -> &Align {
        &self.align
    }
    /// Set the alignment of current control
    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }
    /// Get the border color of current control
    pub fn border_color(&self) -> Option<Color> {
        self.border_color
    }
    /// Set the border color of current control
    pub fn set_border_color<T: Into<Option<Color>>>(&mut self, color: T) {
        self.border_color = color.into();
    }
    /// Get the border color of current control while the cursor is in this control
    pub fn cursorin_border_color(&self) -> Option<Color> {
        self.cursorin_border_color
    }
    /// Set the border color of current control while the cursor is in this control
    pub fn set_cursorin_border_color(&mut self, color: Color) {
        self.cursorin_border_color = Some(color);
    }
    /// Get the content size of current control,
    /// this size excludes the padding space
    /// # Returns
    /// (content_width, content_height)
    pub fn content_size(&self) -> (u32, u32) {
        let content_width = self.size.0 - self.padding.0 - self.padding.1;
        let content_height = self.size.1 - self.padding.2 - self.padding.3;

        (content_width, content_height)
    }
    /// Get the u16 format content size of current control,
    /// this size excludes the padding space
    /// # Returns
    /// (content_width, content_height)
    pub fn content_size16(&self) -> (u16, u16) {
        let (content_width, content_height) = self.content_size();

        (content_width as u16, content_height as u16)
    }
    /// Get the content coordinates(x, y) of current control,
    /// this coordinates excludes the padding space
    pub fn content_position(&self) -> (i32, i32) {
        let content_x = self.position.0 + self.padding.0 as i32;
        let content_y = self.position.1 + self.padding.2 as i32;

        (content_x, content_y)
    }
    /// Get the i16 format content coordinates(x, y) of current control,
    /// this coordinates excludes the padding space
    pub fn content_position16(&self) -> (i16, i16) {
        let (content_x, content_y) = self.content_position();

        (content_x as i16, content_y as i16)
    }
    /// If the cursor is currently in this control
    /// # Notice
    /// The difference between this function and the function `is_cursor_in` is
    /// that `is_cursor_in` function compute the position of the cursor and returns
    /// the result, and this function only returns the result
    pub fn is_cursorin(&self) -> bool {
        self.is_cursorin
    }
}

impl TControl for Rectangle {
    fn render(&mut self) -> Result<(), SuperError> {
        let (x, y) = self.position16();
        let (width, height) = self.size16();
        let (end_x, end_y) = (x + width as i16, y + height as i16);
        let radius = self.radius as i16;

        let canvas = self.canvas();
        // Draw background
        let background_color = if self.is_cursorin {
            self.cursorin_background_color
        } else {
            self.background_color
        };
        canvas.rounded_box(x, y, end_x, end_y, radius, background_color)?;

        // Draw border
        let border_color = if self.is_cursorin {
            self.cursorin_border_color
        } else {
            self.border_color
        };
        if let Some(border_color) = border_color {
            canvas.rounded_rectangle(x, y, end_x, end_y, radius, border_color)?;
        }

        // TODO: implement the content part in concrete struct,
        // please take part in the example in the `Retangle` strait
        // for how to implement render in concrete struct

        Ok(())
    }

    fn is_cursor_in(&mut self, x: i32, y: i32) -> bool {
        if (x >= self.position.0 && x <= self.position.0 + self.size.0 as i32)
            && (y >= self.position.1 && y <= self.position.1 + self.size.1 as i32)
        {
            self.is_cursorin = true;
            true
        } else {
            self.is_cursorin = false;
            false
        }
    }

    fn distance(&self, x: i32, y: i32) -> ControlDistance {
        let x_start = self.position.0;
        let x_end = self.position.0 + self.size.0 as i32;
        let y_start = self.position.1;
        let y_end = self.position.1 + self.size.1 as i32;

        let (distance, direction) = if x < x_start {
            ((x - x_start) as u32, DistanceDirection::Left)
        } else if x > x_end {
            ((x_end - x) as u32, DistanceDirection::Right)
        } else if y < y_start {
            ((y_start - y) as u32, DistanceDirection::Up)
        } else if y > y_end {
            ((y - y_end) as u32, DistanceDirection::Down)
        } else {
            (0, DistanceDirection::Inside)
        };

        ControlDistance::Rectangle(distance, direction)
    }

    fn canvas(&self) -> std::cell::Ref<Canvas<Window>> {
        self.canvas.borrow()
    }

    fn canvas_mut(&self) -> std::cell::RefMut<Canvas<Window>> {
        self.canvas.borrow_mut()
    }

    fn on_mouse_up(&mut self, params: &super::MouseUpParam) -> Result<bool, SuperError> {
        if !self.is_cursor_in(params.x, params.y) {
            return Ok(false);
        }

        self.preclick = Instant::now();

        Ok(true)
    }
}

/// Align type of control
pub enum Align {
    /// Indicates that the control is centered with the specified coordinates(both x and y)
    Center,
    /// Indicates that the control is centered with the specified x coordinate
    HorizontalCenter,
    /// Indicates that the control is centered with the specified y coordinate
    VerticalCenter,
    /// Indicates that the control is aligned with top at the specified y coordinate
    Top,
    /// Indicates that the control is aligned with bottom at the specified y coordinate
    Bottom,
    /// Indicates that the control is aligned with left at the specified x coordinate
    Left,
    /// Indicates that the control is aligned with right at the specified x coordinate
    Right,
}
