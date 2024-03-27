use crate::schema::*;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::url_rules)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct URLRule {
    pub(crate) id: i32,
    pub(crate) url: String,
    pub(crate) group_id: i32,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = url_rules)]
pub struct NewUrlRule<'a> {
    pub(crate) url: &'a str,
    pub(crate) group_id: i32,
}
