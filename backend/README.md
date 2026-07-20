# Backend

Rust/Axum API server for the family doctor finder.

## Conventions

- `src/main.rs` owns process startup, logging, address binding, and top-level router wiring.
- Future HTTP handlers should live under `src/http/`.
- Future business logic should live under `src/services/`.
- PostgreSQL access and sqlx models live under `src/db/`.
- Migrations live under `migrations/` and are embedded in the backend binary.

The server defaults to `0.0.0.0:8080`. Override with `BIND_ADDRESS` when needed.

## REST Baseline

- `GET /health` and `GET /api/health` return JSON health status and verify PostgreSQL connectivity.
- API errors are returned as `{ "error": { "code": "...", "message": "..." } }`.
- Request validation should use the `ValidatedJson<T>` extractor plus the `ValidateRequest` trait for JSON payloads.

## Database

Set `DATABASE_URL` before running the backend. The URL must point to PostgreSQL.

```bash
export DATABASE_URL=$(cat /workspace/.database_url)
cargo run -p backend
```

The backend runs embedded sqlx migrations on startup. To run migrations manually from the repository root:

```bash
sqlx migrate run --source backend/migrations
```

## Configuration

The backend reads configuration from environment variables. See `.env.example` at the repository root for example values.

Required:

- `DATABASE_URL`

Optional with defaults:

- `BIND_ADDRESS`
- `ALLOWED_CORS_ORIGIN`
- `OSM_TILE_URL_TEMPLATE`
- `NOMINATIM_BASE_URL`
- `NOMINATIM_USER_AGENT`
- `RATE_LIMIT_WINDOW_SECS`
- `RATE_LIMIT_MAX_REQUESTS`
