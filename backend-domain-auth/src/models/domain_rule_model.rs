use crate::api_error::ApiError;
use crate::schema::*;
use diesel::{Insertable, Queryable, Selectable, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::domain_rules)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct DomainRule {
    pub(crate) id: i32,
    pub(crate) domain: String,
    pub(crate) group_id: i32,
}

impl DomainRule {
    pub(crate) fn create(
        db: &mut SqliteConnection,
        domain_rule: NewDomainRule,
    ) -> Result<DomainRule, ApiError> {
        todo!()
    }

    pub(crate) fn read_all(db: &mut SqliteConnection) -> Result<Vec<DomainRule>, ApiError> {
        todo!()
    }

    pub(crate) fn delete(
        db: &mut SqliteConnection,
        domain_rule: DomainRule,
    ) -> Result<(), ApiError> {
        todo!()
    }

    pub(crate) fn for_domain(
        db: &mut SqliteConnection,
        domain: &str,
    ) -> Result<Vec<DomainRule>, ApiError> {
        todo!()
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = domain_rules)]
pub struct NewDomainRule<'a> {
    pub(crate) domain: &'a str,
    pub(crate) group_id: i32,
}
