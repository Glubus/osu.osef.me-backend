-- Migration: Create users table
-- Created: 2025-08-26
-- Author: Osef
-- Description: Table for stocking users
-- Version: 1.0.0

-- Table users
CREATE TABLE users (
    discord_id BIGINT PRIMARY KEY,
    username TEXT,        -- optionnel, juste pour afficher
    created_at TIMESTAMP DEFAULT NOW(),
    roles jsonb DEFAULT '["user"]'
);

-- Table device_tokens
CREATE TABLE device_tokens (
    token UUID PRIMARY KEY,
    discord_id BIGINT REFERENCES users(discord_id),
    device_name TEXT,     -- optionnel, pour identifier le device
    created_at TIMESTAMP DEFAULT NOW()
);

-- Table bans
CREATE TABLE bans (
    discord_id BIGINT PRIMARY KEY,
    reason TEXT,           -- optionnel
    banned_at TIMESTAMP DEFAULT NOW()
);

-- Table new_users
-- used as a temporary table to store users that are not yet in the users table
-- this is used to prevent creating multiple users without validating the creation of users with token
CREATE TABLE new_users (
    discord_id BIGINT PRIMARY KEY,
    username TEXT,
    token UUID, -- token used to validate the creation of the user send in private message on discord via the bot
    created_at TIMESTAMP DEFAULT NOW()
);