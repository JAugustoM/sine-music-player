pub enum PlayerCommands {
    Add(std::path::PathBuf),
    Clear,
    Load(std::path::PathBuf),
    ToggleReproduction,
    ToggleRepeat,
}

#[derive(Clone, PartialEq, Eq)]
pub enum EngineState {
    Empty,
    Paused,
    Playing,
}

#[derive(Clone, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    Track,
    Playlist,
}
