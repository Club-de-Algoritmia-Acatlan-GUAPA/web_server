-- Add migration script here

create type relation_type as enum (
    'participant',
    'owner',
    'problem_setter',
    'member',
    'admin'
);

create table relations (
    first_grn varchar(50) not null,
    second_grn varchar(50),
    relation relation_type not null,
    created_at timestamptz not null default now()
);

create index idx_relations on relations (first_grn, second_grn, relation);

