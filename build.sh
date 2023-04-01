#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
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
rm -rf lib/*.node
npx cep -c src -o lib
nr build
cd $DIR
./sh/gen.init.coffee ru
