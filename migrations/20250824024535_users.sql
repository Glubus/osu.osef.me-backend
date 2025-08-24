-- Add migration script here
-- Utilisateurs Discord
CREATE TABLE users (
    discord_id BIGINT PRIMARY KEY,
    username TEXT,        -- optionnel, juste pour afficher
    created_at TIMESTAMP DEFAULT NOW()
);

-- Tokens permanents pour devices
CREATE TABLE device_tokens (
    token UUID PRIMARY KEY,
    discord_id BIGINT REFERENCES users(discord_id),
    device_name TEXT,     -- optionnel, pour identifier le device
    created_at TIMESTAMP DEFAULT NOW()
);

-- Bans simples
CREATE TABLE bans (
    discord_id BIGINT PRIMARY KEY,
    reason TEXT,           -- optionnel
    banned_at TIMESTAMP DEFAULT NOW()
);