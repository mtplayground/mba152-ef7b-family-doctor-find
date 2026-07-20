import { NavLink, Outlet } from 'react-router-dom';

const navItems = [
  { label: 'Search', to: '/' },
  { label: 'Coverage', to: '/coverage' },
];

export function AppShell() {
  return (
    <div className="min-h-screen bg-surface text-ink-900">
      <header className="border-b border-ink-100 bg-white/90 backdrop-blur">
        <div className="mx-auto flex min-h-16 w-full max-w-6xl flex-col gap-3 px-4 py-4 sm:flex-row sm:items-center sm:justify-between sm:px-6 lg:px-8">
          <NavLink to="/" className="flex items-center gap-3">
            <span className="grid h-10 w-10 place-items-center rounded-control bg-civic-700 text-sm font-bold text-white">
              FD
            </span>
            <span>
              <span className="block text-base font-semibold leading-tight">
                Family Doctor Finder
              </span>
              <span className="block text-sm text-ink-500">
                Availability by city
              </span>
            </span>
          </NavLink>

          <nav className="flex gap-1 rounded-control border border-ink-100 bg-surface-raised p-1">
            {navItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                className={({ isActive }) =>
                  [
                    'rounded-control px-3 py-2 text-sm font-medium transition',
                    isActive
                      ? 'bg-civic-700 text-white'
                      : 'text-ink-700 hover:bg-civic-50 hover:text-civic-900',
                  ].join(' ')
                }
              >
                {item.label}
              </NavLink>
            ))}
          </nav>
        </div>
      </header>

      <main className="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 lg:px-8">
        <Outlet />
      </main>
    </div>
  );
}
