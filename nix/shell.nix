{
  pkgs,
  inputs,
  ...
}:
pkgs.mkShell {
  packages = with pkgs; [
    inputs.fenix.packages.${stdenv.hostPlatform.system}.default.toolchain
    rust-analyzer-nightly
    dioxus-cli
  ];
}
