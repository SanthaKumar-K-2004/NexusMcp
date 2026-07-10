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

WORKDIR /app

# Install minimal dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/nexusmcp /usr/local/bin/nexusmcp

# Create data directory for SQLite
RUN mkdir -p /data

ENV NEXUS_DB_PATH=/data/nexusmcp.db

ENTRYPOINT ["nexusmcp"]
CMD ["mcp", "--stealth"]