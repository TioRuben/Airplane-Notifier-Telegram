# Use the official Rust image as a base
FROM rust:1-buster

# Install dependencies for cross-compilation
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    && rm -rf /var/lib/apt/lists/*

# Set the default target to ARM meson64
ENV CARGO_TARGET_ARMV8_UNKNOWN_LINUX_GNUEABIHF_LINKER=aarch64-linux-gnu-gcc

# Set the working directory
WORKDIR /workspace

# Copy the current project to the container
COPY . .

# Build the project
RUN cargo build --target armv8-unknown-linux-gnueabihf
