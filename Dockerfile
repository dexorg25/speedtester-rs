# Start with a rust base, to compile the application for deployment
FROM rust:alpine AS builder

RUN apk add lld musl-dev openssl-dev

WORKDIR /src/
COPY . .

# If rustup components are to be added then this should be restored
# --mount=type=cache,target=/usr/local/rustup \

# Cache build folders, build, and then compress the debug sections
RUN --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections  target/release/server ./speedtester_server; \
    mv ./speedtester_server /usr/local/cargo/bin/speedtester-server

FROM alpine

RUN apk add --no-cache iperf3

COPY --from=builder /usr/local/cargo/bin/speedtester-server /speedtester-server

CMD ["/speedtester-server"]

