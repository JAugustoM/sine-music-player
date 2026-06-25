# Sine Music Player

A desktop music player built with [Rust](https://www.rust-lang.org/) and the [Slint](https://slint.dev/) UI framework.

## Folder Structure

The repository is organized to separate the backend logic from the frontend UI components:

```text
.
├── src/                # Rust backend source code
│   ├── audio_engine/   # Audio playback, status tracking, and music control logic
│   ├── callbacks.rs    # Bridges Slint UI signals to Rust logic
│   ├── helpers.rs      # Utility functions
│   └── main.rs         # Application entry point
├── ui/                 # Slint UI frontend source code
│   ├── assets/         # SVG icons and visual assets
│   ├── global/         # Global Slint states/singletons (e.g., music_control.slint)
│   ├── lib/            # External UI libraries (Material Design 1.0 components)
│   ├── theme/          # Theming configurations (e.g., Dracula theme)
│   └── main.slint      # Main UI layout and window definition
├── build.rs            # Rust build script (compiles Slint files)
├── Cargo.toml          # Rust dependencies and project manifest
├── flake.nix           # Nix flake for reproducible builds and dev shells
└── default.nix         # Nix package definition
```

## Getting Started

### Prerequisites

* [Rust / Cargo](https://rustup.rs/) (latest stable)
* If using the provided flake, the [Nix package manager](https://nixos.org/download.html) with flake support enabled.

### Building and Running

**Standard Rust Setup:**
Clone the repository and run the project using Cargo:
```bash
git clone https://github.com/JAugustoM/sine-music-player.git
cd sine-music-player
cargo run --release
```

**Nix Environment:**
If you manage your environment with Nix, you can drop right into a fully equipped development shell or run the app directly:
```bash
# Run the application directly
nix run

# Or enter the development shell
nix develop
cargo run
```