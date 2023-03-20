mod forwardbutton;
mod nextbutton;
mod playbutton;
mod prebutton;
mod rewindbutton;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{render::Canvas, video::Window};

use crate::{
    ui::components::{rectangle::Rectangle, MouseMotionParam, MouseUpParam, TControl},
    util::error::SuperError,
};

use self::{
    forwardbutton::ForwardButton, nextbutton::NextButton, playbutton::PlayButton,
    prebutton::PreButton, rewindbutton::RewindButton,
};

const BUTTON_MARGIN: u32 = 15;

pub struct PlayBar {
    inner: Rectangle,
    playbutton: PlayButton,
    forwardbutton: ForwardButton,
    rewindbutton: RewindButton,
    prebutton: PreButton,
    nextbutton: NextButton,
}

impl PlayBar {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        let inner = Rectangle::new(x, y, width, height, canvas.clone())?;

        // The initial position of the buttons are not important,
        // since the position of the buttons will be changed later with video size
        // while video is playing. That's why they are all set the same initialized value.
        let (btn_width, btn_height) = (height, height);
        let playbutton = PlayButton::new(x, y, btn_width, btn_height, canvas.clone())?;
        let forwardbutton = ForwardButton::new(x, y, btn_width, btn_height, canvas.clone())?;
        let rewindbutton = RewindButton::new(x, y, btn_width, btn_height, canvas.clone())?;
        let prebutton = PreButton::new(x, y, btn_width, btn_height, canvas.clone())?;
        let nextbutton = NextButton::new(x, y, btn_width, btn_height, canvas)?;

        Ok(Self {
            inner,
            playbutton,
            forwardbutton,
            rewindbutton,
            prebutton,
            nextbutton,
        })
    }

    pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_motion(params)? {
            return Ok(false);
        }
        self.render()?;

        self.playbutton.on_mouse_motion(params)?;
        self.forwardbutton.on_mouse_motion(params)?;
        self.rewindbutton.on_mouse_motion(params)?;
        self.prebutton.on_mouse_motion(params)?;
        self.nextbutton.on_mouse_motion(params)?;

        Ok(true)
    }

    pub fn render(&mut self) -> Result<bool, SuperError> {
        self.playbutton.render()?;
        self.rewindbutton.render()?;
        self.forwardbutton.render()?;
        self.prebutton.render()?;
        self.nextbutton.render()?;

        Ok(true)
    }

    pub fn set_position<X, Y>(&mut self, x: X, y: Y)
    where
        X: Into<Option<i32>>,
        Y: Into<Option<i32>>,
    {
        let x: Option<i32> = x.into();
        let y: Option<i32> = y.into();

        // Set position of current control
        self.inner.set_position(x, y);

        // Set position of play button
        // Warn: Please mention that the `inner.center` function mutst be put
        // after the `inner.set_position` function to get the latest value
        let (center_x, center_y) = self.inner.center();

        // Set position of play button, keep its center the same with the bar center
        self.playbutton.set_center(center_x, center_y);
        let (play_x, _) = self.playbutton.position();

        // Set position of rewind button
        let (rewind_width, rewind_height) = self.rewindbutton.size();
        let (rewind_x, rewind_y) = (
            play_x - rewind_width as i32 - BUTTON_MARGIN as i32,
            center_y - rewind_height as i32 / 2,
        );
        self.rewindbutton.set_position(rewind_x, rewind_y);

        // Set position of pre button
        let (pre_width, pre_height) = self.prebutton.size();
        let (pre_x, pre_y) = (
            rewind_x - pre_width as i32 - BUTTON_MARGIN as i32,
            center_y - pre_height as i32 / 2,
        );
        self.prebutton.set_position(pre_x, pre_y);

        // Set position of forward button
        let (forward_width, forward_height) = self.forwardbutton.size();
        let (forward_x, forward_y) = (
            play_x + forward_width as i32 + BUTTON_MARGIN as i32,
            center_y - forward_height as i32 / 2,
        );
        self.forwardbutton.set_position(forward_x, forward_y);

        // Set position of next button
        let (_, next_height) = self.nextbutton.size();
        let (next_x, next_y) = (
            forward_x + forward_width as i32 + BUTTON_MARGIN as i32,
            center_y - next_height as i32 / 2,
        );
        self.nextbutton.set_position(next_x, next_y);
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        self.playbutton.on_mouse_up(params)?;
        self.rewindbutton.on_mouse_up(params)?;
        self.forwardbutton.on_mouse_up(params)?;
        self.prebutton.on_mouse_up(params)?;
        self.nextbutton.on_mouse_up(params)?;

        Ok(true)
    }
}

impl Deref for PlayBar {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PlayBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
