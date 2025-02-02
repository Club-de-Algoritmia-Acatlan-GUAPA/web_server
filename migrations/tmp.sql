
-- Add migration script here
-- Add migration script here
create table users (
    user_id uuid primary key default uuid_generate_v4(),
    username text unique not null,
    created_at timestamptz not null default now(),
    school_id integer references school(id),
    country_id integer references country(id),
    profile_picture bytea,
    github text,
    website text,
    bio text,
    first_name text,
    last_name text
);

create table email(
    email text primary key,
    user_id uuid not null,
    foreign key (user_id) references users (user_id),
    is_primary boolean not null default false,
    is_verified boolean not null default false
);

create table country(
    id serial primary key,
    country text unique not null,
    abbreviation text unique not null,
    language text not null,
    asset_id uuid references asset(id)
);

create table school(
    id serial primary key,
    school text unique not null,
    asset_id uuid references asset(id)
);

create table asset(
    id uuid primary key default uuid_generate_v4(),
    user_id uuid not null,
    foreign key (user_id) references users (user_id),
    asset bytea not null,
    asset_name text not null,
    asset_type text not null
);
