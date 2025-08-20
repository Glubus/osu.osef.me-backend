-- Table beatmapset
create table if not exists beatmapset (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    osu_id integer unique,
    artist varchar(255) not null,
    artist_unicode varchar(255),
    title varchar(255) not null,
    title_unicode varchar(255),
    creator varchar(255) not null,
    source varchar(255),
    tags text[],
    has_video boolean not null default false,
    has_storyboard boolean not null default false,
    is_explicit boolean not null default false,
    is_featured boolean not null default false,
    cover_url varchar(255),
    preview_url varchar(255),
    osu_file_url varchar(255),
    created_at timestamp default now(),
    updated_at timestamp default now()
);

-- Table beatmap
create table if not exists beatmap (
    id integer GENERATED ALWAYS AS IDENTITY primary key,
    osu_id integer unique,
    beatmapset_id integer NULL references beatmapset(id) on delete cascade,
    difficulty varchar(255) not null,
    difficulty_rating decimal(4,2) not null,
    count_circles integer not null,
    count_sliders integer not null,
    count_spinners integer not null,
    max_combo integer not null,
    drain_time integer not null,
    total_time integer not null,
    bpm decimal(10,2) not null,
    cs decimal(3,1) not null,
    ar decimal(3,1) not null,
    od decimal(3,1) not null,
    hp decimal(3,1) not null,
    mode integer not null default 0,
    status varchar(20) not null default 'pending',
    file_md5 varchar(32) not null unique,
    file_path varchar(255) not null,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint valid_difficulty_rating check (difficulty_rating >= 0),
    constraint valid_mode check (mode in (0, 1, 2, 3)),
    constraint valid_status check (status in ('pending', 'ranked', 'qualified', 'loved', 'graveyard'))
);


-- Indexes pour la table beatmapset
create index if not exists idx_beatmapset_artist on beatmapset(artist);
create index if not exists idx_beatmapset_title on beatmapset(title);
create index if not exists idx_beatmapset_creator on beatmapset(creator);
create index if not exists idx_beatmapset_created_at on beatmapset(created_at);

-- Indexes pour la table beatmap
create index if not exists idx_beatmap_beatmapset_id on beatmap(beatmapset_id);
create index if not exists idx_beatmap_difficulty on beatmap(difficulty);
create index if not exists idx_beatmap_difficulty_rating on beatmap(difficulty_rating);
create index if not exists idx_beatmap_mode on beatmap(mode);
create index if not exists idx_beatmap_status on beatmap(status);
create index if not exists idx_beatmap_file_md5 on beatmap(file_md5); 