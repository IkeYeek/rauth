use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::user_model::User;
use diesel::ExpressionMethods;
use diesel::{
    insert_into, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable,
    SelectableHelper, SqliteConnection,
};
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
                    eprintln!("{e:?}");
                    Err(ApiError::Internal)
                }
            },
            Err(e) => {
                eprintln!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn read_all(db: &mut SqliteConnection) -> Result<Vec<URLRule>, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .select(URLRule::as_select())
            .load(db)
            .map_err(|e| ApiError::Internal)?)
    }

    pub(crate) fn delete(db: &mut SqliteConnection, url_rule: &URLRule) -> Result<(), ApiError> {
        match diesel::delete(
            crate::schema::url_rules::dsl::url_rules
                .filter(crate::schema::url_rules::dsl::id.eq(url_rule.id)),
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
            .map_err(|err| ApiError::Internal)?)
    }

    pub(crate) fn for_group(
        db: &mut SqliteConnection,
        group: &Group,
    ) -> Result<Vec<URLRule>, ApiError> {
        Ok(crate::schema::url_rules::dsl::url_rules
            .filter(crate::schema::url_rules::dsl::group_id.eq(group.id))
            .select(URLRule::as_select())
            .load(db)
            .map_err(|err| ApiError::Internal)?)
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
pub struct NewURLRule<'a> {
    pub(crate) url: &'a str,
    pub(crate) group_id: i32,
}
