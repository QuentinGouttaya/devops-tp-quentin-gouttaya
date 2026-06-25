FROM rust:1.96.0-slim-bullseye AS builder

WORKDIR /app


COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/app/target \
  # On compile et on copie immédiatement le binaire hors du dossier cache
  # car le dossier /app/target n'est pas persistant entre les stages de Docker
  cargo build --release && \
  cp target/release/task-manager-backend /app/server


FROM debian:bullseye-slim

WORKDIR /app

# Copie du binaire depuis le builder (vers la racine du WORKDIR)
COPY --from=builder /app/server /app/server

# Copie du dossier de migrations (pour SQLx ou équivalent)
COPY --from=builder /app/migrations /app/migrations

# Création du dossier data
RUN mkdir -p /data

# Variables d'environnement recommandées
ENV RUST_LOG="info"
ENV RUST_BACKTRACE=1

EXPOSE 3000

CMD ["./server"]
