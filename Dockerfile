# Start with a rust base, to compile the application for deployment
FROM rust:alpine AS builder

# Add deps for build (everything is linked static)
RUN apk add musl-dev make pkgconfig perl

WORKDIR /src/
COPY . .

# If rustup components are to be added then this should be restored
# --mount=type=cache,target=/usr/local/rustup \

# Cache build folders, build, and then compress the debug sections
RUN --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo install --path speedtester-server; \
    objcopy --compress-debug-sections /usr/local/cargo/bin/speedtester_server ./speedtester_server; \
    mv ./speedtester_server /usr/local/cargo/bin/speedtester_server

FROM alpine

COPY --from=builder /usr/local/cargo/bin/speedtester_server /usr/local/bin/speedtester_server

CMD ["speedtester_server"]

