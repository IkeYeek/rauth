// @generated automatically by Diesel CLI.

diesel::table! {
    domain_rules (id) {
        id -> Integer,
        domain -> Text,
        group_id -> Integer,
    }
}

diesel::table! {
    groups (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    groups_users (group_id, user_id) {
        group_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    jwt (id) {
        id -> Integer,
        jwt_id -> Text,
        user_id -> Integer,
        needs_refresh -> Integer,
    }
}

diesel::table! {
    roles_users (role, user_id) {
        role -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    url_rules (id) {
        id -> Integer,
        url -> Text,
        group_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        login -> Text,
        hash -> Text,
    }
}

diesel::joinable!(jwt -> users (user_id));
diesel::joinable!(roles_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    domain_rules,
    groups,
    groups_users,
    jwt,
    roles_users,
    url_rules,
    users,
);
