-- Your SQL goes here
create table roles_users (
    role_id integer references roles(id) not null,
    user_id integer references users(id) not null,
    primary key (role_id, user_id)
)