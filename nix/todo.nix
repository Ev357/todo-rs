{
  lib,
  craneLib,
}: let
  unfilteredRoot = ../.;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources unfilteredRoot)
      (lib.fileset.maybeMissing (unfilteredRoot + "/${pname}"))
    ];
  };

  pname = "todo";
  cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
  commonArgs = {
    inherit pname src;
    version = cargoToml.workspace.package.version;

    strictDeps = true;

    cargoExtraArgs = "-p ${pname}";
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;

      meta = {
        description = "Yet another todo app.";
        homepage = "https://github.com/Ev357/todo-rs";
        platforms = lib.systems.flakeExposed;
        license = lib.licenses.mit;
        mainProgram = pname;
      };
    }
  )
