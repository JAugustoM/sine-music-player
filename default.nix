{
  lib,
  rustPlatform,
  alsa-lib,
  cmake,
  dbus,
  fontconfig,
  libGL,
  libxkbcommon,
  makeWrapper,
  pkg-config,
  vulkan-loader,
  wayland,
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  nativeBuildInputs = [
    cmake
    makeWrapper
    pkg-config
  ];

  buildInputs = [
    alsa-lib
    fontconfig
    libxkbcommon
    wayland
  ];

  postInstall = ''
    wrapProgram $out/bin/${manifest.name} \
      --prefix LD_LIBRARY_PATH : ${
        lib.makeLibraryPath [
          dbus
          libGL
          libxkbcommon
          vulkan-loader
          wayland
        ]
      }
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "rodio-0.22.2" = "sha256-ja+7gQ0m8tC+wr+2qyn/0ON2oxlfK4cYhU4HWCqzn2U=";
    };
  };

  meta = {
    description = "A music player made with Rust + Slint";
    homepage = "https://github.com/JAugustoM/sine-music-player";
    license = lib.licenses.unlicense;
    maintainers = [ ];
  };
})
