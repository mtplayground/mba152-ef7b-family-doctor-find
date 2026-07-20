# Frontend

React single-page application for the family doctor finder.

## Conventions

- `src/main.tsx` mounts the React application.
- `src/App.tsx` owns the initial route-level composition until routing is introduced.
- Future shared UI belongs in `src/components/`.
- Future API client and query hooks belong in `src/api/`.
- Future feature screens belong in `src/features/`.
- Future shared DTO shapes belong in `src/types/`.

Run locally with:

```bash
npm install
npm run typecheck
npm run lint
npm run format:check
npm run dev -- --host 0.0.0.0
```
