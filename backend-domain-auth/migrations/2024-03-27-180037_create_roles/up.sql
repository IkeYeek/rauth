-- Your SQL goes here
CREATE TABLE roles (
    id integer primary key not null,
    role_name text unique not null,
    superior_role integer,

    foreign key (superior_role) references roles(id)
)