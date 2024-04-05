use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::group_user_model::GroupUser;
use crate::models::role_model::Role;
use crate::models::role_user_model::RoleUser;
use crate::schema;
use crate::schema::groups;
use crate::schema::users::dsl::users;
use crate::schema::users::{hash, id, login};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::ExpressionMethods;
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, JoinOnDsl, QueryDsl, Queryable,
    RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
};
use log::error;
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
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) login: String,
    pub(crate) hash: String,
}

impl User {
    pub(crate) fn create(db: &mut SqliteConnection, u: &NewUser) -> Result<User, ApiError> {
        match insert_into(users).values(u).get_results::<User>(db) {
            Ok(mut res) => match res.pop() {
                Some(created_user) => {
                    RoleUser::add_role_to_user(
                        db,
                        &created_user,
                        &Role {
                            role: "root".to_string(),
                        },
                    )?;
                    Ok(created_user)
                }
                None => Err(ApiError::Internal),
            },
            Err(_) => Err(ApiError::UserCreation),
        }
    }

    pub(crate) fn get_all(db: &mut SqliteConnection) -> Result<Vec<User>, ApiError> {
        match users.select(User::as_select()).load(db) {
            Ok(all_users) => Ok(all_users),
            Err(e) => {
                error!("1{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn get(db: &mut SqliteConnection, user_id: i32) -> Result<User, ApiError> {
        match users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(&mut *db)
        {
            Ok(read_user) => Ok(read_user),
            Err(_) => Err(ApiError::User),
        }
    }

    pub(crate) fn lookup(
        db: &mut SqliteConnection,
        user_login: &str,
        user_hash: &str,
    ) -> Result<User, ApiError> {
        users
            .filter(login.eq(user_login))
            .filter(hash.eq(user_hash))
            .select(User::as_select())
            .first(db)
            .map_err(|_| ApiError::User)
    }

    pub(crate) fn update_user(db: &mut SqliteConnection, user: &User) -> Result<(), ApiError> {
        match diesel::update(users)
            .filter(id.eq(user.id))
            .set(user)
            .execute(&mut *db)
        {
            Ok(_) => Ok(()),
            Err(DieselError::DatabaseError(e, _)) => match e {
                DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::NotNullViolation => {
                    Err(ApiError::User)
                }
                _ => Err(ApiError::Internal),
            },
            Err(_) => Err(ApiError::Internal),
        }
    }

    pub(crate) fn delete_user(db: &mut SqliteConnection, user: &User) -> Result<(), ApiError> {
        for group in Self::get_groups(db, user)? {
            GroupUser::remove_user_from_group(db, user, &group)?;
        }
        match diesel::delete(users.filter(id.eq(user.id))).execute(db) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiError::Internal),
        }
    }

    pub(crate) fn get_groups(
        db: &mut SqliteConnection,
        user: &User,
    ) -> Result<Vec<Group>, ApiError> {
        match groups::table
            .inner_join(
                schema::groups_users::dsl::groups_users
                    .on(schema::groups_users::dsl::group_id.eq(groups::dsl::id)),
            )
            .filter(schema::groups_users::dsl::user_id.eq(user.id))
            .select(Group::as_select())
            .load::<Group>(db)
        {
            Ok(g) => Ok(g),
            Err(diesel::result::Error::NotFound) => Err(ApiError::Group),
            _ => Err(ApiError::Internal),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub login: String,
    pub hash: String,
}
