-- Your SQL goes here
create table url_rules(
                              id integer primary key not null ,
                              url text not null,
                              group_id integer not null,
                              unique (url, group_id),
                              foreign key (group_id) references groups(group_id)
)
