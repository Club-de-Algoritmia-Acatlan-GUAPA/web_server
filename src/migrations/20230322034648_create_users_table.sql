-- Add migration script here
create table users (
    id serial not null,
    primary key(id),
    email text not null,
    password text not null,
    is_validated boolean not null,
    creation_date timestampz not null,
    username text not null,
    user_role text not null
);
