#!/bin/bash -eux

set -eo pipefail

CWD="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
UMEE_VERSION="${UMEE_VERSION:-main}"
PROTO_GEN_DOCKER_TAG="${PROTO_GEN_DOCKER_TAG:-protogen-docker-tag}"


echo Script proto-gen started

docker build -t $PROTO_GEN_DOCKER_TAG --file $CWD/umee_protogen.dockerfile \
  --build-arg UMEE_VERSION=$UMEE_VERSION .



# if docker ps -a --format '{{.Names}}' | grep -Eq "^${PROTO_GEN_DOCKER_TAG}\$\$"; then
#   docker start -a $PROTO_GEN_DOCKER_TAG
# fi

# docker rm $PROTO_GEN_DOCKER_TAG

# docker run --name $PROTO_GEN_DOCKER_TAG -v $CWD/proto:/proto
# docker run --name $PROTO_GEN_DOCKER_TAG -v $CWD/proto:/proto \
#   $PROTO_GEN_DOCKER_TAG generate --path /proto/umee/leverage/v1/query.proto --template '{"version":"v1","plugins":[{"name":"rust","out":"proto"}]}'
# fi
# proto/umee/leverage/v1/query.proto
# docker run $PROTO_GEN_DOCKER_TAG generate /protogen

# cd proto
# proto_dirs=$(find ./umee -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
# for dir in $proto_dirs; do
#   for file in $(find "${dir}" -maxdepth 1 -name '*.proto'); do
#     if grep go_package $file &> /dev/null ; then
#       buf generate --template buf.gen.gogo.yaml $file
#     fi
#   done
# done

# cd ..

# after the proto files have been generated add them to the the repo
# in the proper location. Then, remove the ephemeral tree used for generation
# cp -r github.com/umee-network/umee/v2/* .
# rm -rf github.com

# go mod tidy
