# Use an official Rust image as the base, specifying the amd64 platform
FROM --platform=linux/amd64 rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/chat-app

# Copy the Cargo files (Cargo.toml and Cargo.lock)
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the project's source code
COPY src ./src

# Build the release version of the application for the amd64 platform
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Use a smaller base image for the final image, specifying the amd64 platform
FROM --platform=linux/amd64 debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user and switch to it
RUN useradd -m -u 1000 appuser
USER appuser

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/chat-app/target/x86_64-unknown-linux-gnu/release/chat-app .

# Expose the port the application will listen on
EXPOSE 8080

CMD ["./chat-app"]
