#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

cd $1
rm -rf lib
build(){
  bunx cargo-cp-artifact -nc lib/lib.node -- cargo build --features main --message-format=json-render-diagnostics
}

build || ./sh/gen.init.coffee $1 && build
bunx cep -c test -o lib
cd $DIR
./sh/gen.init.coffee $1
exec $1/lib/test.js

