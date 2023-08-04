## We use the latest Rust stable release as base image
#FROM rust:1.71.0-slim as chef
##ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
##FROM ${BASE_IMAGE} AS chef
##FROM rust:1.71.0-alpine as chef
##FROM clux/muslrust:stable AS chef
#
#RUN apt-get update \
#    && apt-get install -y musl-tools 
#    
##RUN rustup target add x86_64-unknown-linux-musl
#
##RUN rustup target add x86_64-unknown-linux-musl
##RUN apk update && apk upgrade && apk  --no-cache add musl-dev
#RUN cargo install cargo-chef
#
#WORKDIR /app
#
#
#FROM chef AS planner
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM chef AS builder
#COPY --from=planner /app/recipe.json recipe.json
#
#RUN apt update \
#    && apt-get install -y lld clang pkg-config -y\
#    && apt-get install -y openssl ca-certificates libssl-dev -y
##RUN apt-get install musl-tools llvm musl-dev -y
##ENV CC_aarch64_unknown_linux_musl=clang
##ENV AR_aarch64_unknown_linux_musl=llvm-ar
##ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
##RUN apt-get update \
##    && apt-get install -y musl-tools \
##    && rustup target add x86_64-unknown-linux-musl
##RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json --bin web_server
#
#COPY . .
##ADD --chown=rust:rust . ./
#ENV SQLX_OFFLINE true
#RUN cargo build --release --bin web_server
##RUN cargo build --target x86_64-unknown-linux-musl --release --bin web_server
#
## When `docker run` is executed, launch the binary!
#
##FROM scratch
##WORKDIR /app
##COPY --from=builder /app/target/release/web_server web_server
##COPY configuration configuration
##CMD ["/app/web_server"]
##EXPOSE 8000
#
##FROM alpine:latest AS runtime
#FROM debian:bullseye-slim AS runtime
##RUN addgroup -S myuser && adduser -S myuser -G myuser
#COPY --from=builder \
#    /app/target/release/web_server \
#    /usr/local/bin/
##USER myuser
#RUN apt-get update -y \
#    && apt-get install -y --no-install-recommends openssl ca-certificates \
#    && apt-get install -y lld clang pkg-config -y\
#    && apt-get install -y ca-certificates libssl-dev \
#    # Clean up
#    && apt-get autoremove -y \
#    && apt-get clean -y \
#    && rm -rf /var/lib/apt/lists/*
#COPY configuration configuration
#ENTRYPOINT ["/usr/local/bin/web_server"]

#FROM ekidd/rust-musl-builder:latest AS builder-chef

#FROM clux/muslrust:stable AS builder-chef
#
#USER root
#ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
#RUN cargo install cargo-chef
#G
#FROM builder-chef AS planner
#WORKDIR /app
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM builder-chef AS builder
#WORKDIR /app
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
#ENV SQLX_OFFLINE true
#COPY . .
#RUN cargo build --release --target x86_64-unknown-linux-musl \
#    && strip /app/target/x86_64-unknown-linux-musl/release/web_server
#
#FROM alpine:3.13 AS runtime
#RUN addgroup -S zero2prod && adduser -S zero2prod -G zero2prod
#COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/web_server /usr/local/bin/
#COPY configuration configuration
#ENV APP_ENVIRONMENT production
#USER zero2prod
#CMD ["/usr/local/bin/web_server"]

#FROM rust:1.71.0-slim 
#
#ARG APP_NAME="web_server"
#ARG TARGET="aarch64-unknown-linux-musl"
#ARG GITHUB_SSH_KEY=""
#RUN apt-get update
#RUN apt-get install clang llvm -y
##RUN rustup target add $TARGET
#
#RUN rustup target add aarch64-unknown-linux-musl
##RUN mkdir /usr/src/$APP_NAME
##WORKDIR /usr/src/$APP_NAME
#WORKDIR /app
#
#ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
#
#ENV CC_aarch64_unknown_linux_musl=clang
#ENV AR_aarch64_unknown_linux_musl=llvm-ar
#ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
#ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"
#
#COPY . .
##COPY ./src ./src
#
##RUN mkdir /root/.ssh/
##RUN echo "$GITHUB_SSH_KEY" > /root/.ssh/id_rsa;
##RUN chmod 400 /root/.ssh/id_rsa
##RUN ssh-keyscan -H github.com >> /etc/ssh/ssh_known_hosts
#
#RUN cargo build --release --target=aarch64-unknown-linux-musl
#
#RUN groupadd -g 10001 -r $APP_NAME
#RUN useradd -r -g $APP_NAME -u 10001 $APP_NAME
#
## ------------------------------------------------------------------------------
#
#FROM scratch
#ARG APP_NAME="app"
#ARG TARGET="aarch64-unknown-linux-musl"
##WORKDIR /user/local/bin/
##COPY --from=0 /etc/passwd /etc/passwd
#COPY --from=0 /app/target/$TARGET/release/$APP_NAME ./app
#USER $APP_NAME
#
#CMD ["./app"]


FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools
WORKDIR /app


FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl


# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/web_server /web_server
COPY configuration configuration
ENTRYPOINT ["/web_server"]
EXPOSE 8000
