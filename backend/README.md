# Backend

Rust/Axum API server for the family doctor finder.

## Conventions

- `src/main.rs` owns process startup, logging, address binding, and top-level router wiring.
- Future HTTP handlers should live under `src/http/`.
- Future business logic should live under `src/services/`.
- PostgreSQL access and sqlx models live under `src/db/`.
- Migrations live under `migrations/` and are embedded in the backend binary.

The server defaults to `0.0.0.0:8080`. Override with `BIND_ADDRESS` when needed.

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
