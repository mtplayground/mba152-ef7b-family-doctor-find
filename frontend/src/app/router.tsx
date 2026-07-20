import { createBrowserRouter } from 'react-router-dom';
import { AppShell } from './AppShell';
import { CoveragePage } from '../pages/CoveragePage';
import { HomePage } from '../pages/HomePage';
import { NotFoundPage } from '../pages/NotFoundPage';
import { ResultsPage } from '../pages/ResultsPage';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <AppShell />,
    children: [
      { index: true, element: <HomePage /> },
      { path: 'coverage', element: <CoveragePage /> },
      { path: 'results', element: <ResultsPage /> },
      { path: '*', element: <NotFoundPage /> },
    ],
  },
]);
