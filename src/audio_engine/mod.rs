pub mod engine_status;
pub mod enums;
pub mod music_data;

use engine_status::EngineStatus;
use enums::*;
use music_data::MusicData;

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
            PlayerCommands::Clear => {
                self.player.clear();
                self.status.state = EngineState::Empty;
                self.status.timestamp = None;
            }
            PlayerCommands::Load(path) => {
                if let Err(e) = self.load_file(path) {
                    println!("Failed to load file {e}");
                } else if let EngineState::New = self.status.state {
                    self.status.state = EngineState::Playing;
                }
            }
            PlayerCommands::Play => {
                self.player.play();
                self.status.state = EngineState::Playing;
            }
            PlayerCommands::Pause => {
                self.player.pause();
                self.status.state = EngineState::Paused;
            }
            PlayerCommands::Stop => self.player.stop(),
        }
    }

    fn load_file(&self, path: PathBuf) -> Result<()> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let source = Decoder::try_from(buffer)?;
        self.player.append(source);

        Ok(())
    }
}
