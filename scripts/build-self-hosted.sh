#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_DIR="${SELF_HOSTED_ARTIFACT_DIR:-${ROOT_DIR}/dist/self-hosted}"
FRONTEND_OUT="${ARTIFACT_DIR}/frontend"
BACKEND_OUT="${ARTIFACT_DIR}/backend"

cd "${ROOT_DIR}"

echo "Building Family Doctor Finder frontend"
(
  cd frontend
  if [[ -f package-lock.json ]]; then
    npm ci
  else
    npm install
  fi
  npm run typecheck
  npm run lint
  npm run build
)

echo "Building Family Doctor Finder backend"
cargo build --release -p backend

rm -rf "${ARTIFACT_DIR}"
mkdir -p "${FRONTEND_OUT}" "${BACKEND_OUT}"
cp -R frontend/dist/. "${FRONTEND_OUT}/"
cp target/release/backend "${BACKEND_OUT}/backend"

cat > "${ARTIFACT_DIR}/README.txt" <<'EOF'
Family Doctor Finder self-hosted artifact

frontend/ contains the static Vite build.
backend/backend is the release Axum API binary.

Run the backend with DATABASE_URL set to PostgreSQL and BIND_ADDRESS set for
your host. Serve frontend/ with a static web server and reverse proxy /api/*
and /health to the backend process.
EOF

echo "Self-hosted artifact written to ${ARTIFACT_DIR}"
