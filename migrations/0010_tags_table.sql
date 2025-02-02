-- add migration script here
create table tag (
    id serial primary key,
    name varchar(255) not null
);

create table problem_tag (
    problem_id int not null,
    tag_id int not null,
    primary key (problem_id, tag_id),
    foreign key (problem_id) references problem(id),
    foreign key (tag_id) references tag(id)
);
