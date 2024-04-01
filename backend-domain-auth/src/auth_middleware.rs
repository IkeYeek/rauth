use actix_web::dev::{Service, Transform};
use crate::models::group_model::Group;
use crate::models::role_model::Role;

enum RouteGuard {
    Role(Role),
    Group(Group),
    RoleGroup(Role, Group),
}

