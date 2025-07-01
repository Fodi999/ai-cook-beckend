# Use the official Rust image as a parent image
FROM rust:1.75 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml file
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached unless Cargo.toml changes)
RUN cargo build --release
RUN rm src/main.rs

# Copy the source code
COPY src ./src
COPY migrations ./migrations

# Build the application with SQLX offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bookworm-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/itcook-backend /app/

# Copy migrations
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose the port that the app runs on
EXPOSE 8000

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8000

# Command to run the application
CMD ["./itcook-backend"]
