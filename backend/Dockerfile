
FROM rust:1.96.0-alpine3.24 AS builder

WORKDIR /app


COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/app/target \
  cargo build --release && \
  cp target/release/task_manager_backend /app/server

# On prépare le dossier /data pour qu'il appartienne à l'utilisateur non-root de distroless
RUN mkdir -p /app/data && chown -R 65532:65532 /app/data

# L'image `cc` contient le strict minimum pour les binaires C dynamiques
FROM gcr.io/distroless/cc-debian13:nonroot

WORKDIR /app

# On copie le binaire
COPY --from=builder /app/server /app/server

# On copie le dossier data avec les droits du user nonroot
COPY --from=builder --chown=nonroot:nonroot /app/data /data

ENV RUST_LOG="info"
ENV RUST_BACKTRACE=1

EXPOSE 3000
USER nonroot

CMD ["/app/server"]
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
