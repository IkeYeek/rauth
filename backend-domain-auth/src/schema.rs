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
    roles (id) {
        id -> Integer,
        role_name -> Text,
        superior_role -> Nullable<Integer>,
    }
}

diesel::table! {
    roles_users (role_id, user_id) {
        role_id -> Integer,
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

diesel::joinable!(roles_users -> roles (role_id));
diesel::joinable!(roles_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    domain_rules,
    groups,
    groups_users,
    roles,
    roles_users,
    url_rules,
    users,
);
