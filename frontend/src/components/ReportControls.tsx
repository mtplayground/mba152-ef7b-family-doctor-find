import { useState } from 'react';
import { ApiError } from '../api/client';
import {
  useConfirmAcceptingMutation,
  useStatusChangeMutation,
} from '../api/hooks';

interface ReportControlsProps {
  doctorId: number;
  compact?: boolean;
}

type LastAction = 'confirm' | 'change' | null;

export function ReportControls({
  doctorId,
  compact = false,
}: ReportControlsProps) {
  const [lastAction, setLastAction] = useState<LastAction>(null);
  const [lastError, setLastError] = useState<Error | null>(null);
  const confirmAccepting = useConfirmAcceptingMutation(doctorId);
  const reportChange = useStatusChangeMutation(doctorId);
  const isSaving = confirmAccepting.isPending || reportChange.isPending;

  function handleConfirmAccepting() {
    setLastAction(null);
    setLastError(null);
    confirmAccepting.mutate(
      {},
      {
        onSuccess: () => setLastAction('confirm'),
        onError: (error) => setLastError(normalizeError(error)),
      },
    );
  }

  function handleReportChange() {
    setLastAction(null);
    setLastError(null);
    reportChange.mutate(
      {},
      {
        onSuccess: () => setLastAction('change'),
        onError: (error) => setLastError(normalizeError(error)),
      },
    );
  }

  return (
    <div className={compact ? 'grid gap-2' : 'grid gap-3'}>
      <div
        className={compact ? 'flex flex-wrap gap-2' : 'flex flex-wrap gap-3'}
      >
        <button
          type="button"
          disabled={isSaving}
          className={[
            'rounded-control bg-civic-700 font-semibold text-white transition hover:bg-civic-600 disabled:cursor-not-allowed disabled:opacity-60',
            compact ? 'px-3 py-2 text-xs' : 'px-4 py-2 text-sm',
          ].join(' ')}
          onClick={handleConfirmAccepting}
        >
          {confirmAccepting.isPending ? 'Saving...' : 'Still accepting'}
        </button>
        <button
          type="button"
          disabled={isSaving}
          className={[
            'rounded-control border border-service-red/30 bg-white font-semibold text-service-red transition hover:bg-service-red/10 disabled:cursor-not-allowed disabled:opacity-60',
            compact ? 'px-3 py-2 text-xs' : 'px-4 py-2 text-sm',
          ].join(' ')}
          onClick={handleReportChange}
        >
          {reportChange.isPending ? 'Saving...' : 'Report change'}
        </button>
      </div>

      <p
        className={[
          'min-h-5 text-sm',
          lastError ? 'text-service-red' : 'text-ink-500',
          compact ? 'text-xs' : '',
        ].join(' ')}
        aria-live="polite"
      >
        {lastError ? errorMessage(lastError) : successMessage(lastAction)}
      </p>
    </div>
  );
}

function successMessage(action: LastAction) {
  if (action === 'confirm') {
    return 'Confirmation recorded.';
  }

  if (action === 'change') {
    return 'Status-change report recorded.';
  }

  return ' ';
}

function errorMessage(error: Error) {
  return error instanceof ApiError
    ? error.message
    : 'The report could not be saved.';
}

function normalizeError(error: unknown) {
  return error instanceof Error
    ? error
    : new Error('The report could not be saved.');
}
