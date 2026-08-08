import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(({ mode }) => {
  // Same API_HOST/APP_PORT vars the `generate:types` script already reads
  // (see .env.example) — inside docker-compose they resolve to the backend
  // service name, on the host machine they default to localhost. Proxying
  // here mirrors prod nginx's `location /api/` and `/health` blocks, so the
  // browser only ever talks to one origin and the backend needs no CORS
  // layer for dev. `loadEnv` (rather than reading `process.env` directly)
  // avoids pulling in @types/node just for this one config file.
  const env = loadEnv(mode, ".", "");
  const backendTarget = `http://${env.API_HOST ?? "localhost"}:${env.APP_PORT ?? "3000"}`;

  return {
    plugins: [react(), tailwindcss()],
    server: {
      proxy: {
        "/api": backendTarget,
        "/health": backendTarget,
      },
    },
  };
});
