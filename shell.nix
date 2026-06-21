{ pkgs, lib, ... }:
pkgs.mkShell {
  packages = with pkgs; [
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
  ];

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
}
