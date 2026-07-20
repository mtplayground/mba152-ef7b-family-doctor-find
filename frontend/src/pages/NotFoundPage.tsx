import { Link } from 'react-router-dom';

export function NotFoundPage() {
  return (
    <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
      <p className="mb-3 text-sm font-semibold uppercase text-service-red">
        Not found
      </p>
      <h1 className="text-3xl font-semibold text-ink-900">
        This page is not in the directory.
      </h1>
      <Link
        to="/"
        className="mt-6 inline-flex rounded-control bg-civic-700 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-civic-600 focus:outline-none focus-visible:shadow-focus"
      >
        Return to search
      </Link>
    </section>
  );
}
