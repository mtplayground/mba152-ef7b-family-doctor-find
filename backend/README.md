# Backend

Rust/Axum API server for the family doctor finder.

## Conventions

- `src/main.rs` owns process startup, logging, address binding, and top-level router wiring.
- Future HTTP handlers should live under `src/http/`.
- Future business logic should live under `src/services/`.
- Future PostgreSQL access and sqlx models should live under `src/db/`.
- Migrations should live under `migrations/` once database tooling is introduced.

The server defaults to `0.0.0.0:8080`. Override with `BIND_ADDRESS` when needed.

