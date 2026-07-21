import { Link, useNavigate, useParams } from 'react-router-dom';
import { ApiError } from '../api/client';
import { useDoctorDetail } from '../api/hooks';
import type {
  AvailabilityReportKind,
  AvailabilityStatus,
  DoctorDetail,
  ReportHistoryItem,
} from '../api/types';
import { ReportControls } from '../components/ReportControls';

export function DoctorDetailPage() {
  const navigate = useNavigate();
  const { doctorId } = useParams();
  const parsedDoctorId = parseDoctorId(doctorId);
  const detail = useDoctorDetail(parsedDoctorId);

  if (!parsedDoctorId) {
    return (
      <section className="rounded-lg border border-service-red/20 bg-white p-6 shadow-sm sm:p-8">
        <h1 className="text-2xl font-semibold text-service-red">
          Doctor not found
        </h1>
        <p className="mt-3 text-sm leading-6 text-ink-700">
          The listing identifier is invalid.
        </p>
        <Link
          to="/"
          className="mt-5 inline-flex rounded-control bg-civic-700 px-4 py-2 text-sm font-semibold text-white"
        >
          Search again
        </Link>
      </section>
    );
  }

  if (detail.isPending) {
    return (
      <section
        className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8"
        aria-busy="true"
      >
        <p className="mb-3 text-sm font-semibold uppercase text-civic-700">
          Loading listing
        </p>
        <div className="h-8 w-64 animate-pulse rounded bg-surface-muted" />
        <p className="mt-4 max-w-xl text-sm leading-6 text-ink-600">
          Fetching contact details, clinic address, and report history.
        </p>
        <div className="mt-6 grid gap-4 lg:grid-cols-[minmax(0,1fr)_320px]">
          <div className="h-72 animate-pulse rounded-lg bg-surface-muted" />
          <div className="h-72 animate-pulse rounded-lg bg-surface-muted" />
        </div>
      </section>
    );
  }

  if (detail.isError || !detail.data) {
    return (
      <section className="rounded-lg border border-service-red/20 bg-white p-6 shadow-sm sm:p-8">
        <h1 className="text-2xl font-semibold text-service-red">
          Detail unavailable
        </h1>
        <p className="mt-3 text-sm leading-6 text-ink-700">
          {detail.error instanceof ApiError
            ? detail.error.message
            : 'The directory could not load this doctor listing.'}
        </p>
        <div className="mt-5 flex flex-wrap gap-3">
          <button
            type="button"
            className="rounded-control bg-civic-700 px-4 py-2 text-sm font-semibold text-white transition hover:bg-civic-600"
            onClick={() => void detail.refetch()}
          >
            Try again
          </button>
          <button
            type="button"
            className="rounded-control border border-ink-100 bg-white px-4 py-2 text-sm font-semibold text-ink-700 transition hover:bg-civic-50 hover:text-civic-900"
            onClick={() => navigate(-1)}
          >
            Back
          </button>
        </div>
      </section>
    );
  }

  return <DoctorDetailContent detail={detail.data} onBack={() => navigate(-1)} />;
}

function DoctorDetailContent({
  detail,
  onBack,
}: {
  detail: DoctorDetail;
  onBack: () => void;
}) {
  return (
    <div className="grid gap-6">
      <button
        type="button"
        className="w-fit rounded-control border border-ink-100 bg-white px-4 py-2 text-sm font-semibold text-ink-700 transition hover:bg-civic-50 hover:text-civic-900"
        onClick={onBack}
      >
        Back to results
      </button>

      <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
          <div>
            <p className="text-sm font-semibold uppercase text-civic-700">
              Doctor listing
            </p>
            <h1 className="mt-2 text-3xl font-semibold leading-tight text-ink-900 sm:text-4xl">
              {detail.fullName}
              {detail.credentials ? (
                <span className="ml-2 text-xl font-medium text-ink-500">
                  {detail.credentials}
                </span>
              ) : null}
            </h1>
            <p className="mt-3 text-base text-ink-700">
              {detail.clinic.name} in {detail.area.name}, {detail.city.name}
            </p>
          </div>
          <div className="grid gap-2 sm:justify-items-end">
            <StatusBadge status={detail.status.current_status} />
            <span className="text-sm font-semibold text-ink-700">
              {recencyLabel(detail)}
            </span>
          </div>
        </div>
      </section>

      <div className="grid gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
        <section className="rounded-lg border border-ink-100 bg-white p-6 shadow-sm">
          <h2 className="text-xl font-semibold text-ink-900">
            Contact and address
          </h2>
          <dl className="mt-5 grid gap-5 text-sm sm:grid-cols-2">
            <InfoBlock label="Doctor phone" value={detail.phone} />
            <InfoBlock label="Doctor email" value={detail.email} />
            <InfoBlock label="Clinic phone" value={detail.clinic.phone} />
            <InfoBlock label="Clinic fax" value={detail.clinic.fax} />
            <InfoBlock label="Clinic email" value={detail.clinic.email} />
            <InfoLink label="Clinic website" value={detail.clinic.websiteUrl} />
            <InfoLink label="Doctor profile" value={detail.profileUrl} />
            <InfoBlock label="Area" value={detail.area.name} />
            <div className="sm:col-span-2">
              <dt className="font-semibold text-ink-500">Clinic address</dt>
              <dd className="mt-1 text-ink-800">{formatAddress(detail)}</dd>
            </div>
          </dl>
        </section>

        <section className="rounded-lg border border-ink-100 bg-civic-50 p-6">
          <h2 className="text-xl font-semibold text-civic-900">
            Current status
          </h2>
          <dl className="mt-5 grid gap-4 text-sm">
            <InfoBlock
              label="Accepting status"
              value={statusLabel(detail.status.current_status)}
            />
            <InfoBlock
              label="Last report"
              value={formatOptionalDate(detail.status.last_reported_at)}
            />
            <InfoBlock
              label="Last accepting confirmation"
              value={formatOptionalDate(
                detail.status.last_confirmed_accepting_at,
              )}
            />
          </dl>
          <div className="mt-6">
            <ReportControls doctorId={detail.id} />
          </div>
        </section>
      </div>

      <section className="rounded-lg border border-ink-100 bg-surface-raised p-6 shadow-sm sm:p-8">
        <div className="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
          <div>
            <p className="text-sm font-semibold uppercase text-civic-700">
              Report history
            </p>
            <h2 className="mt-2 text-2xl font-semibold text-ink-900">
              Availability updates
            </h2>
          </div>
          <p className="text-sm text-ink-500">
            {detail.reportHistory.length}{' '}
            {detail.reportHistory.length === 1 ? 'report' : 'reports'}
          </p>
        </div>

        {detail.reportHistory.length === 0 ? (
          <div className="mt-6 rounded-lg border border-dashed border-ink-100 bg-surface-muted px-4 py-4 text-sm text-ink-700">
            <p className="font-semibold text-ink-900">
              No report history is available yet.
            </p>
            <p className="mt-2 leading-6">
              Use the status controls above when you can confirm whether this
              doctor is still accepting new patients.
            </p>
          </div>
        ) : (
          <ol className="mt-6 divide-y divide-ink-100 overflow-hidden rounded-lg border border-ink-100 bg-white">
            {detail.reportHistory.map((report) => (
              <ReportHistoryRow key={report.id} report={report} />
            ))}
          </ol>
        )}
      </section>
    </div>
  );
}

