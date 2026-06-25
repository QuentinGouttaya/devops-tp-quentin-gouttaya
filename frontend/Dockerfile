# 1. Builder : Node.js pour compiler Svelte via Vite
FROM node:26.4.0-alpine3.23 AS builder
WORKDIR /app

# Copie des manifestes pour optimiser le cache Docker
COPY package.json package-lock.json ./
RUN npm ci

# Copie du reste des fichiers (incluant src, public, index.html, etc.)
COPY . .
RUN npm run build

# 2. Runtime : NGINX pour servir les fichiers générés (qui seront dans /app/dist)
FROM nginx:alpine

# Remplacement de la configuration par défaut par la vôtre
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
