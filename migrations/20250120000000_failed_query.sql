-- Migration: Create failed_query table
-- Created: 2025-08-26
-- Author: Osef
-- Description: Table for stocking failed queries
-- Version: 1.0.0

CREATE TABLE failed_query (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    hash text NOT NULL,
    created_at timestamp DEFAULT now()
);

-- Index -- 
CREATE INDEX idx_failed_query_hash ON failed_query(hash);
CREATE INDEX idx_failed_query_created_at ON failed_query(created_at);
