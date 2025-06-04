CREATE TABLE connections (
    id SERIAL PRIMARY KEY, -- Recommended: Add a serial primary key for unique identification
    event_time TIMESTAMPTZ NOT NULL,
    agent_name TEXT NOT NULL,
    status_ok BOOLEAN NOT NULL,
    object_data JSONB
);