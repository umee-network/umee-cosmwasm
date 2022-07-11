#!/bin/bash -eux

set -eo pipefail

CWD="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
BASENAME="$(basename $CWD)"
OPTIMIZER_VERSION="0.12.6"

docker run --rm -v $CWD:/code \
  --mount type=volume,source="${BASENAME}_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:$OPTIMIZER_VERSION
