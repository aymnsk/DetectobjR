FROM rust:1.60 as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && \
    apt-get install -y cmake libclang-dev clang pkg-config libopencv-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libopencv-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/opencv-rust-detection /usr/local/bin/opencv-rust-detection
COPY models /app/models
COPY static /app/static

WORKDIR /app
ENV RUST_LOG=info

EXPOSE 8080

CMD ["opencv-rust-detection", "--web"]
