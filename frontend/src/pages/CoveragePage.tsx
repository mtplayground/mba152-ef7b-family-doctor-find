const regions = ['Ontario', 'British Columbia', 'Alberta', 'Nova Scotia'];

export function CoveragePage() {
  return (
    <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
      <p className="mb-3 text-sm font-semibold uppercase text-civic-700">
        Coverage
      </p>
      <h1 className="text-3xl font-semibold text-ink-900">
        Canadian city and area directory
      </h1>
      <div className="mt-6 grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
        {regions.map((region) => (
          <div
            key={region}
            className="rounded-lg border border-ink-100 bg-surface-muted px-4 py-3 text-sm font-medium text-ink-800"
          >
            {region}
          </div>
        ))}
      </div>
    </section>
  );
}
