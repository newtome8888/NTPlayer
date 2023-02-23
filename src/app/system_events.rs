use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

use crate::{
    entity::EventMessage,
    global::EVENT_CHANNEL,
    util::error::{safe_send, SuperError},
};
use super::MainLoopState;

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
    pub(in crate::app) fn handle_events(&mut self) -> Result<MainLoopState, SuperError> {
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

                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    // trigger event handler
                    return Ok(MainLoopState::Continue);
                }
                _ => return Ok(MainLoopState::Continue),
            }
        }

        Ok(MainLoopState::Continue)
    }
}
