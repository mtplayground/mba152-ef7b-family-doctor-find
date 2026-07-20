const statusRows = [
  ['Cities', 'Search entry'],
  ['Listings', 'Clinic and doctor records'],
  ['Reports', 'Community availability updates'],
];

export function HomePage() {
  return (
    <div className="grid gap-8 lg:grid-cols-[minmax(0,1fr)_360px]">
      <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
        <p className="mb-3 text-sm font-semibold uppercase text-civic-700">
          Public-service directory
        </p>
        <h1 className="max-w-3xl text-4xl font-semibold leading-tight text-ink-900 sm:text-5xl">
          Family doctor availability, organized by city.
        </h1>
        <p className="mt-5 max-w-2xl text-lg text-ink-700">
          A civic directory for finding family doctors and clinics, with recent
          accepting-status reports surfaced where they matter most.
        </p>
      </section>

      <aside className="rounded-lg border border-civic-100 bg-civic-50 p-6">
        <h2 className="text-base font-semibold text-civic-900">
          Directory Signals
        </h2>
        <dl className="mt-5 divide-y divide-civic-100">
          {statusRows.map(([label, value]) => (
            <div
              key={label}
              className="grid grid-cols-[100px_minmax(0,1fr)] gap-4 py-4 first:pt-0 last:pb-0"
            >
              <dt className="text-sm font-medium text-civic-700">{label}</dt>
              <dd className="text-sm text-ink-700">{value}</dd>
            </div>
          ))}
        </dl>
      </aside>

      <section className="grid gap-4 sm:grid-cols-3 lg:col-span-2">
        <ShellPanel
          title="Search"
          body="City-first entry into the directory."
        />
        <ShellPanel
          title="Results"
          body="Scannable listing surfaces for clinics."
        />
        <ShellPanel
          title="Reports"
          body="Recent availability signals for each listing."
        />
      </section>
    </div>
  );
}

function ShellPanel({ title, body }: { title: string; body: string }) {
  return (
    <article className="rounded-lg border border-ink-100 bg-surface-raised p-5">
      <h2 className="text-base font-semibold text-ink-900">{title}</h2>
      <p className="mt-2 text-sm leading-6 text-ink-700">{body}</p>
    </article>
  );
}
