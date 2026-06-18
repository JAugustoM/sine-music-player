{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = with pkgs; [
    alsa-lib
    cmake
    dbus
    gcc

    #Slint
    fontconfig
    libxkbcommon
    wayland

    libX11
    libXcursor
    libXrandr
    libXi
  ];

  env.LD_LIBRARY_PATH = "${
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
  }:$LD_LIBRARY_PATH";
}
