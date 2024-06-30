FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-gnu 
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools
WORKDIR /app


FROM chef AS planner
COPY ./web_server /app/web_server
COPY ./primitypes /app/primitypes
WORKDIR /app/web_server
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/web_server/recipe.json /app/web_server/recipe.json
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools

WORKDIR /app/web_server
RUN rustup target add x86_64-unknown-linux-gnu 
COPY ./primitypes /app/primitypes
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json
COPY ./web_server /app/web_server
RUN cargo build --release --target x86_64-unknown-linux-gnu --bin web_server

FROM bitnami/minideb:latest as end
COPY --from=builder /app/web_server/target/x86_64-unknown-linux-gnu/release/web_server /web_server
ENTRYPOINT ["/web_server"]
EXPOSE 8000
