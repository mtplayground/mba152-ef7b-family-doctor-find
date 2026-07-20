CREATE TYPE availability_report_status AS ENUM (
    'accepting',
    'not_accepting',
    'unknown'
);

CREATE TYPE availability_report_kind AS ENUM (
    'confirm_accepting',
    'status_change'
);

CREATE TABLE availability_reports (
    id BIGSERIAL PRIMARY KEY,
    family_doctor_id BIGINT NOT NULL REFERENCES family_doctors (id) ON DELETE CASCADE,
    report_kind availability_report_kind NOT NULL,
    reported_status availability_report_status NOT NULL,
    note TEXT,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT availability_reports_note_not_blank CHECK (
        note IS NULL OR BTRIM(note) <> ''
    ),
    CONSTRAINT availability_reports_note_length CHECK (
        note IS NULL OR LENGTH(note) <= 1000
    ),
    CONSTRAINT availability_reports_confirm_accepting_status CHECK (
        report_kind <> 'confirm_accepting' OR reported_status = 'accepting'
    )
);

CREATE INDEX availability_reports_doctor_submitted_at_idx
ON availability_reports (family_doctor_id, submitted_at DESC);

CREATE INDEX availability_reports_status_submitted_at_idx
ON availability_reports (reported_status, submitted_at DESC);
