FROM rust:1.76-slim-bookworm

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    postgresql-client \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Make build script executable
RUN chmod +x build.sh

# Set environment variables
ENV RUST_LOG=info

# Expose port
EXPOSE 8000

# Run build script
CMD ["./build.sh"]