use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, AudioSubsystem, Sdl};
use std::{
    mem::MaybeUninit,
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use crate::{
    entity::{EventMessage, PlayData},
    global::{
        EventReceiver, EventSender, AUDIO_PTS_MILLIS, AUDIO_SUMMARY, EVENT_CHANNEL,
        FORWARD_REWIND_AMOUNT, VOLUME, VOLUME_STEP,
    },
    media::{
        decoder::MediaDecoder,
        player::{traits::Player, MediaPlayer},
    },
    sound::Sounder,
    ui::{components::dialog::show_error, main_window::MainWindow},
    util::error::{handle_result, handle_send_result, SuperError},
};

pub struct NtApp {
    sdl_context: Sdl,
    audio_subsystem: AudioSubsystem,
    main_window: MainWindow,
    sounder: Option<Sounder>,
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
        let audio_subsystem = sdl_context.audio()?;
        let wind = MainWindow::new(&video_subsystem)?;

        let sender = &EVENT_CHANNEL.0;
        let receiver = &EVENT_CHANNEL.1;

        Ok(Self {
            sdl_context,
            audio_subsystem,
            main_window: wind,
            sounder: None,
            decoder: None,
            player: None,
            sender: &sender,
            receiver: &receiver,
        })
    }

    pub fn run(&mut self) -> Result<(), SuperError> {
        self.sender.send(EventMessage::Play(PlayData {
            path: r"E:\Movie\三体\[www.dbmp4.com]三体.EP07.HD1080p.mp4",
        }));
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
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return Ok(MainLoopState::Quit),
                    Some(Keycode::Up) => {
                        VOLUME.fetch_add(VOLUME_STEP, Ordering::Acquire);
                    }
                    Some(Keycode::Down) => {
                        let volume = VOLUME.load(Ordering::Acquire);
                        if volume >= VOLUME_STEP {
                            VOLUME.fetch_sub(VOLUME_STEP, Ordering::Acquire);
                        }
                    }
                    Some(Keycode::Left) => {}
                    Some(Keycode::Right) => {}
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

    /// Handler for custom events, if the return value is Ok(false),
    /// means the main loop should be terminated, otherwise just continue
    fn handle_custom_events(&mut self) -> Result<MainLoopState, SuperError> {
        if let Ok(m) = self.receiver.recv_timeout(Duration::from_millis(50)) {
            match m {
                EventMessage::Play(data) => {
                    let mut md = MediaDecoder::new(data.path)?;
                    md.seek_to(0);

                    let mut plr = MediaPlayer::new();
                    plr.start()?;

                    self.decoder = Some(md);

                    // Everty time play new media, the audio summary will be changed,
                    // that's why the sounder is initialized here after media decoder is initialized
                    let r = AUDIO_SUMMARY.read().unwrap();
                    if let Some(summary) = r.as_ref() {
                        let sounder = Sounder::new(&self.audio_subsystem, summary);
                        self.sounder = Some(sounder);
                    }
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
                    let start = AUDIO_PTS_MILLIS.load(Ordering::Acquire) + FORWARD_REWIND_AMOUNT;

                    self.player.as_mut().unwrap().pause();
                    self.decoder.as_mut().unwrap().seek_to(start);
                    self.player.as_mut().unwrap().resume();
                }
                EventMessage::Rewind => {
                    let start = AUDIO_PTS_MILLIS.load(Ordering::Acquire) - FORWARD_REWIND_AMOUNT;

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
                EventMessage::RenderAudio(frame) => {
                    if let Some(sounder) = &self.sounder {
                        let result = sounder.play_sound(frame);
                        handle_result(result);
                    }
                }
                EventMessage::RenderSubtitle(_) => todo!(), // Render video and sound
                EventMessage::Resize((width, height)) => {
                    self.main_window.resize(width, height)?;
                }
                EventMessage::SetPosition { x, y } => {
                    self.main_window.set_position(x, y);
                }
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
