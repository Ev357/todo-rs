{
  lib,
  pkgs,
  craneLib,
}: let
  src = pkgs.lib.cleanSource ../.;

  pname = "todo-server";
  cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
  commonArgs = {
    inherit pname;
    version = cargoToml.workspace.package.version;

    strictDeps = true;

    cargoExtraArgs = "-p ${pname}";
  };

  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      src = craneLib.cleanCargoSource src;
    }
  );
in
  craneLib.buildPackage (
    commonArgs
    // {
      inherit src cargoArtifacts;

      meta = {
        description = "Server for todo-rs";
        homepage = "https://github.com/Ev357/todo-rs";
        platforms = lib.systems.flakeExposed;
        license = lib.licenses.mit;
        mainProgram = pname;
      };
    }
  )
