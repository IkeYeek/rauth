-- Your SQL goes here
create table jwt (
    id integer primary key not null,
    jwt_id text unique not null,
    user_id integer not null references users(id),
    needs_refresh integer default (0) CHECK (needs_refresh IN (0, 1)) not null
)