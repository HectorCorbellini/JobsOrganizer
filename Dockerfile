# Multi-stage Dockerfile for Organizer

# Build stage
FROM rust:1.82-slim as builder

WORKDIR /usr/src/organizer
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /usr/src/organizer
COPY --from=builder /usr/src/organizer/target/release/organizer ./organizer
ENTRYPOINT ["/usr/src/organizer/organizer"]
