-- Migration: Create msd table
-- Created: 2025-08-26
-- Author: Osef
-- Description: Table for stocking MSD (Map Specific Difficulty)
-- Version: 1.0.0

-- Table MSD
create table if not exists msd (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    beatmap_id integer references beatmap(id) on delete cascade,
    overall decimal(6,3),
    stream decimal(6,3),
    jumpstream decimal(6,3),
    handstream decimal(6,3),
    stamina decimal(6,3),
    jackspeed decimal(6,3),
    chordjack decimal(6,3),
    technical decimal(6,3),
    rate decimal(4,2),
    main_pattern varchar(255),
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_overall check (overall >= 0),
    constraint valid_stream check (stream >= 0),
    constraint valid_jumpstream check (jumpstream >= 0),
    constraint valid_handstream check (handstream >= 0),
    constraint valid_stamina check (stamina >= 0),
    constraint valid_jackspeed check (jackspeed >= 0),
    constraint valid_chordjack check (chordjack >= 0),
    constraint valid_technical check (technical >= 0),
    constraint valid_rate check (rate > 0)
);
    
-- Indexes -- 
create index if not exists idx_msd_beatmap_id on msd(beatmap_id);
create index if not exists idx_msd_overall on msd(overall);
create index if not exists idx_msd_stream on msd(stream);
create index if not exists idx_msd_created_at on msd(created_at);
