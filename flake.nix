{
  description = "todo";

  nixConfig = {
    extra-substituters = [
      "https://fenix.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "fenix.cachix.org-1:ecJhr+RdYEdcVgUkjruiYhjbBloIEGov7bos90cZi0Q="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    ...
  }: let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    packages = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      toolchain = pkgs.callPackage ./nix/toolchain.nix {};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
    in {
      todo-server = pkgs.callPackage ./nix/todo-server.nix {inherit craneLib;};
    });

    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      toolchain = pkgs.callPackage ./nix/toolchain.nix {};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
    in {
      default = pkgs.callPackage ./nix/shell.nix {inherit craneLib;};
    });
  };
}
