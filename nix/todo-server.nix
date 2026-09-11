{
  lib,
  craneLib,
  ...
}: let
  unfilteredRoot = ../.;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources unfilteredRoot)
      (unfilteredRoot + "/todo-server/.sqlx")
      (unfilteredRoot + "/todo-server/migrations")
    ];
  };
in
  craneLib.buildPackage {
    pname = "todo-server";
    version = "0.1.0";
    inherit src;

    cargoExtraArgs = "-p todo-server";

    meta = {
      description = "Server for todo-rs";
      homepage = "https://github.com/Ev357/todo-rs";
      platforms = lib.systems.flakeExposed;
      license = lib.licenses.mit;
      mainProgram = "todo-server";
    };
  }
