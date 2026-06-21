pub enum PlayerCommands {
    Add(std::path::PathBuf),
    Clear,
    ToggleReproduction,
    ToggleRepeat,
    SkipNext,
    SkipPrevious,
    Seek(std::time::Duration),
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
