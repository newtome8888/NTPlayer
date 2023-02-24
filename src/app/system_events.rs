use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    EventPump, Sdl,
};

use super::MainLoopState;
use crate::{
    entity::EventMessage,
    global::EVENT_CHANNEL,
    ui::{start_window::StartWindow, video_window::VideoWindow},
    util::error::{safe_send, SuperError},
};

pub(in crate::app) struct SdlEvents {
    event_pump: EventPump,
}

impl SdlEvents {
    pub(in crate::app) fn new(ctx: &Sdl) -> Result<Self, SuperError> {
        let event_pump = ctx.event_pump()?;

        Ok(Self { event_pump })
    }

    /// Handler for sdl events, if the return value is Ok(false),
    /// means the main loop should be terminated, otherwise just continue
    pub(in crate::app) fn handle_events(
        &mut self,
        start_window: &mut StartWindow,
        main_window: &mut Option<VideoWindow>,
    ) -> Result<MainLoopState, SuperError> {
        let sender = &EVENT_CHANNEL.0;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(MainLoopState::Quit),
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return Ok(MainLoopState::Quit),
                    Some(Keycode::Up) => {
                        safe_send(sender.send(EventMessage::UpVolume));
                    }
                    Some(Keycode::Down) => {
                        safe_send(sender.send(EventMessage::DownVolume));
                    }
                    Some(Keycode::Left) => {
                        safe_send(sender.send(EventMessage::Rewind));
                    }
                    Some(Keycode::Right) => {
                        safe_send(sender.send(EventMessage::Forward));
                    }
                    _ => {}
                },
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    let params = MouseMotionParameters {
                        timestamp,
                        window_id,
                        which,
                        mousestate,
                        x,
                        y,
                        xrel,
                        yrel,
                    };

                    start_window.on_mouse_motion(params);
                }
                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    let params = MouseUpParameters {
                        timestamp,
                        window_id,
                        which,
                        mouse_btn,
                        clicks,
                        x,
                        y,
                    };
                    start_window.on_mouse_up(params);
                }
                _ => return Ok(MainLoopState::Continue),
            }
        }
        Ok(MainLoopState::Continue)
    }
}

pub struct MouseUpParameters {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

pub struct MouseMotionParameters {
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub mousestate: MouseState,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}
