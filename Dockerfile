# Start with a rust base, to compile the application for deployment before building the runtime image
FROM rust:1.64.0-slim-bullseye AS builder

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends libssl-dev pkg-config build-essential; \
    rustup install stable; 

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    cargo build --release --package speedtester_server; \
    objcopy --compress-debug-sections target/release/speedtester_server ./speedtester_server

FROM debian:11.3-slim

# RUN set -eux; \
#     export DEBIAN_FRONTEND=noninteractive; \
#     apt clean autoclean; \
#     apt autoremove --yes; \
#     rm -rf /var/lib/{apt,dpkg,cache,log}/; \
#     echo "Installed base utils!"

WORKDIR /app

COPY --from=builder /app/speedtester_server ./speedtester_server

CMD ["./speedtester_server"]

