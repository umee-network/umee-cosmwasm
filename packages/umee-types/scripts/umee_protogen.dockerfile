FROM alpine AS base-alpine
RUN apk update
RUN apk add git

FROM base-alpine AS umee-git
ARG UMEE_VERSION="main"
RUN git clone --branch="${UMEE_VERSION}" https://github.com/umee-network/umee.git

FROM bufbuild/buf AS bufbuild-rust
RUN apk add cargo
RUN cargo install protobuf-codegen
ENV PATH="/root/.cargo/bin:${PATH}"
RUN which buf

FROM bufbuild-rust
WORKDIR /proto
COPY --from=umee-git /umee/proto/ /proto/

RUN pwd
RUN ls
RUN ls /proto
RUN ls /proto/umee
RUN ls /proto/umee/leverage
RUN ls /proto/umee/leverage/v1
# RUN echo $(which buf)
# RUN buf --help
# RUN echo $PATH

# ENTRYPOINT ["/usr/local/bin/buf"]
# RUN buf generate /proto/umee/leverage/v1 --template '{"version":"v1","plugins":[{"name":"rust","out":"proto"}]}'
RUN buf generate --path umee/leverage/v1/query.proto --template '{"version":"v1","plugins":[{"name":"rust","out":"proto/leverage"}]}'
RUN buf generate --path umee/leverage/v1/query.proto --template '{"version":"v1","plugins":[{"name":"rust","out":"proto"}]}'
# RUN buf generate /proto --template '{"version":"v1","plugins":[{"name":"rust","out":"proto"}]}'
