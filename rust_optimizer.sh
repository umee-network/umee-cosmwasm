#!/bin/bash -eu

set -eo pipefail

CWD="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
BASENAME="$(basename $CWD)"
OPTIMIZER_VERSION="0.12.8"
architecture=$(uname -m)

if [[ "$architecture" == "arm64" ]];then
  docker run --rm -v $CWD:/code \
    --mount type=volume,source="${BASENAME}_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer-arm64:$OPTIMIZER_VERSION
else
  docker run --rm -v $CWD:/code \
    --mount type=volume,source="${BASENAME}_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:$OPTIMIZER_VERSION
fi 

echo Finish rust optimizer