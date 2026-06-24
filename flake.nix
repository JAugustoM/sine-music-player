{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust-toolchain = inputs.fenix.packages.${system}.stable.withComponents [
            "cargo"
            "clippy"
            "rust-analyzer"
            "rust-src"
            "rustc"
            "rustfmt"
          ];
        in
        {
          packages.default = pkgs.callPackage ./default.nix { };

          devShells.default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                alsa-lib
                cmake
                dbus
                gcc
                pkg-config

                #Slint
                fontconfig
                libxkbcommon
                wayland

                libX11
                libXcursor
                libXrandr
                libXi
              ]
              ++ [ rust-toolchain ];

            LD_LIBRARY_PATH = "${
              lib.makeLibraryPath (
                with pkgs;
                [
                  alsa-lib
                  dbus
                  fontconfig

                  libGL
                  libxkbcommon
                  vulkan-loader
                  wayland
                ]
              )
            }:LD_LIBRARY_PATH";
          };
        };
    };
}
