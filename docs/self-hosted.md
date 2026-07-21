# Self-Hosted Deployment

Family Doctor Finder can be deployed as a static Vite frontend plus one Rust/Axum API binary. This phase intentionally does not add Docker or CI/CD.

## Prerequisites

- Rust stable toolchain with `cargo`
- Node.js 20 or newer with `npm`
- PostgreSQL 16 or compatible PostgreSQL server
- A static file server or reverse proxy such as Caddy, nginx, or Apache

## Build Artifacts

From the repository root:

```bash
scripts/build-self-hosted.sh
```

The script runs frontend typecheck, lint, and build, then builds the backend release binary. Artifacts are written to:

- `dist/self-hosted/frontend/` - static frontend files
- `dist/self-hosted/backend/backend` - Axum API binary

Set `SELF_HOSTED_ARTIFACT_DIR=/path/to/output` to write the artifact somewhere else.

## Backend Environment

The backend reads configuration from environment variables at process startup. Required:

```bash
DATABASE_URL=postgres://user:password@host:5432/family_doctor_find
```

Optional:

```bash
BIND_ADDRESS=0.0.0.0:8080
ALLOWED_CORS_ORIGIN=https://family-doctor-finder.example
OSM_TILE_URL_TEMPLATE=https://tile.openstreetmap.org/{z}/{x}/{y}.png
NOMINATIM_BASE_URL=https://nominatim.openstreetmap.org
NOMINATIM_USER_AGENT=family-doctor-finder/0.1
RATE_LIMIT_WINDOW_SECS=60
RATE_LIMIT_MAX_REQUESTS=30
REPORT_REPEAT_WINDOW_SECS=3600
REPORT_REPEAT_MAX_REQUESTS=1
```

Use a PostgreSQL URL only. Do not use SQLite, in-memory storage, JSON files, or local volumes for persistent application state.

The backend runs embedded sqlx migrations on startup. Manual migration is optional:

```bash
sqlx migrate run --source backend/migrations
```

## Frontend Environment

For same-origin hosting, leave `VITE_API_BASE_URL` unset before building. The frontend will call `/api/...` on the same origin.

For separate frontend and backend origins, set both values before building/running:

```bash
export VITE_API_BASE_URL=https://api.family-doctor-finder.example
export ALLOWED_CORS_ORIGIN=https://family-doctor-finder.example
scripts/build-self-hosted.sh
```

## Run The Backend

Example:

```bash
export DATABASE_URL=postgres://user:password@host:5432/family_doctor_find
export BIND_ADDRESS=127.0.0.1:8080
dist/self-hosted/backend/backend
```

Health checks:

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/api/health
```

## Same-Origin Reverse Proxy

Serve the static frontend and proxy API routes to the backend. Example Caddyfile:

```caddyfile
family-doctor-finder.example {
	root * /srv/family-doctor-finder/frontend

	handle /api/* {
		reverse_proxy 127.0.0.1:8080
	}

	handle /health {
		reverse_proxy 127.0.0.1:8080
	}

	handle {
		try_files {path} /index.html
		file_server
	}
}
```

Example nginx server block:

```nginx
server {
    listen 80;
    server_name family-doctor-finder.example;

    root /srv/family-doctor-finder/frontend;
    index index.html;

    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location = /health {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
    }

    location / {
        try_files $uri /index.html;
    }
}
```

## Deployment Checklist

1. Build artifacts with `scripts/build-self-hosted.sh`.
2. Copy `dist/self-hosted/frontend/` to the static web root.
3. Copy `dist/self-hosted/backend/backend` to the application host.
4. Set `DATABASE_URL` and any optional runtime variables in the process manager.
5. Start the backend process and verify `/health`.
6. Serve the frontend with SPA fallback and proxy `/api/*` plus `/health` to the backend.
7. Verify city search, doctor results, detail pages, and report submission controls from the public URL.
