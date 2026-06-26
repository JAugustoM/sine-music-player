pub enum PlayerCommands {
    Add(std::path::PathBuf),
    AddFolder(std::path::PathBuf),
    Clear,
    ToggleReproduction,
    ToggleRepeat,
    ToggleShuffle,
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

#[derive(Clone, PartialEq, Eq)]
pub enum ShuffleMode {
    Off,
    On,
}
