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
        domain_rule: &NewDomainRule,
    ) -> Result<DomainRule, ApiError> {
        match insert_into(crate::schema::domain_rules::dsl::domain_rules)
            .values(domain_rule)
            .get_result::<DomainRule>(db)
        {
            Ok(rule) => Ok(rule),
            Err(diesel::result::Error::DatabaseError(e, _)) => match e {
                diesel::result::DatabaseErrorKind::UniqueViolation
                | diesel::result::DatabaseErrorKind::NotNullViolation => Err(ApiError::DomainRule),
                _ => {
                    error!("3{e:?}");
                    Err(ApiError::Internal)
                }
            },
            Err(e) => {
                error!("4{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn get_all(db: &mut SqliteConnection) -> Result<Vec<DomainRule>, ApiError> {
        crate::schema::domain_rules::dsl::domain_rules
            .select(DomainRule::as_select())
            .load(db)
            .map_err(|e| {
                error!("5{e:?}");
                ApiError::Internal
            })
    }

    pub(crate) fn get(db: &mut SqliteConnection, rule_id: i32) -> Result<DomainRule, ApiError> {
        crate::schema::domain_rules::dsl::domain_rules
            .filter(crate::schema::domain_rules::dsl::id.eq(rule_id))
            .get_result::<DomainRule>(db)
            .map_err(|e| {
                error!("6{e:?}");
                ApiError::Internal
            })
    }

    pub(crate) fn delete(db: &mut SqliteConnection, domain_rule_id: i32) -> Result<(), ApiError> {
        match diesel::delete(
            crate::schema::domain_rules::dsl::domain_rules
                .filter(crate::schema::domain_rules::dsl::id.eq(domain_rule_id)),
        )
        .execute(db)
        {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::NotFound) => Err(ApiError::DomainRule),
            _ => Err(ApiError::Internal),
        }
    }

    pub(crate) fn for_domain(
        db: &mut SqliteConnection,
        domain: &str,
    ) -> Result<Vec<DomainRule>, ApiError> {
        crate::schema::domain_rules::dsl::domain_rules
            .filter(crate::schema::domain_rules::dsl::domain.eq(domain))
            .select(DomainRule::as_select())
            .load(db)
            .map_err(|_| ApiError::Internal)
    }

    pub(crate) fn for_group(
        db: &mut SqliteConnection,
        group: &Group,
    ) -> Result<Vec<DomainRule>, ApiError> {
        crate::schema::domain_rules::dsl::domain_rules
            .filter(crate::schema::domain_rules::dsl::group_id.eq(group.id))
            .select(DomainRule::as_select())
            .load(db)
            .map_err(|_| ApiError::Internal)
    }

    pub(crate) fn for_user(
        db: &mut SqliteConnection,
        user: &User,
    ) -> Result<Vec<DomainRule>, ApiError> {
        let mut rules = Vec::<DomainRule>::new();
        for group in User::get_groups(db, user)? {
            rules.append(
                &mut crate::schema::domain_rules::dsl::domain_rules
                    .filter(crate::schema::domain_rules::dsl::group_id.eq(group.id))
                    .select(DomainRule::as_select())
                    .load(db)
                    .map_err(|_| ApiError::Internal)?,
            );
        }
        Ok(rules)
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::domain_rules)]
pub struct NewDomainRule {
    pub(crate) domain: String,
    pub(crate) group_id: i32,
}
