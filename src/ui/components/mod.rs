pub mod circular;
pub mod dialog;
pub mod rectangle;
pub mod triangle;

use std::{cell::{Ref, RefMut}};

use sdl2::{
    mouse::{MouseButton, MouseState, MouseWheelDirection},
    render::Canvas,
    video::Window,
};

use crate::util::error::SuperError;

const DOUBLE_CLICK_INTERVAL: u128 = 200;

pub trait TControl {
    /// Handle mouse botton down event
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if !self.is_cursor_in(params.x, params.y) {
            return Ok(false);
        }

        Ok(true)
    }
    /// Handle mouse botton up event
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.is_cursor_in(params.x, params.y) {
            return Ok(false);
        }

        Ok(true)
    }
    /// Handle mouse move event
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if !self.is_cursor_in(params.x, params.y) {
            return Ok(false);
        }

        Ok(true)
    }
    /// Handle mouse wheel event
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    fn on_mouse_wheel(&mut self, params: &MouseWheelParam) -> Result<bool, SuperError> {
        if !self.is_cursor_in(params.x, params.y) {
            return Ok(false);
        }

        Ok(true)
    }
    /// Render the data of current control to the screen
    /// # Returns
    /// * `bool` - true if the event is handled, false if the event is rejected.
    /// The reason for rejection may be the cursor is not in the area of current control,
    /// or other reasons.
    /// * `SuperError` - the error information
    /// # Warning
    /// * This function is not completely implemented yet,
    /// in this basic struct, we finished drawing background and border, 
    /// and leave the foreground part to user who call this function. 
    /// Depends on the concrete basic structs, there may be different degrees of implementation,
    /// please check the basic struct of your own type.
    /// # Example
    /// Let's take a example of how to render in rectangle control:
    /// ```
    /// struct RectControl{
    ///     inner: Rectangle
    /// }
    /// 
    /// impl RectControl{
    ///     pub fn new(x: i32, y: i32, width: u32, height: u32, canvas: Rc<RefCell<Canvas<Window>>>)
    ///     -> Result<Self, SuperError>{
    ///         let inner: Rectangle = Rectangle::new(x, y, width, height, canvas.clone());
    ///         Ok(Self{
    ///             inner: inner,
    ///         })
    ///     }
    /// 
    ///     pub fn render(&self) -> Result<bool, SuperError>{
    ///         // Call the render function of parent control before drawing contents, 
    ///         // since the basic struct(Rectangle) has done some preparing work,
    ///         // such as background, border, etc.
    ///         self.inner.render()?;
    ///         
    ///         // Draw contents
    ///         let canvas_mut = self.canvas_mut();
    ///         let (content_x, content_y) = self.content_position16();
    ///         let (content_width, content_height) = self.content_size16();
    ///         canvas.line(
    ///            content_x,
    ///            content_y,
    ///            content_x + content_width as i16,
    ///            content_y + content_height as i16,
    ///            self.foreground_color(),
    ///         )?;
    ///         
    ///         Ok(true)
    ///     }
    /// }
    /// 
    /// impl Deref for RectControl{
    ///     type Target = Rectangle;
    /// 
    ///     fn deref(&self) -> &Target{
    ///         &self.inner
    ///     }
    /// }
    /// 
    /// impl DerefMut for RectControl{
    ///     fn deref_mut(&mut self) -> &mut Target{
    ///         &mut self.inner
    ///     }
    /// }
    /// ```
    fn render(&mut self) -> Result<(), SuperError>;
    /// Check if the cursor is currently in this control
    /// # Arguments:
    /// * `x`: The x coordinate of the cursor
    /// * `y`: The y coordinate of the cursor
    /// # Returns
    /// * `true` if the cursor is in the area of current control
    /// * `false` if the cursor is not in the area of current control
    fn is_cursor_in(&mut self, x: i32, y: i32) -> bool;
    /// Compute the distance between specified point and specified rectangle
    /// # Arguments:
    /// * `x`: The x coordinate of the point
    /// * `y`: The y coordinate of the point
    /// # Returns
    /// * `u32` - the distance between the point and the rectangle
    /// * `DistanceDirection` - the direction of the point located at the rectangle
    fn distance(&self, x: i32, y: i32) -> ControlDistance;
    fn canvas(&self) -> Ref<Canvas<Window>>;
    fn canvas_mut(&self) -> RefMut<Canvas<Window>>;

}

/// Distance type of control
pub enum ControlDistance {
    Rectangle(u32, DistanceDirection),
    Circular(f32),
}

pub enum DistanceDirection {
    Inside,
    Up,
    Down,
    Left,
    Right,
}

pub struct MouseDownParam {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

pub struct MouseUpParam {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

pub struct MouseMotionParam {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub mousestate: MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

pub struct MouseWheelParam {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: MouseWheelDirection,
}
