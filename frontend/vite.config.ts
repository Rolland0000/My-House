import { existsSync, statSync, createReadStream } from "node:fs";
import path from "node:path";
import { defineConfig, loadEnv, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// Same two prefixes as prod nginx's media aliases; `owner-requests/*` is never served.
const PUBLIC_MEDIA_PREFIXES = ["avatars", "listings"];
const MEDIA_CONTENT_TYPES: Record<string, string> = {
  jpg: "image/jpeg",
  jpeg: "image/jpeg",
  png: "image/png",
  webp: "image/webp",
};

// Dev-server stand-in for nginx's `/media/` aliases; `apply: "serve"` keeps it out of the build.
function serveLocalMedia(storageRoot: string): Plugin {
  return {
    name: "serve-local-media",
    apply: "serve",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith("/media/")) {
          next();
          return;
        }

        let requested: string;
        try {
          requested = decodeURIComponent(req.url.slice("/media/".length).split("?")[0]);
        } catch {
          res.statusCode = 400;
          res.end();
          return;
        }
        const segments = requested.split("/").filter(Boolean);

        if (!PUBLIC_MEDIA_PREFIXES.includes(segments[0])) {
          next();
          return;
        }

        // Answered here rather than via next(), which would mask a bad path as the SPA's 200.
        if (segments.includes("..")) {
          res.statusCode = 403;
          res.end();
          return;
        }

        const resolvedRoot = path.resolve(storageRoot);
        const filePath = path.resolve(resolvedRoot, ...segments);
        if (
          !filePath.startsWith(resolvedRoot + path.sep) ||
          !existsSync(filePath) ||
          !statSync(filePath).isFile()
        ) {
          res.statusCode = 404;
          res.end();
          return;
        }

        const ext = path.extname(filePath).slice(1).toLowerCase();
        res.setHeader("Content-Type", MEDIA_CONTENT_TYPES[ext] ?? "application/octet-stream");
        // An unhandled stream 'error' event would crash the dev server.
        createReadStream(filePath)
          .on("error", () => {
            res.statusCode = 500;
            res.end();
          })
          .pipe(res);
      });
    },
  };
}

export default defineConfig(({ mode }) => {
  // Same API_HOST/APP_PORT vars the `generate:types` script already reads
  // (see .env.example) — inside docker-compose they resolve to the backend
  // service name, on the host machine they default to localhost. Proxying
  // here mirrors prod nginx's `location /api/` and `/health` blocks, so the
  // browser only ever talks to one origin and the backend needs no CORS
  // layer for dev.
  const env = loadEnv(mode, ".", "");
  const backendTarget = `http://${env.API_HOST ?? "localhost"}:${env.APP_PORT ?? "3000"}`;
  // Default matches a bare `cargo run`'s LOCAL_STORAGE_PATH (./storage under backend/).
  const mediaRoot = env.LOCAL_MEDIA_PATH ?? path.resolve(import.meta.dirname, "../backend/storage");

  return {
    plugins: [react(), tailwindcss(), serveLocalMedia(mediaRoot)],
    server: {
      proxy: {
        "/api": backendTarget,
        "/health": backendTarget,
      },
    },
  };
});
