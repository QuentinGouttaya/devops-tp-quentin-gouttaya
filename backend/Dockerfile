# ---------------------------------------------------
# Étape 1 : Builder
# ---------------------------------------------------
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

# ---------------------------------------------------
# Étape 2 : Runtime Distroless (Sécurisé) PAS DE SHELL ni PACKAGE MANAGER
# ---------------------------------------------------
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
