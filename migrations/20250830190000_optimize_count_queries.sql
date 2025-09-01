-- Migration: Optimize count queries
-- Created: 2025-08-30
-- Author: Osef
-- Description: Add missing indexes to optimize count queries
-- Version: 1.0.0

-- Index pour optimiser le comptage des beatmaps
CREATE INDEX IF NOT EXISTS idx_beatmap_count ON beatmap(id);

-- Index pour optimiser le comptage des beatmapsets
CREATE INDEX IF NOT EXISTS idx_beatmapset_count ON beatmapset(id);

-- Index composite pour optimiser la requête MSD avec rate = 1.0
CREATE INDEX IF NOT EXISTS idx_msd_rate_pattern ON msd(rate, main_pattern) WHERE rate = 1.0 AND main_pattern IS NOT NULL;

-- Index pour optimiser le filtrage par rate
CREATE INDEX IF NOT EXISTS idx_msd_rate ON msd(rate) WHERE rate = 1.0;
