# Frontend

React single-page application for the family doctor finder.

## Conventions

- `src/main.jsx` mounts the React application.
- `src/App.jsx` owns the initial route-level composition until routing is introduced.
- Future shared UI belongs in `src/components/`.
- Future API client and query hooks belong in `src/api/`.
- Future feature screens belong in `src/features/`.
- Future shared DTO shapes belong in `src/types/`.

Run locally with:

```bash
npm install
npm run dev -- --host 0.0.0.0
```

