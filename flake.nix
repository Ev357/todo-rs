{
  description = "todo-rs";

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
    dioxus = {
      url = "github:Ev357/dioxus/feat/query-method";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    dioxus,
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

      wasm-bindgen-cli = self.packages.${system}.wasm-bindgen-cli;
      dioxus-cli = self.packages.${system}.dioxus-cli;
    in {
      todo-server = pkgs.callPackage ./nix/todo-server.nix {inherit craneLib;};
      todo-web = pkgs.callPackage ./nix/todo-web.nix {inherit craneLib wasm-bindgen-cli dioxus-cli;};
      wasm-bindgen-cli = pkgs.callPackage ./nix/wasm-bindgen-cli.nix {inherit craneLib;};
      dioxus-cli = dioxus.packages.${system}.dioxus-cli;
    });

    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      toolchain = pkgs.callPackage ./nix/toolchain.nix {};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      wasm-bindgen-cli = self.packages.${system}.wasm-bindgen-cli;
      dioxus-cli = self.packages.${system}.dioxus-cli;
    in {
      default = pkgs.callPackage ./nix/shell.nix {inherit craneLib wasm-bindgen-cli dioxus-cli;};
    });
  };
}
