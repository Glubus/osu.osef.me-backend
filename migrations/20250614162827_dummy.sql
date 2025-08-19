-- Add migration script here

create table if not exists dummy (
    id serial primary key,
    name varchar(255) not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);
