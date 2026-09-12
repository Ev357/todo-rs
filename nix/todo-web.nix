{
  lib,
  craneLib,
  dioxus-cli,
  wasm-bindgen-cli,
  binaryen,
  esbuild,
  makeWrapper,
}: let
  unfilteredRoot = ../.;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources unfilteredRoot)
      (lib.fileset.maybeMissing (unfilteredRoot + "/${pname}"))
    ];
  };

  pname = "todo-web";
  cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
  commonArgs = {
    inherit pname src;
    version = cargoToml.workspace.package.version;

    strictDeps = true;
    doCheck = false;
    doNotPostBuildInstallCargoBinaries = true;

    nativeBuildInputs = [
      dioxus-cli
      wasm-bindgen-cli
      binaryen
      esbuild
      makeWrapper
    ];

    cargoExtraArgs = "-p ${pname} --target wasm32-unknown-unknown";
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLib.buildPackage (
    commonArgs
    // rec {
      inherit cargoArtifacts;

      buildPhaseCargoCommand =
        # bash
        ''
          dx bundle --platform web --release --package ${pname}
        '';

      installPhase =
        # bash
        ''
          runHook preInstall

          mkdir -p $out/bin $out/share/${pname}

          cp -r target/dx/${pname}/release/web/public $out/share/${pname}/

          cp target/dx/${pname}/release/web/server $out/bin/${meta.mainProgram}

          wrapProgram $out/bin/${meta.mainProgram} \
            --set-default DIOXUS_PUBLIC_PATH "$out/share/${pname}/public"

          runHook postInstall
        '';

      meta = {
        description = "Web for todo-rs";
        homepage = "https://github.com/Ev357/todo-rs";
        platforms = lib.systems.flakeExposed;
        license = lib.licenses.mit;
        mainProgram = "todo-web";
      };
    }
  )
