-- Your SQL goes here
create table groups (
    id integer primary key not null,
    name text unique not null
);
insert into groups (name) values ('public');