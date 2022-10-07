# Start with a rust base, to compile the application for deployment
FROM rust:alpine AS builder

# Add deps for build (everything is linked static so these can be installed in builder only)
# But we still need openssl-dev for headers. the dynamic libs 
RUN apk add openssl-dev pkgconfig musl-dev make lksctp-tools-static

WORKDIR /src/
COPY . .

# If rustup components are to be added then this should be restored
# --mount=type=cache,target=/usr/local/rustup \

# Cache build folders, build, and then compress the debug sections
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo install --path speedtester-server --target x86_64-unknown-linux-musl; \
    objcopy --compress-debug-sections /usr/local/cargo/bin/speedtester_server ./speedtester_server; \
    mv ./speedtester_server /usr/local/cargo/bin/speedtester_server

FROM scratch

COPY --from=builder /usr/local/cargo/bin/speedtester_server /speedtester_server

CMD ["/speedtester_server"]

