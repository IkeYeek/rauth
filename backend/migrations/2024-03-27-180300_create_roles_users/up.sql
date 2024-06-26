-- Your SQL goes here
create table roles_users (
    role text CHECK ( role in ('root', 'super', 'user') ) not null ,
    user_id integer references users(id) not null unique,
    primary key (role, user_id)
);
insert into roles_users (role, user_id)
values ('root', 1);