-- Your SQL goes here
create table domain_rules (
    id integer primary key not null ,
    domain text not null,
    group_id integer not null,
    unique (domain, group_id),
    foreign key (group_id) references groups(group_id)
)