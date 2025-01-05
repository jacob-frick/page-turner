CREATE TABLE book_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),         -- Auto-generated UUID
    status VARCHAR(255) UNIQUE NOT NULL                    -- Status name (e.g., "ToRead", "IsReading")
);

-- Insert predefined statuses with UUID
INSERT INTO book_statuses (status) VALUES
    ('ToRead'),
    ('IsReading'),
    ('FinishedReading'),
    ('Favorited');