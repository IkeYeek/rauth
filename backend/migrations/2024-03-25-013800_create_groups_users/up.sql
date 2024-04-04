-- Your SQL goes here
create table groups_users (
    group_id integer references groups(group_id) not null ,
    user_id integer references users(user_id) not null ,
    primary key (group_id, user_id)
);
insert into groups_users(group_id, user_id) values (1, 1)