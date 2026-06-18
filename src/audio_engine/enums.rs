pub enum PlayerCommands {
    Clear,
    Load(std::path::PathBuf),
    Pause,
    Play,
    Stop,
}

#[derive(Clone)]
pub enum EngineState {
    Empty,
    New,
    Paused,
    Playing,
}
