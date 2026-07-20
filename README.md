# Family Doctor Finder

A public-service directory for finding family doctors by Canadian city or area, checking whether listings are accepting new patients, and collecting anonymous availability reports.

## Repository Layout

- `frontend/` - React single-page application built with Vite.
- `backend/` - Rust/Axum API server.
- `.plan` - Architecture and issue sequencing notes for the full implementation.

## Local Development

The backend is the runtime entry point and listens on `0.0.0.0:8080` by default.

```bash
cargo build
cargo clippy --workspace --all-targets
cargo run -p backend
```

The frontend can be installed and built independently:

```bash
cd frontend
npm install
npm run typecheck
npm run lint
npm run format:check
npm run build
```

## Directory Conventions

Backend code should keep HTTP routing, service logic, data access, and configuration in separate modules as the implementation grows. Frontend code should organize reusable UI separately from feature-specific screens and API/query code.

Persistent application state must use PostgreSQL through the backend; do not add browser-only or file-backed persistence for server-owned data.
