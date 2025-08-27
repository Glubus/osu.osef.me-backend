-- Migration: Create skins table
-- Created: 2025-08-26
-- Author: Osef
-- Description: Table for stocking skins
-- Version: 1.0.0

-- Table skins
create table skins (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    name text not null,
    author text not null,
    version text not null, -- 1.0, 1.1, 1.2, etc
    download_url text not null,
    download_count integer default 0,
    cover_url text,
    created_at timestamp default now(),
    note_type varchar(15) not null, -- circle, arrow, diamond, etc
    tags text[] not null -- tags for the skin
);

-- Indexes -- 
create index if not exists idx_skins_created_at on skins(created_at);
create index if not exists idx_skins_note_type on skins(note_type);
create index if not exists idx_skins_tags on skins using gin(tags);
create index if not exists idx_skins_download_count on skins(download_count);