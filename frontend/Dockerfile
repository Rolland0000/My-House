# syntax=docker/dockerfile:1

# ── Builder ────
FROM node:22-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

# `src/shared/api/types.ts` is generated from the live backend's OpenAPI
# schema (`npm run generate:types`) and is gitignored — it must already exist
# on disk in the build context before `docker build` runs. This stage does
# not regenerate it: doing so would require network access to a live backend
# from inside an isolated build, which the CI pipeline provides as a separate
# step ahead of the image build instead.
COPY . .
RUN npm run build

# ── Runtime ──────
FROM nginx:stable-alpine AS runtime

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=5 \
    CMD wget -q --spider http://localhost/ || exit 1

# ── Dev ────────
# Toolchain only — no source is copied in. docker-compose bind-mounts the
# frontend/ directory over /app in the dev profile; node_modules is kept in a
# named volume so the host bind mount doesn't shadow container-installed
# deps. Vite runs with HMR, bound to 0.0.0.0 so it's reachable from the host.
FROM node:22-alpine AS dev

WORKDIR /app

EXPOSE 5173

HEALTHCHECK --interval=10s --timeout=3s --start-period=15s --retries=5 \
    CMD wget -q --spider http://localhost:5173/ || exit 1

CMD ["sh", "-c", "npm install && npm run dev -- --host 0.0.0.0 --port 5173"]
