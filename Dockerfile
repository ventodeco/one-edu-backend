# Stage 1: Build
FROM rust:1.81.0 as builder
WORKDIR /usr/src/oneedubackend

# Install build dependencies
RUN apt-get update && apt-get install -y build-essential

# Copy the project files
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Final Image
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libzstd1 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /usr/src/oneedubackend/target/release/one-edu-backend .

# Ensure the binary has execution permissions
RUN chmod +x ./one-edu-backend

EXPOSE 8000
# Define the entry point
ENTRYPOINT ["./one-edu-backend"]
