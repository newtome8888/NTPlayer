mod system_events;

use log::debug;
use sdl2::{image::InitFlag, AudioSubsystem, Sdl};
use std::{sync::atomic::Ordering, time::Duration, thread};

use crate::{
    entity::{EventMessage, PlayData},
    global::{
        AUDIO_SUMMARY, EVENT_CHANNEL, FR_STEP, GLOBAL_PTS_MILLIS, MAX_VOLUME, VIDEO_SUMMARY,
        VOLUME, VOLUME_STEP,
    },
    media::{
        decoder::MediaDecoder,
        player::{traits::Player, MediaPlayer},
    },
    sound::Sounder,
    ui::{components::dialog::show_error, main_window::MainWindow},
    util::error::{handle_result, safe_send, SuperError},
};

use self::system_events::SdlEvents;

pub struct NtApp {
    sdl_context: Sdl,
    audio_subsystem: AudioSubsystem,
    main_window: MainWindow,
}

impl NtApp {
    pub fn new() -> Result<Self, SuperError> {
        let _image_context = sdl2::image::init(InitFlag::JPG | InitFlag::PNG);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let audio_subsystem = sdl_context.audio()?;
        let wind = MainWindow::new(&video_subsystem)?;

        Ok(Self {
            sdl_context,
            audio_subsystem,
            main_window: wind,
        })
    }

    pub fn run(&mut self) -> Result<(), SuperError> {
        let mut sdl_eventer = SdlEvents::new(&self.sdl_context)?;
        let sender = &EVENT_CHANNEL.0;
        let receiver = &EVENT_CHANNEL.1;
        let mut decoder: Option<MediaDecoder> = None;
        let mut player: Option<MediaPlayer> = None;
        let mut sounder: Option<Sounder> = None;

        sender.send(EventMessage::Play(PlayData {
            path: r"E:\Movie\三体\[www.dbmp4.com]三体.EP07.HD1080p.mp4",
        }));
        loop {
            if sdl_eventer.handle_events()? == MainLoopState::Quit {
                break;
            }

            if let Ok(m) = receiver.recv_timeout(Duration::from_millis(50)) {
                match m {
                    EventMessage::Play(data) => {
                        let mut md = MediaDecoder::new(data.path)?;
                        md.seek_to(0);

                        let mut plr = MediaPlayer::new();
                        plr.start()?;

                        decoder = Some(md);
                        player = Some(plr);

                        // Everty time play new media, the audio summary will be changed,
                        // that's why the sounder is initialized here after media decoder is initialized
                        let r = AUDIO_SUMMARY.read().unwrap();
                        if let Some(summary) = r.as_ref() {
                            let sdr = Sounder::new(&self.audio_subsystem, summary);
                            sounder = Some(sdr);
                        }
                    }
                    EventMessage::Pause => {
                        if let Some(player) = player.as_mut() {
                            player.pause();
                        }
                    }
                    EventMessage::Resume => {
                        if let Some(player) = player.as_mut() {
                            player.resume();
                        }
                    }
                    EventMessage::Stop => {
                        if let Some(decoder) = decoder.as_mut() {
                            decoder.stop();
                        }
                        if let Some(player) = player.as_mut() {
                            player.stop();
                        }
                    }
                    EventMessage::Forward => {
                        debug!("forward");
                        if let (Some(player), Some(decoder)) = (player.as_mut(), decoder.as_mut()) {
                            let r = VIDEO_SUMMARY.read().unwrap();
                            let summary = r.as_ref().unwrap();

                            let pts = GLOBAL_PTS_MILLIS.load(Ordering::Acquire);
                            let start = pts + FR_STEP;

                            if start as u64 > summary.duration_millis {
                                decoder.stop();
                                player.stop();
                            } else {
                                player.seeking();
                                decoder.seek_to(start);
                            }
                        }
                    }
                    EventMessage::Rewind => {
                        debug!("rewind");
                        if let (Some(player), Some(decoder)) = (player.as_mut(), decoder.as_mut()) {
                            let adjust_diff = 2000;

                            let pts = GLOBAL_PTS_MILLIS.load(Ordering::Acquire);
                            let start = pts - FR_STEP - adjust_diff;
                            player.seeking();
                            decoder.seek_to(start);
                        }
                    }
                    EventMessage::FileOpened(data) => {
                        safe_send(sender.send(EventMessage::Play(PlayData { path: data.path })));
                    }
                    EventMessage::DirOpened(_) => {
                        todo!();
                    }
                    EventMessage::ShowError(msg) => {
                        show_error(msg.as_str());
                    }
                    EventMessage::RenderVideo(frame) => {
                        self.main_window.update_video_frame(frame);
                    }
                    EventMessage::RenderAudio(frame) => {
                        if let Some(sounder) = sounder.as_mut() {
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
                    EventMessage::UpVolume => {
                        let mut volume = VOLUME.load(Ordering::Acquire);
                        if volume <= MAX_VOLUME {
                            volume += VOLUME_STEP;
                            VOLUME.store(volume, Ordering::Release);
                        }
                    }
                    EventMessage::DownVolume => {
                        let mut volume = VOLUME.load(Ordering::Acquire);
                        if volume >= VOLUME_STEP {
                            volume -= VOLUME_STEP;
                            VOLUME.store(volume, Ordering::Release);
                        }
                    }
                    EventMessage::SeekFinished => {
                        if let Some(player) = player.as_mut() {
                            player.seek_finished();
                        }
                    }
                }
            }

            self.main_window.redrawl()?;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(in crate::app) enum MainLoopState {
    Continue,
    Quit,
}
