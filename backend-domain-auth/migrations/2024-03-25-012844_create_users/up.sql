-- Your SQL goes here
create table users (
    id integer primary key not null ,
    login text unique not null,
    hash text not null
);
insert into users(login, hash) values ('root', 'root');