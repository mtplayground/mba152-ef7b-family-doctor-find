export type ResultsViewMode = 'list' | 'map';

interface ResultsViewToggleProps {
  value: ResultsViewMode;
  onChange: (value: ResultsViewMode) => void;
}

const options: Array<{ value: ResultsViewMode; label: string }> = [
  { value: 'list', label: 'List' },
  { value: 'map', label: 'Map' },
];

export function ResultsViewToggle({ value, onChange }: ResultsViewToggleProps) {
  return (
    <div
      className="inline-flex rounded-control border border-ink-100 bg-white p-1"
      role="group"
      aria-label="Results view"
    >
      {options.map((option) => {
        const isSelected = value === option.value;

        return (
          <button
            key={option.value}
            type="button"
            aria-pressed={isSelected}
            className={[
              'min-h-10 rounded-control px-4 text-sm font-semibold transition',
              isSelected
                ? 'bg-civic-700 text-white shadow-sm'
                : 'text-ink-700 hover:bg-civic-50 hover:text-civic-900',
            ].join(' ')}
            onClick={() => onChange(option.value)}
          >
            {option.label}
          </button>
        );
      })}
    </div>
  );
}
