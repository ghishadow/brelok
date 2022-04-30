{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    buildInputs = [
      # Rust
      pkgs.rustup

      # Shells
      pkgs.zsh

      # Dependencies
      pkgs.cacert
      pkgs.openssl
      pkgs.git
      pkgs.zlib
      pkgs.pkg-config
    ];
    RUST_BACKTRACE = 1;
  }
