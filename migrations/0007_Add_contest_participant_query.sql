-- Add migration script here
create table contest_participant
(
    contest_id int not null,
    user_id uuid not null,
    foreign key (contest_id) references contest (id),
    foreign key (user_id) references users (user_id)
);
