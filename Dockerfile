# Use the official Rust image as a parent image
FROM rust:1.84 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached unless Cargo.toml changes)
RUN cargo build --release
RUN rm src/main.rs

# Copy the source code
COPY src ./src
COPY migrations ./migrations

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Build the actual project
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary and migrations
COPY --from=builder /usr/src/app/target/release/itcook-backend /app/
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose the port your app will run on
EXPOSE 8000

# Set environment variables
ENV RUST_LOG=debug
ENV PORT=8000

# Create a startup script with better error handling
RUN echo '#!/bin/bash\n\
echo "🚀 Starting IT Cook Backend..."\n\
echo "📊 Environment:"\n\
echo "PORT: $PORT"\n\
echo "RUST_LOG: $RUST_LOG"\n\
echo "DATABASE_URL: ${DATABASE_URL:0:50}..."\n\
echo "JWT_SECRET: ${JWT_SECRET:0:10}..."\n\
echo "🏃 Running backend..."\n\
exec ./itcook-backend\n\
' > /app/start.sh && chmod +x /app/start.sh

# Start the app with error handling
CMD ["/app/start.sh"]

