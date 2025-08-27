-- Migration: Create pending_beatmap table
-- Created: 2025-08-26
-- Author: Osef
-- Description: Table for stocking pending beatmaps
-- Version: 1.0.0

-- Table pending_beatmap
create table if not exists pending_beatmap (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    hash text not null unique,
    created_at timestamp default now()
);

-- Indexes -- 
create index if not exists idx_pending_beatmap_created_at on pending_beatmap(created_at);
create index if not exists idx_pending_beatmap_hash on pending_beatmap(hash);


