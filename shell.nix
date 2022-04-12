{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Rust
    pkgs.cargo
    pkgs.rustc

    # Shells
    pkgs.zsh

    # Tools
    pkgs.cargo-audit
    pkgs.nixfmt

    # Dependencies
    pkgs.cacert
    pkgs.openssl
    pkgs.git
    pkgs.zlib
    pkgs.pkg-config
  ];
  RUST_BACKTRACE = 1;
}
