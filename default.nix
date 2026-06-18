{
  lib,
  rustPlatform,
  alsa-lib,
  cmake,
  fontconfig,
  pkg-config,
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
    pkg-config
  ];

  buildInputs = [
    alsa-lib
    fontconfig
  ];

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "rodio-0.22.2" = "sha256-ja+7gQ0m8tC+wr+2qyn/0ON2oxlfK4cYhU4HWCqzn2U=";
    };
  };

  meta = {
    description = "A music player made with Rust + Slint";
    homepage = "";
    license = lib.licenses.unlicense;
    maintainers = [ ];
  };
})
