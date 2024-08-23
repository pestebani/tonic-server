FROM rust:1.80-slim AS builder

COPY . /home/myapp

WORKDIR /home/myapp

RUN apt update  && apt-get install -y libssl-dev pkg-config musl-dev musl-tools libudev-dev perl build-essential checkinstall zlib1g-dev protobuf-compiler && \
    export RUST_BACKTRACE=full && export OPENSSL_LIB_DIR=/usr && export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig && export RUSTFLAGS='-C target-feature=+crt-static' && rustup target add x86_64-unknown-linux-musl && cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

COPY --from=builder /home/myapp/target/x86_64-unknown-linux-musl/release/tonic-server /sbin/tonic-server
EXPOSE 50051
ENTRYPOINT ["tonic-server"]
CMD ["tonic-server"]
