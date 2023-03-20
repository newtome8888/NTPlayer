use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use log::{error, warn};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

use crate::{
    ui::{
        components::{rectangle::Rectangle, MouseDownParam, MouseMotionParam, TControl},
        DEFAULT_FONT_PATH, TTF_CONTEXT,
    },
    util::error::SuperError,
};

/// The left and right padding for the bar body，
/// it's located between the prefix/suffix and bar body
const BAR_LR_PADDING: u32 = 5;

/// The position of bar body
/// # Notice
/// * This variable is put here because its value cannot be set in render function,
/// due to the borrow mechanism and deref mechanism.
/// * If better solution is found later,
/// it could be put into the `ProgressBar` struct as part of the struct
/// # Warn
/// * This variable is is designed to be used in local thread,
/// it's forbidden to be used in multiple threads unless it's changed to multiple threads version.
static mut BAR_POSITION: (i32, i32) = (-1, -1);
/// The size of bar body
/// # Notices
/// * This variable is put here because its value cannot be set in render function,
/// due to the borrow mechanism and deref mechanism.
/// * If better solution is found later,
/// it could be put into the `ProgressBar` struct as part of the struct
/// # Warn
/// * This variable is is designed to be used in local thread,
/// it's forbidden to be used in multiple threads unless it's changed to multiple threads version.
static mut BAR_SIZE: (u32, u32) = (0, 0);

pub struct ProgressBar {
    inner: Rectangle,
    /// The maximum value of the progress bar
    max: u64,
    value: u64,
    /// The color of progress bar that displayed before current progress value
    unprogressed_color: Color,
    /// The color of progress bar that displayed after current progress value
    progressed_color: Color,
    cursorbutton_color: Color,
    /// The radius of cursor button
    cursorbutton_rad: i32,
    /// The text that will be displayed before the progress bar
    prefix: Option<String>,
    /// The text that will be displayed after the progress bar
    suffix: Option<String>,
    /// The font of suffix and prefix
    font: Font<'static, 'static>,
    /// The font color for suffix and prefix
    font_color: Color,
    /// The position of cursor if the cursor is currently in the progress bar
    cursor_position: (i32, i32),
}

#[allow(unused)]
impl ProgressBar {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = Rectangle::new(x, y, width, height, canvas)?;
        let font = TTF_CONTEXT.load_font(DEFAULT_FONT_PATH, 10)?;

        Ok(Self {
            inner,
            prefix: None,
            suffix: None,
            max: 100,
            value: 0,
            progressed_color: Color::RGB(65, 105, 225),
            unprogressed_color: Color::GREY,
            font,
            font_color: Color::WHITE,
            cursor_position: (-1, -1),
            cursorbutton_color: Color::WHITE,
            cursorbutton_rad: 5,
        })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let mut canvas = self.canvas_mut();
        let (content_x, content_y) = self.content_position();
        let (content_width, content_height) = self.content_size();
        let (_, center_y) = self.center();

        // Draw prefix
        let (mut prefix_width, mut prefix_height) = (0, 0);
        if let Some(prefix) = self.prefix.as_ref() {
            // Prepare texture
            let sfs = self.font.render(prefix.as_str()).blended(self.font_color)?;
            let tc = canvas.texture_creator();
            let texture = tc.create_texture_from_surface(sfs)?;
            // Compute size
            let query = texture.query();
            let width = query.width;
            let height = query.height;
            let y = center_y - height as i32 / 2;

            let rect = Rect::new(content_x, y, width, height);
            canvas.copy(&texture, None, rect)?;

            (prefix_width, prefix_height) = (width, height);
        }

        // Draw suffix
        let (mut suffix_width, mut suffix_height) = (0, 0);
        if let Some(suffix) = self.suffix.as_ref() {
            // Prepare texture
            let sfs = self.font.render(suffix.as_str()).blended(self.font_color)?;
            let tc = canvas.texture_creator();
            let texture = tc.create_texture_from_surface(sfs)?;
            // Compute size
            let query = texture.query();
            let width = query.width;
            let height = query.height;
            let x = content_x + content_width as i32 - width as i32;
            let y = center_y - height as i32 / 2;

            let rect = Rect::new(x, y, width, height);
            canvas.copy(&texture, None, rect)?;

            (suffix_width, suffix_height) = (width, height);
        }

