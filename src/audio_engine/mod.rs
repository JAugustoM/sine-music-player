pub mod engine_commands;
pub mod engine_enums;
pub mod engine_status;
pub mod music;

mod engine_helpers;

use engine_enums::*;
use engine_status::EngineStatus;
use music::Music;

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use rodio::{MixerDeviceSink, Player};

pub struct AudioEngine {
    player: Player,
    command_rx: Receiver<PlayerCommands>,
    status_sx: Sender<EngineStatus>,
    status: EngineStatus,
}

impl AudioEngine {
    pub fn new(
        handle: &MixerDeviceSink,
        command_rx: Receiver<PlayerCommands>,
        status_sx: Sender<EngineStatus>,
    ) -> Self {
        let player = rodio::Player::connect_new(handle.mixer());
        let status = EngineStatus::new();

        AudioEngine {
            player,
            command_rx,
            status_sx,
            status,
        }
    }

    pub fn start(mut self) {
        thread::spawn(move || {
            loop {
                match self.command_rx.try_recv() {
                    Ok(command) => {
                        let mut latest_seek = None;

                        if let PlayerCommands::Seek(dur) = command {
                            latest_seek = Some(dur);
                        } else {
                            self.handle_command(command);
                        }

                        while let Ok(cmd) = self.command_rx.try_recv() {
                            if let PlayerCommands::Seek(dur) = cmd {
                                latest_seek = Some(dur);
                            } else {
                                self.handle_command(cmd);
                            }
                        }

                        if let Some(dur) = latest_seek {
                            self.handle_command(PlayerCommands::Seek(dur));
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        if self.player.empty() && *self.state() == EngineState::Playing {
                            match self.repeat() {
                                RepeatMode::Track => self.play_music(),
                                RepeatMode::Playlist => {
                                    self.status.current_track =
                                        (self.current_track() + 1) % self.playlist().len();
                                    self.play_music();
                                }
                                RepeatMode::Off => {
                                    if self.current_track() + 1 < self.playlist().len() {
                                        self.status.current_track += 1;
                                        self.play_music();
                                    } else {
                                        self.status.state = EngineState::Paused;
                                    }
                                }
                            }
                        }

                        if let EngineState::Playing = self.status.state {
                            self.status.timestamp = Some(self.player.get_pos());
                        }

                        if self.status_sx.send(self.status.clone()).is_err() {
                            break;
                        }

                        self.status.error = None;

                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
        });
    }

    fn handle_command(&mut self, command: PlayerCommands) {
        match command {
            PlayerCommands::Add(path) => self.add_music(path),
            PlayerCommands::AddFolder(path) => self.add_folder(path),
            PlayerCommands::Clear => self.clear_music(),
            PlayerCommands::ToggleReproduction => self.toggle_reproduction(),
            PlayerCommands::ToggleRepeat => self.toggle_repeat(),
            PlayerCommands::SkipPrevious => self.skip_previous(),
            PlayerCommands::SkipNext => self.skip_next(),
            PlayerCommands::Seek(duration) => self.seek(duration),
        }
    }
}
