use std::{cell::RefCell, rc::Rc};

use log::{debug, error};
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor, MouseButton, SystemCursor},
    rect::Rect,
    render::Canvas,
    surface::Surface,
    sys::{CursorShape, SDL_SetCursor, SDL_SetSurfaceAlphaMod},
    video::Window,
};

use crate::{
    app::{MouseMotionParameters, MouseUpParameters},
    entity::{EventMessage, FileOpenedData},
    global::EVENT_CHANNEL,
    util::error::{safe_send, SuperError},
};

pub struct PlayButton {
    canvas: Rc<RefCell<Canvas<Window>>>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    center: (i32, i32),
    cursor_state: SystemCursor,
}

impl PlayButton {
    pub fn default(canvas: Rc<RefCell<Canvas<Window>>>) -> Result<Self, SuperError> {
        Self::new(0, 0, 50, 30, canvas)
    }

    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let canvas_clone = canvas.clone();
        let (window_w, window_h) = canvas_clone.borrow().window().size();
        let center_x = window_w / 2;
        let center_y = window_h / 2;

        Ok(Self {
            x,
            y,
            width,
            height,
            canvas,
            center: (center_x as i32, center_y as i32),
            cursor_state: SystemCursor::Arrow,
        })
    }

    pub fn render(&mut self) -> Result<(), SuperError> {
        let canvas = self.canvas.clone();
        let mut canvas_mut = canvas.borrow_mut();
        let texture_creator = canvas_mut.texture_creator();
        let sfs = Surface::from_file("./assets/play_black_circle.png")?;
        let texture = texture_creator.create_texture_from_surface(sfs)?;

        canvas_mut.clear();
        canvas_mut.copy(
            &texture,
            None,
            Rect::new(self.x, self.y, self.width, self.height),
        )?;

        Ok(())
    }

    /// This method should be put at the center of method chain,
    /// otherwise the position will be affected by new size of window
    pub fn with_center(mut self) -> Self {
        let (center_x, center_y) = self.center;

        let x = center_x - self.width as i32 / 2;
        let y = center_y - self.height as i32 / 2;
        self.x = x as i32;
        self.y = y as i32;

        self
    }

    /// Make this control fullfill the whole window frame
    pub fn with_fullfill(mut self) -> Self {
        let canvas = self.canvas.clone();
        let (window_w, window_h) = canvas.borrow().window().size();
        self.width = window_w;
        self.height = window_h;

        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;

        self
    }

    /// Handle mouse up event, trigger event if coditions are satisfied
    pub fn on_mouse_up(&mut self, params: MouseUpParameters) {
        if params.mouse_btn == MouseButton::Left && self.is_in_area(params.x, params.y) {
            // Play button is clicked, open file
            let path = rfd::FileDialog::new().pick_file();
            if let Some(path) = path {
                safe_send(
                    EVENT_CHANNEL
                        .0
                        .send(EventMessage::FileOpened(FileOpenedData { path: path })),
                );
            }
        }
    }

    /// Mouse move event handler
    pub fn on_mouse_motion(&mut self, params: MouseMotionParameters) {
        if self.is_in_area(params.x, params.y) && self.cursor_state != SystemCursor::Hand {
            match Cursor::from_system(SystemCursor::Hand) {
                Ok(cursor) => {
                    cursor.set();
                },
                Err(err) => {
                    error!("{}", err);
                }
            }
        } else if self.is_out_area(params.x, params.y) && self.cursor_state != SystemCursor::Arrow {
            debug!("is out area");
            match Cursor::from_system(SystemCursor::Arrow) {
                Ok(cursor) => cursor.set(),
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
    }

    /// Check if the mouse button is moved into the area of this control
    fn is_in_area(&self, x: i32, y: i32) -> bool {
        return (x >= self.x && x <= self.x + self.width as i32)
            && (y >= self.y && y <= self.y + self.height as i32);
    }

    /// Check if the mouse button is moved out of the area of this control
    fn is_out_area(&self, x: i32, y: i32) -> bool {
        let max_distance = 10;

        let x_start = self.x;
        let x_end = self.x + self.width as i32;
        let y_start = self.y;
        let y_end = self.y + self.height as i32;

        return (x < x_start && x >= (x_start - max_distance))
            || (x > x_end && x <= (x_end + max_distance))
            || (y < y_start && y >= (y_start - max_distance))
            || (y > y_end && y <= (y_end + max_distance));
    }
}
