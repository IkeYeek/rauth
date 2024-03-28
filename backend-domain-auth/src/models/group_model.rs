use crate::api_error::ApiError;
use crate::models::group_user_model::GroupUser;
use crate::models::user_model::User;
use crate::schema::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, SqliteConnection,
};
use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl};
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    AsChangeset,
    Clone,
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

    pub(crate) fn users_from_group(
        db: &mut SqliteConnection,
        group: &Group,
    ) -> Result<Vec<User>, ApiError> {
        let groups =
            GroupUser::belonging_to(group)
                .inner_join(crate::schema::users::table.on(
                    crate::schema::groups_users::dsl::user_id.eq(crate::schema::users::dsl::id),
                ))
                .select(User::as_select())
                .load::<User>(db)
                .map_err(|e| match e {
                    diesel::result::Error::NotFound => ApiError::Group,
                    _ => ApiError::Internal,
                })?;
        Ok(groups)
    }

    pub(crate) fn delete_group(db: &mut SqliteConnection, group: &Group) -> Result<(), ApiError> {
        Group::users_from_group(&mut *db, group)?
            .iter()
            .try_for_each(|user| GroupUser::remove_user_from_group(db, user, group))?;

        match diesel::delete(groups::dsl::groups.filter(groups::dsl::id.eq(group.id))).execute(db) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiError::Internal),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    pub(crate) name: String,
}
