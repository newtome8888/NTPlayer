// Hide console window on Windows platform, never remove it
#![windows_subsystem = "windows"]

mod config;
mod entity;
mod filemanager;
mod global_variables;
mod media;
mod ui;
mod util;

use log::error;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::VideoSubsystem;
use sdl2::{event::Event, Sdl};
use std::{sync::atomic::Ordering, time::Duration};

use entity::{EventMessage, PlayData};
use global_variables::{
    EventReceiver, EventSender, AUDIO_PTS, EVENT_CHANNEL, FORWARD_REWIND_AMOUNT,
};

use media::{
    decoder::MediaDecoder,
    player::{traits::Player, MediaPlayer},
};
use ui::components::dialog::show_error;
use ui::main_window::MainWindow;
use util::{
    error::{handle_send_result, SuperError},
    log_builder,
};

// Four threads, one for decoding, one for playing audio,
// one for playing video, maint thread for rendering audio and video

fn main() -> Result<(), SuperError> {
    log_builder::load_logger(log::LevelFilter::Warn);

    match NtApp::new() {
        Ok(mut app) => {
            // The error occurred while app running, should be logged and shown
            if let Err(err) = app.run() {
                error!("{}", err);
                show_error(err.to_string().as_str());
            }
        }
        Err(err) => {
            error!("{}", err);
            show_error(err.to_string().as_str());
            // App init error, it makes no sence to continue
            panic!()
        }
    }

    Ok(())
}

pub struct NtApp {
    sdl_context: Sdl,
    video: VideoSubsystem,
    main_window: MainWindow,
    decoder: Option<MediaDecoder>,
    player: Option<MediaPlayer>,
    sender: &'static EventSender,
    receiver: &'static EventReceiver,
}

impl NtApp {
    pub fn new() -> Result<Self, SuperError> {
        let _image_context = sdl2::image::init(InitFlag::JPG | InitFlag::PNG);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let wind = MainWindow::new(&video_subsystem)?;
        let sender = &EVENT_CHANNEL.0;
        let receiver = &EVENT_CHANNEL.1;

        Ok(Self {
            sdl_context,
            video: video_subsystem,
            main_window: wind,
            decoder: None,
            player: None,
            sender: &sender,
            receiver: &receiver,
        })
    }

    pub fn run(&mut self) -> Result<(), SuperError> {
        loop {
            if self.handle_sdl_events()? == MainLoopState::Quit {
                break;
            }

            if self.handle_custom_events()? == MainLoopState::Quit {
                break;
            }
        }

        Ok(())
    }

    /// Handler for sdl events, if the return value is Ok(false),
    /// means the main loop should be terminated, otherwise just continue
    fn handle_sdl_events(&mut self) -> Result<MainLoopState, SuperError> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(MainLoopState::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(MainLoopState::Quit),

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

    /// Handler for custom events, if the return value is Ok(false),
    /// means the main loop should be terminated, otherwise just continue
    fn handle_custom_events(&mut self) -> Result<MainLoopState, SuperError> {
        if let Ok(m) = self.receiver.recv_timeout(Duration::from_millis(50)) {
            match m {
                EventMessage::Play(data) => {
                    let mut md = MediaDecoder::new(data.path)?;
                    md.seek_to(0);

                    let mut plr = MediaPlayer::new();
                    plr.start(&md.media_summary);

                    self.decoder = Some(md);
                    self.player = Some(plr);
                }
                EventMessage::Pause => {
                    if let Some(player) = self.player.as_mut() {
                        player.pause();
                    }
                }
                EventMessage::Resume => {
                    if let Some(player) = self.player.as_mut() {
                        player.resume();
                    }
                }
                EventMessage::Stop => {
                    self.decoder.as_mut().unwrap().stop();
                    self.player.as_mut().unwrap().stop();
                }
                EventMessage::Forward => {
                    let start = AUDIO_PTS.load(Ordering::Acquire) + FORWARD_REWIND_AMOUNT;

                    self.player.as_mut().unwrap().pause();
                    self.decoder.as_mut().unwrap().seek_to(start);
                    self.player.as_mut().unwrap().resume();
                }
                EventMessage::Rewind => {
                    let start = AUDIO_PTS.load(Ordering::Acquire) - FORWARD_REWIND_AMOUNT;

                    self.player.as_mut().unwrap().pause();
                    self.decoder.as_mut().unwrap().seek_to(start);
                    self.player.as_mut().unwrap().resume();
                }
                EventMessage::FileOpened(data) => {
                    handle_send_result(
                        self.sender
                            .send(EventMessage::Play(PlayData { path: data.path })),
                    );
                }
                EventMessage::DirOpened(data) => {
                    todo!();
                }
                EventMessage::ShowError(msg) => {
                    show_error(msg.as_str());
                }
                EventMessage::RenderVideo(frame) => {
                    self.main_window.update_video_frame(frame);
                }
                EventMessage::RenderAudio(_) => todo!(),
                EventMessage::RenderSubtitle(_) => todo!(), // Render video and sound
            }
        }

        self.main_window.redrawl()?;

        Ok(MainLoopState::Continue)
    }
}

#[derive(Clone, PartialEq, Eq)]
enum MainLoopState {
    Continue,
    Quit,
}