function ReportHistoryRow({ report }: { report: ReportHistoryItem }) {
  return (
    <li className="grid gap-3 p-4 sm:grid-cols-[180px_minmax(0,1fr)] sm:p-5">
      <div>
        <p className="text-sm font-semibold text-ink-900">
          {formatDate(report.submittedAt)}
        </p>
        <p className="mt-1 text-xs uppercase text-ink-500">
          {reportKindLabel(report.reportKind)}
        </p>
      </div>
      <div>
        <StatusBadge status={report.reportedStatus} />
        {report.note ? (
          <p className="mt-3 text-sm leading-6 text-ink-700">{report.note}</p>
        ) : (
          <p className="mt-3 text-sm leading-6 text-ink-500">
            No note included.
          </p>
        )}
      </div>
    </li>
  );
}

function InfoBlock({
  label,
  value,
}: {
  label: string;
  value: string | null | undefined;
}) {
  return (
    <div>
      <dt className="font-semibold text-ink-500">{label}</dt>
      <dd className="mt-1 break-words text-ink-800">{value || 'Not listed'}</dd>
    </div>
  );
}

function InfoLink({
  label,
  value,
}: {
  label: string;
  value: string | null | undefined;
}) {
  return (
    <div>
      <dt className="font-semibold text-ink-500">{label}</dt>
      <dd className="mt-1 break-words">
        {value ? (
          <a
            href={value}
            className="font-semibold text-civic-700 hover:text-civic-900"
          >
            {value}
          </a>
        ) : (
          <span className="text-ink-800">Not listed</span>
        )}
      </dd>
    </div>
  );
}

function StatusBadge({ status }: { status: AvailabilityStatus }) {
  return (
    <span
      className={[
        'inline-flex w-fit rounded-control px-3 py-1 text-xs font-semibold uppercase',
        statusClassName(status),
      ].join(' ')}
    >
      {statusLabel(status)}
    </span>
  );
}

function parseDoctorId(value: string | undefined) {
  if (!value) {
    return undefined;
  }

  const parsed = Number(value);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : undefined;
}

function formatAddress(detail: DoctorDetail) {
  return [
    detail.clinic.addressLine1,
    detail.clinic.addressLine2,
    detail.clinic.municipality,
    detail.clinic.provinceCode,
    detail.clinic.postalCode,
  ]
    .filter(Boolean)
    .join(', ');
}

function recencyLabel(detail: DoctorDetail) {
  const daysAgo = detail.status.last_confirmed_accepting_days_ago;

  if (daysAgo === null) {
    return 'No accepting confirmation';
  }

  if (daysAgo === 0) {
    return 'Confirmed accepting today';
  }

  if (daysAgo === 1) {
    return 'Confirmed accepting 1 day ago';
  }

  return `Confirmed accepting ${daysAgo} days ago`;
}

function formatOptionalDate(value: string | null) {
  return value ? formatDate(value) : 'Not recorded';
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat('en-CA', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  }).format(date);
}

function reportKindLabel(kind: AvailabilityReportKind) {
  if (kind === 'confirm_accepting') {
    return 'Confirmation';
  }

  return 'Status change';
}

function statusLabel(status: AvailabilityStatus) {
  if (status === 'accepting') {
    return 'Accepting';
  }

  if (status === 'not_accepting') {
    return 'Not accepting';
  }

  return 'Unknown';
}

function statusClassName(status: AvailabilityStatus) {
  if (status === 'accepting') {
    return 'bg-civic-50 text-civic-700';
  }

  if (status === 'not_accepting') {
    return 'bg-service-red/10 text-service-red';
  }

  return 'bg-surface-muted text-ink-700';
}
