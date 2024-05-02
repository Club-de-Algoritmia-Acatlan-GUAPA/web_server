create table contest (
    contest_id serial primary key,
    created_at timestamptz not null default now(),
    author uuid,
    foreign key (author) references users (user_id),
    body text,
    name text
);

create type submission_status as enum (
    'pending',
    'accepted',
    'wrong_answer',
    'runtime_error',
    'time_limit_exceeded',
    'compilation_error',
    'partial_points',
    'unknown_error'
);

create type validation_type as enum (
    'testlib_checker',
    'literal_checker',
    'interactive'
);


-- tags https://stackoverflow.com/questions/41311191/in-postgres-how-to-match-multiple-tags-for-best-performance
create table problem (
    id serial primary key not null,
    created_at timestamptz not null default now(),
    submitted_by uuid not null,
    foreign key (submitted_by) references users (user_id),
    body jsonb not null,
    checker text,
    validation validation_type not null,
    memory_limit smallint not null,
    time_limit smallint not null,
    is_public boolean not null,
    testcases uuid[]
);

-- https://wiki.postgresql.org/wiki/BinaryFilesInDB#What_is_the_best_way_to_store_the_files_in_the_Database.3F
create table testcase (
    problem_id bigint,
    foreign key (problem_id) references problem (id),
    body bytea,
    name text,
    order_execution smallint,
    input_file bytea,
    output_file bytea
);

create table submission (
    submission_id bit(128) primary key,
    user_id uuid not null,
    foreign key (user_id) references users (user_id),
    status submission_status not null default 'pending',
    output jsonb,
    code text,
    language varchar(50) not null,
    execution_time integer,
    points integer
);

create table failed_submission (
    submission_id bit(128) primary key,
    user_id uuid not null,
    foreign key (user_id) references users (user_id),
    status submission_status not null default 'pending',
    output jsonb,
    code text,
    language varchar(50) not null,
    execution_time integer,
    used_memory integer,
    points integer
);

create table problem_serial (
    problem_id bigserial primary key,
    dummy boolean
);
