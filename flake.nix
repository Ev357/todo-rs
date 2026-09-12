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
        overlays = [
          self.overlays.default
          fenix.overlays.default
        ];
      };

      toolchain = pkgs.callPackage ./nix/toolchain.nix {};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
    in {
      todo-server = pkgs.callPackage ./nix/todo-server.nix {inherit craneLib;};
      todo-web = pkgs.callPackage ./nix/todo-web.nix {inherit craneLib;};
      todo = pkgs.callPackage ./nix/todo.nix {inherit craneLib;};
      wasm-bindgen-cli = pkgs.callPackage ./nix/wasm-bindgen-cli.nix {inherit craneLib;};
      dioxus-cli = dioxus.packages.${system}.dioxus-cli;
      default = self.packages.${system}.todo;
    });

    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          self.overlays.default
          fenix.overlays.default
        ];
      };

      toolchain = pkgs.callPackage ./nix/toolchain.nix {};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
    in {
      default = pkgs.callPackage ./nix/shell.nix {inherit craneLib;};
    });

    overlays = {
      todo = final: _: let
        system = final.stdenv.hostPlatform.system;
      in {
        todo-server = self.packages.${system}.todo-server;
        todo-web = self.packages.${system}.todo-web;
        todo = self.packages.${system}.todo;
        wasm-bindgen-cli = self.packages.${system}.wasm-bindgen-cli;
        dioxus-cli = self.packages.${system}.dioxus-cli;
      };
      default = self.overlays.todo;
    };

    homeModules = {
      todo = {
        imports = [./nix/home-manager.nix];
        nixpkgs.overlays = [self.overlays.default];
      };
      default = self.homeModules.todo;
    };

    nixosModules = {
      todo = {
        imports = [./nix/nixos];
        nixpkgs.overlays = [self.overlays.default];
      };
      default = self.nixosModules.todo;
    };

    nixosConfigurations.container = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        self.nixosModules.default
        ./nix/container.nix
      ];
    };
  };
}
