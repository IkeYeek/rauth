use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Queryable, Selectable, PartialEq, Debug, Serialize, Deserialize, AsChangeset,
)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Role {
    pub(crate) id: i32,
    pub(crate) role_name: String,
    pub(crate) superior_role: Option<i32>,
}

impl Role {}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub role_name: String,
    pub(crate) superior_role: Option<i32>,
}