        // Draw progress bar
        {
            let bar_x = content_x + prefix_width as i32 + BAR_LR_PADDING as i32;
            let bar_width = content_width - prefix_width - suffix_width - BAR_LR_PADDING * 2;

            unsafe {
                BAR_POSITION = (bar_x, content_y);
                BAR_SIZE = (bar_width, content_height);
            }

            if bar_width <= 0 {
                warn!("progress bar width is too small: {}", bar_width);
                return Ok(true);
            }

            let progressed_width = (bar_width as u64 * self.value / self.max) as u32;

            if bar_width < progressed_width {
                error!(
                    "progress bar width should not be smaller than progressed width: {}:{}",
                    bar_width, progressed_width
                );
            }

            let remain_width = bar_width - progressed_width;

            // Draw the body of progressed part
            let progressed_rect = Rect::new(bar_x, content_y, progressed_width, content_height);
            canvas.set_draw_color(self.progressed_color);
            canvas.fill_rect(progressed_rect)?;

            // Draw the body of unprogressed part
            let remain_start = bar_x + progressed_width as i32;
            let remain_rect = Rect::new(remain_start, content_y, remain_width, content_height);
            canvas.set_draw_color(self.unprogressed_color);
            canvas.draw_rect(remain_rect)?;
        }

        // Draw the slide button(filled circle) at the position of cursor
        {
            let x = self.cursor_position.0;
            let y = center_y;
            let rad = self.cursorbutton_rad;
            if x >= 0 {
                canvas.set_draw_color(self.cursorbutton_color);
                for i in -rad..=rad {
                    for j in -rad..=rad {
                        if i * i + j * j < rad * rad {
                            canvas.draw_point(Point::new(x + i, center_y + j))?;
                        }
                    }
                }
            }
        }

        Ok(true)
    }

    /// Set the max value of the progress
    pub fn set_max_value(&mut self, max: u64) {
        self.max = max;
    }

    /// Get the current value of the progress bar
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Set the current value of the progress bar
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }

    /// Set the prefix that will be displayed before the progress bar
    pub fn set_prefix<T: Into<Option<String>>>(&mut self, prefix: T) {
        self.prefix = prefix.into();
    }

    /// Set the suffix that will be displayed after the progress bar
    pub fn set_suffix<T: Into<Option<String>>>(&mut self, suffix: T) {
        self.suffix = suffix.into();
    }

    /// Set the color of the first part of the progress bar
    pub fn set_progressed_color(&mut self, color: Color) {
        self.progressed_color = color;
    }

    /// Set the color of the last part of the progress bar
    pub fn set_unprogressed_color(&mut self, color: Color) {
        self.unprogressed_color = color;
    }

    /// Set the font size of prefix and suffix
    pub fn set_font_size(&mut self, size: u16) -> Result<(), SuperError> {
        self.font = TTF_CONTEXT.load_font(DEFAULT_FONT_PATH, size)?;

        Ok(())
    }

    /// Set the font color of prefix and suffix
    pub fn set_font_color(&mut self, color: Color) {
        self.font_color = color;
    }

    /// Set the color of cursor button
    pub fn set_cursorbutton_color(&mut self, color: Color) {
        self.cursorbutton_color = color;
    }

    /// Set the radius of cursor button
    pub fn set_cursorbutton_rad(&mut self, rad: i32) {
        self.cursorbutton_rad = rad;
    }

    /// Handle the mouse down event of progress bar,
    /// if the clicked position is on the body of the bar,
    /// compute the corresponding value of that position
    pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_down(params)? {
            return Ok(false);
        }

        // Compute the current value by clicked position
        let x = params.x;
        let bar_x = unsafe { BAR_POSITION.0 };
        let bar_width = unsafe { BAR_SIZE.0 };

        // Only clicking on the bar is valid, do nothing out range of the bar
        if x < bar_x || x > (bar_x + bar_width as i32) {
            return Ok(false);
        }

        // update the current value with the position user clicked at
        let progressed_width = (x - bar_x) as u64;
        self.value = self.max * progressed_width / bar_width as u64;

        Ok(true)
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_motion(params)? {
            self.cursor_position = (-1, -1);

            return Ok(false);
        }

        self.cursor_position = (params.x, params.y);

        Ok(true)
    }
}

impl Deref for ProgressBar {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ProgressBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
