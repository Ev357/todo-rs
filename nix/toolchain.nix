{fenix}:
fenix.combine [
  fenix.default.cargo
  fenix.default.clippy
  fenix.default.rustc
  fenix.default.rustfmt
  fenix.default.rust-std
  fenix.complete.rust-src
  fenix.targets.wasm32-unknown-unknown.latest.rust-std
]
