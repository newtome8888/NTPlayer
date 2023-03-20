use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc, time::Instant,
};

use sdl2::{render::Canvas, video::Window};

use crate::util::error::SuperError;
use super::{ControlDistance, TControl};

pub trait TCircular {}

pub struct Circular {
    canvas: Rc<RefCell<Canvas<Window>>>,
    center_x: i32,
    center_y: i32,
    radius: u32,
    preclick: Instant
}

impl Circular {}

impl TCircular for Circular {}

impl TControl for Circular {
    /// Check if a point is in the specified circular
    /// # Arguments
    /// * `x`: The x coordinate of the point
    /// * `y`: The y coordinate of the point
    fn is_cursor_in(&mut self, x: i32, y: i32) -> bool {
        let x = x as i64;
        let y = y as i64;
        let center_x = self.center_x as i64;
        let center_y = self.center_y as i64;
        let radius = self.radius as i64;

        let x_distance = x - center_x;
        let y_distance = y - center_y;

        x_distance * x_distance + y_distance * y_distance <= radius * radius
    }

    /// Get the distance between a point and circular
    /// # Arguments
    /// * `x`: The x coordinate of the point
    /// * `y`: The y coordinate of the point
    fn distance(&self, x: i32, y: i32) -> ControlDistance {
        let x = x as i64;
        let y = y as i64;
        let center_x = self.center_x as i64;
        let center_y = self.center_y as i64;
        let radius = self.radius as f64;

        let x_distance = x - center_x;
        let y_distance = y - center_y;

        let sum_square = (x_distance * x_distance + y_distance * y_distance) as f64;
        let distance = (sum_square.sqrt() - radius) as f32;

        ControlDistance::Circular(distance)
    }

    fn render(&mut self) -> Result<(), SuperError> {
        todo!()
    }

    fn canvas(&self) -> Ref<Canvas<Window>> {
        self.canvas.borrow()
    }

    fn canvas_mut(&self) -> RefMut<Canvas<Window>> {
        self.canvas.borrow_mut()
    }

}
