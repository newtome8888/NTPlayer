use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::atomic::Ordering,
};

use sdl2::{render::Canvas, video::Window};

use crate::{
    entity::EventMessage,
    ui::components::{rectangle::progressbar::ProgressBar, MouseDownParam},
    util::error::{safe_send, SuperError},
    EVENT_CHANNEL, VIDEO_PTS_MILLIS, VIDEO_SUMMARY,
};

pub struct StateBar {
    inner: ProgressBar,
}

impl StateBar {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = ProgressBar::new(x, y, width, height, canvas)?;

        Ok(Self { inner })
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        let summary = VIDEO_SUMMARY.read()?;
        if let Some(summary) = summary.as_ref() {
            // Set max value
            let duration = summary.duration_millis;
            self.inner.set_max_value(duration);

            // Set suffix
            let suffix = Self::format_millis(duration);
            self.inner.set_suffix(suffix);
        }

        let pts = VIDEO_PTS_MILLIS.load(Ordering::Acquire) as u64;
        let prefix = Self::format_millis(pts);
        // Set current value
        self.inner.set_value(pts);
        // Set prefix
        self.inner.set_prefix(prefix);

        // Render contents
        self.inner.render()?;

        Ok(true)
    }

    /// If mouse is clicked in the body of the progress bar,
    /// skip to the corresponding position to play
    pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_down(params)? {
            return Ok(false);
        }

        let millis = self.value() as i64;
        safe_send(EVENT_CHANNEL.0.send(EventMessage::SeekTo(millis)));

        Ok(true)
    }

    /// Convert milliseconds to "HH:mm:ss" format
    fn format_millis(millis: u64) -> String {
        let total_secs = millis / 1000;

        let hours = (total_secs / 3600).to_string();
        let remainder = total_secs % 3600;
        let minutes = (remainder / 60).to_string();
        let secs = (remainder % 60).to_string();

        let formatted_time = [
            format!("{:0>2}", hours),
            format!("{:0>2}", minutes),
            format!("{:0>2}", secs),
        ]
        .join(":");

        formatted_time
    }
}

impl Deref for StateBar {
    type Target = ProgressBar;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StateBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
