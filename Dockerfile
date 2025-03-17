# Use the official Rust image to build the application
FROM rust:latest AS builder

# Set the working directory to the "api" subfolder (adjust as needed)
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock first to cache dependencies
COPY api/Cargo.toml api/Cargo.lock ./

# Set up the application structure (copy the entire api folder)
COPY api ./api

# Set the working directory to the "api" folder where the Rust project resides
WORKDIR /app/api

# Build the release version of the application
RUN cargo build --release

# Create a smaller image based on a minimal Linux distribution
FROM debian:bullseye-slim

# Install the necessary dependencies for the Rocket application (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    openssl \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory for the app
WORKDIR /app

# Copy the compiled Rust application from the builder stage
COPY --from=builder /app/api/target/release/tasks .

# Expose the port that Rocket will run on (typically 8000)
EXPOSE 8000

# Run the Rocket application when the container starts
CMD ["./tasks"]