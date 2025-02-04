-- Add migration script here
create extension if not exists "uuid-ossp";

create table users (
    user_id uuid primary key default uuid_generate_v4(),
    username text unique not null,
    email text unique not null,
    password_hash text not null,
    is_validated boolean not null default false,
    created_at timestamptz not null default now(),
    github text,
    website text,
    bio text,
    first_name text,
    last_name text
);

create table asset(
    id uuid primary key default uuid_generate_v4(),
    user_id uuid not null,
    foreign key (user_id) references users (user_id),
    asset bytea not null,
    asset_name text not null,
    asset_type text not null
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

