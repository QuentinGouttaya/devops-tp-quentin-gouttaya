FROM rust:1.96.0-alpine3.24 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/task-manager-backend /app/server
COPY --from=builder /app/migrations /app/migrations
RUN mkdir -p /data
EXPOSE 3000
CMD ["./server"]
