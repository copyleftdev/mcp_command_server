# Stage 1: Build the application
FROM rust:1.84.1-slim-bookworm as builder

# Create a non-root user
RUN useradd -m -u 1000 -U -s /bin/bash rustacean

# Create app directory and set ownership
WORKDIR /app
COPY --chown=rustacean:rustacean . .

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the application
USER rustacean
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Create a non-root user
RUN useradd -m -u 1000 -U -s /bin/bash rustacean

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set up application directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder --chown=rustacean:rustacean /app/target/release/mcp_command_server /app/mcp_command_server

# Copy the .context file from the builder stage
COPY --from=builder --chown=rustacean:rustacean /app/.context /app/.context

# Copy the exclude.yaml file from the builder stage
COPY --from=builder --chown=rustacean:rustacean /app/exclude.yaml /app/exclude.yaml

# Use non-root user for running the application
USER rustacean

# Expose the default port
EXPOSE 3030

# Run the application
CMD ["/app/mcp_command_server"]
