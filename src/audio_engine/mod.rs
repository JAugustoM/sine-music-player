pub mod engine_status;
pub mod enums;
pub mod music;

mod engine_helpers;

use engine_status::EngineStatus;
use enums::*;
use music::Music;

use anyhow::Result;
use rodio::{Decoder, MixerDeviceSink, Player};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

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
                    Ok(command) => self.handle_command(command),
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
            PlayerCommands::Clear => {
                self.player.clear();
                self.status.playlist.clear();
                self.status.state = EngineState::Empty;
                self.status.timestamp = None;
            }
            PlayerCommands::Load(path) => {
                if let Err(e) = self.load_file(&path) {
                    println!("Failed to load file {e}");
                }
            }
            PlayerCommands::ToggleReproduction => self.toggle_reproduction(),
            PlayerCommands::ToggleRepeat => match self.status.repeat {
                RepeatMode::Off => self.status.repeat = RepeatMode::Track,
                RepeatMode::Track => self.status.repeat = RepeatMode::Playlist,
                RepeatMode::Playlist => self.status.repeat = RepeatMode::Off,
            },
        }
    }

    fn add_music(&mut self, path: PathBuf) {
        let music = Music::new(path);

        match music {
            Ok(music) => self.status.playlist.push(music),
            Err(e) => {
                println!("{e}");
                return;
            }
        }

        if *self.state() == EngineState::Empty {
            self.status.state = EngineState::Paused;
        }
    }

    fn toggle_reproduction(&mut self) {
        match self.status.state {
            EngineState::Empty => return,
            EngineState::Playing => {
                self.player.pause();
                self.status.state = EngineState::Paused;
            }
            EngineState::Paused => self.play_music(),
        }
    }

    fn play_music(&mut self) {
        let index = self.status.current_track;

        let music = &self.status.playlist[index];

        if let Ok(()) = self.load_file(&music.path) {
            self.player.play();
            self.status.state = EngineState::Playing;
        }
    }

    fn load_file(&self, path: &PathBuf) -> Result<()> {
        let file = File::open(path)?;
        println!("{:?}", path.file_name());
        let buffer = BufReader::new(file);
        let source = Decoder::try_from(buffer)?;

        self.player.append(source);

        Ok(())
    }
}
