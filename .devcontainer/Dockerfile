FROM rust:1-bullseye

# Install extras
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    rustup default nightly; \
    rustup component add rust-src clippy rustfmt; \
    cargo install sqlx-cli cargo-edit; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends lld libsctp-dev

# Fix permissions error introduced by above
RUN chown vscode:rustlang /usr/local/cargo/registry /usr/local/cargo/git
