-- Your SQL goes here
create table users (
    id integer primary key not null ,
    login text unique not null CHECK (
        length(login) > 3
        ),
);
insert into users(login, hash) values ('root', 'root');