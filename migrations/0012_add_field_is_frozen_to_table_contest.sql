-- Add migration script here
alter table if exists contest add column is_frozen boolean not null default true;
