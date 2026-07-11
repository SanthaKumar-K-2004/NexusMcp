# NexusMCP - Multi-stage Docker Build
FROM rust:1.80-bookworm as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install Chromium and dependency libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
    chromium \
    ca-certificates \
    fonts-liberation \
    libnss3 \
    libnspr4 \
    && rm -rf /var/lib/apt/lists/*

# Set Chrome Path for headless_chrome
ENV CHROME_PATH=/usr/bin/chromium
ENV NO_SANDBOX=1
ENV NEXUS_NO_SANDBOX=1
ENV NEXUS_DB_PATH=/data/nexusmcp_profiles.db

WORKDIR /app

# Create data directory for SQLite profile storage and set permissions for non-root execution
RUN mkdir -p /data && chown -R nobody:nogroup /data

# Copy binary
COPY --from=builder /app/target/release/nexusmcp /usr/local/bin/nexusmcp

# Run as non-root user
USER nobody

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/nexusmcp"]
CMD ["mcp", "--stealth"]