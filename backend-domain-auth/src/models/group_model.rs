use crate::api_error::ApiError;
use crate::schema::users::dsl::users;
use crate::schema::users::id;
use crate::schema::*;
use actix_web::web;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::ExpressionMethods;
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, QueryDsl, QueryResult, Queryable,
    RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
};
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable, Queryable, Selectable, PartialEq, Debug, Serialize, Deserialize, AsChangeset,
)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Group {
    pub(crate) id: i32,
    pub(crate) name: String,
}

impl Group {
    pub(crate) fn create_group(db: &mut SqliteConnection, g: &NewGroup) -> Result<Group, ApiError> {
        match insert_into(groups::dsl::groups)
            .values(&*g)
            .get_results::<Group>(db)
        {
            Ok(mut res) => match res.pop() {
                Some(created_group) => Ok(created_group),
                None => Err(ApiError::Internal),
            },
            Err(_) => Err(ApiError::GroupCreation),
        }
    }

    pub(crate) fn read_by_id(db: &mut SqliteConnection, group_id: i32) -> Result<Group, ApiError> {
        match groups::dsl::groups
            .filter(groups::dsl::id.eq(group_id))
            .select(Group::as_select())
            .first(db)
        {
            Ok(g) => Ok(g),
            Err(diesel::result::Error::NotFound) => Err(ApiError::Group),
            Err(e) => {
                eprintln!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn update_group(db: &mut SqliteConnection, group: &Group) -> Result<(), ApiError> {
        match diesel::update(groups::dsl::groups)
            .filter(groups::dsl::id.eq(group.id))
            .set(group)
            .execute(&mut *db)
        {
            Ok(_) => Ok(()),
            Err(DieselError::DatabaseError(e, _)) => match e {
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::NotNullViolation => {
                    Err(ApiError::Group)
                }
                _ => Err(ApiError::Internal),
            },
            Err(_) => Err(ApiError::Internal),
        }
    }

    pub(crate) fn delete_group(db: &mut SqliteConnection, group: &Group) -> Result<(), ApiError> {
        match diesel::delete(groups::dsl::groups.filter(groups::dsl::id.eq(group.id))).execute(db) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiError::Internal),
        }
        //TODO remove users from grp
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    pub(crate) name: String,
}
