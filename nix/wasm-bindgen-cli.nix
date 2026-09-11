{
  lib,
  craneLib,
  fetchCrate,
}: let
  cargoLock = fromTOML (builtins.readFile ../Cargo.lock);

  wasmBindgen =
    lib.findFirst
    (pkg: pkg.name == "wasm-bindgen")
    (throw "Could not find wasm-bindgen package")
    cargoLock.package;
in
  craneLib.buildPackage rec {
    pname = "wasm-bindgen-cli";
    version = wasmBindgen.version;

    src = fetchCrate {
      inherit version;

      pname = "wasm-bindgen-cli";
      hash = "sha256-a7lcXJnnZkYReja+iUO7NqqrWyv3toxnUgQb8s4IS5s=";
    };

    doCheck = false;
  }
