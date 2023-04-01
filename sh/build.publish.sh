#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}

cd $DIR

set -ex

init() {
  if [ ! -d $DIR/$1/node_modules ]; then
    if ! [ -x "$(command -v pnpm)" ]; then
      npm install -g pnpm
    fi
    cd $DIR/$1
    pnpm i
    cd $DIR
  fi
}

export PATH=$DIR/.direnv/bin:$PATH

init .
init ru
./sh/gen.init.coffee misc
./sh/gen.init.coffee redis

cd ru

if ! [ -x "$(command -v cross)" ]; then
  cargo install cross
fi

dist() {
  rm -rf lib
  mkdir -p lib
  mv $DIR/target/$1/release/libru.$ext lib/I.node
  ./dist.coffee $1
}

cross_build() {
  cross build --release --target $1
  dist $1
}

rust_build() {
  rustup target add $1
  cargo build --release --target $1
  dist $1
}

case "$OSTYPE" in
  darwin*)
    ext=dylib
    rust_build aarch64-apple-darwin
    rust_build x86_64-apple-darwin
    ;;
  linux*)
    ext=so
    cross_build aarch64-unknown-linux-gnu
    cross_build x86_64-unknown-linux-gnu

  # https://github.com/rust-lang/cargo/issues/8607 musl 目标上的 cdylib crate 类型支持
  export RUSTFLAGS="-C target-feature=-crt-static"
  cross_build aarch64-unknown-linux-musl
  cross_build x86_64-unknown-linux-musl
  ;;
msys*)
  ext=dll
  pnpm run build
  ./dist.coffee
  ;;
esac
