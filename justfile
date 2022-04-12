set dotenv-load

version := "0.0.1"

bt := '0'

export RUST_BACKTRACE := bt

alias b := build
alias s := shell

default: shell


fuzz:
  cargo +nightly fuzz run fuzz-compiler

build:
   cargo build

watch:
      cargo watch

run:
   cargo run

sloc:
  tokei

# create a nix shell with all tools needed for development
shell:
   @echo $test
   nix develop .  --command "zsh"


# install development dependencies
install-dev-deps:
  rustup install nightly
  rustup update nightly
  cargo +nightly install cargo-fuzz
  cargo install cargo-check
  cargo install cargo-limit
  cargo install cargo-watch


# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:ghishadow/buklo.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo cargo publish
  cd ../..
  rm -rf tmp/release
