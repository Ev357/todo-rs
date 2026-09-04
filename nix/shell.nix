{
  callPackage,
  lib,
  mkShell,
  at-spi2-core,
  cairo,
  gdk-pixbuf,
  glib,
  gtk3,
  harfbuzz,
  libsoup_3,
  openssl,
  pango,
  pkg-config,
  rust-analyzer-nightly,
  taplo,
  webkitgtk_4_1,
  xdotool,
}: let
  toolchain = callPackage ./toolchain.nix {};
  runtimeLibs = [
    at-spi2-core
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    libsoup_3
    openssl
    pango
    webkitgtk_4_1
    xdotool
  ];
in
  mkShell {
    packages =
      [
        toolchain
        rust-analyzer-nightly
        taplo
        pkg-config
      ]
      ++ runtimeLibs;

    shellHook =
      # bash
      ''
        export DATABASE_URL="sqlite://$XDG_DATA_HOME/todo/database.db"
        export PATH="$PATH:$HOME/.cargo/bin"
      '';

    env = {
      LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibs;
    };
  }
