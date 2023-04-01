#!/usr/bin/env bash

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

. ./sh/pid.sh

set -ex

if [ ! $cmd ];then
  cmd=run
fi

if [ $1 ];then
  project=$1
else
  project=ru
fi

RUST_BACKTRACE=1 exec watchexec \
  --shell=none \
  -w $project \
  -r --exts rs,toml,coffee,js \
  --project-origin $project \
  --ignore target/ \
  -- ./run.sh $project
