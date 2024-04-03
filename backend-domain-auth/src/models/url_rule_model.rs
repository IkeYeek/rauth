use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::user_model::User;
use diesel::ExpressionMethods;
use diesel::{
    insert_into, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
    SqliteConnection,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::url_rules)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct URLRule {
    pub(crate) id: i32,
    pub(crate) url: String,
    pub(crate) group_id: i32,
}

impl URLRule {
    pub(crate) fn create(
        db: &mut SqliteConnection,
        url_rule: &NewURLRule,
    ) -> Result<URLRule, ApiError> {
        match insert_into(crate::schema::url_rules::dsl::url_rules)
            .values(url_rule)
            .get_result::<URLRule>(db)
        {
            Ok(rule) => Ok(rule),
            Err(diesel::result::Error::DatabaseError(e, _)) => match e {
                diesel::result::DatabaseErrorKind::UniqueViolation
                | diesel::result::DatabaseErrorKind::NotNullViolation => Err(ApiError::URLRule),
                _ => {
                    error!("{e:?}");
                    Err(ApiError::Internal)
                }
            },
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn get(db: &mut SqliteConnection, rule_id: i32) -> Result<URLRule, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .filter(crate::schema::url_rules::dsl::id.eq(rule_id))
            .get_result::<URLRule>(db)
            .map_err(|_| ApiError::Internal)?)
    }

    pub(crate) fn get_all(db: &mut SqliteConnection) -> Result<Vec<URLRule>, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .select(URLRule::as_select())
            .load(db)
            .map_err(|_| ApiError::Internal)?)
    }

    pub(crate) fn delete(db: &mut SqliteConnection, url_rule_id: i32) -> Result<(), ApiError> {
        match diesel::delete(
            crate::schema::url_rules::dsl::url_rules
                .filter(crate::schema::url_rules::dsl::id.eq(url_rule_id)),
        )
        .execute(db)
        {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::NotFound) => Err(ApiError::URLRule),
            _ => Err(ApiError::Internal),
        }
    }

    pub(crate) fn for_url(db: &mut SqliteConnection, url: &str) -> Result<Vec<URLRule>, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .filter(crate::schema::url_rules::dsl::url.eq(url))
            .select(URLRule::as_select())
            .load(db)
            .map_err(|_| ApiError::Internal)?)
    }

    pub(crate) fn for_group(
        db: &mut SqliteConnection,
        group: &Group,
    ) -> Result<Vec<URLRule>, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .filter(crate::schema::url_rules::dsl::group_id.eq(group.id))
            .select(URLRule::as_select())
            .load(db)
            .map_err(|_| ApiError::Internal)?)
    }

    pub(crate) fn for_user(
        db: &mut SqliteConnection,
        user: &User,
    ) -> Result<Vec<URLRule>, ApiError> {
        let mut rules = Vec::<URLRule>::new();
        for group in User::get_groups(db, user)? {
            rules.append(
                &mut crate::schema::url_rules::dsl::url_rules
                    .filter(crate::schema::url_rules::dsl::group_id.eq(group.id))
                    .select(URLRule::as_select())
                    .load(db)
                    .map_err(|_| ApiError::Internal)?,
            );
        }
        Ok(rules)
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::url_rules)]
pub struct NewURLRule {
    pub(crate) url: String,
    pub(crate) group_id: i32,
}
