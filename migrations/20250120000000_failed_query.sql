-- Migration: Create failed_query table
-- Created: 2025-01-20

CREATE TABLE failed_query (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    hash text NOT NULL,
    created_at timestamp DEFAULT now()
);

-- Index pour optimiser les recherches par hash
CREATE INDEX idx_failed_query_hash ON failed_query(hash);

-- Index pour optimiser les recherches par date
CREATE INDEX idx_failed_query_created_at ON failed_query(created_at);
