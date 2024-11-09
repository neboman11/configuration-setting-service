# 1. This tells docker to use the Rust official image
FROM rust:1.80

# 2. Copy the files in your machine to the Docker image
WORKDIR /app
COPY src ./src
COPY migrations ./migrations
COPY Cargo.toml ./
COPY Cargo.lock ./

# Build your program for release
RUN cargo build --release

VOLUME /data

EXPOSE 8084

# Run the binary
CMD ["./target/release/configuration-setting-service"]
