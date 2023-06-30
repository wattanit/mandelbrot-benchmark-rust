FROM rust:1.66 as builder
WORKDIR /usr/src/mandelbrot-benchmark-rust
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mandelbrot-benchmark-rust /usr/local/bin/mandelbrot-benchmark-rust
ENTRYPOINT ["mandelbrot-benchmark-rust"]
