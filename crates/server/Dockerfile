# Start with a rust base, to compile the application for deployment
FROM rust AS builder

WORKDIR /src/
COPY ../.. .

RUN apt-get update && apt-get install -y musl-tools;

RUN rustup target add x86_64-unknown-linux-musl;

# Cache build folders, build, and then compress the debug sections
RUN --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release -p server --target=x86_64-unknown-linux-musl; \
    objcopy --compress-debug-sections  target/release/server ./speedtester_server

FROM scratch

COPY --from=builder /src/speedtester_server /speedtester-server

CMD ["/speedtester-server"]

