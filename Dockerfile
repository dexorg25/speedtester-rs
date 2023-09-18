# Start with a rust base, to compile the application for deployment
FROM rust:alpine AS builder

RUN apk add lld musl-dev openssl-dev

WORKDIR /src/
COPY . .

# Cache build folders, build, and then compress the debug sections
RUN --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release -p server; \
    objcopy --compress-debug-sections  target/release/server ./speedtester_server

FROM alpine

RUN apk add --no-cache iperf3

COPY --from=builder /src/speedtester_server /speedtester-server

CMD ["/speedtester-server"]

