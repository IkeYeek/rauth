use actix_web::{HttpRequest, web};
use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::group_user_model::GroupUser;
use crate::models::role_model::Role;
use crate::models::role_user_model::RoleUser;
use crate::schema;
use crate::schema::groups;
use crate::schema::users::dsl::users;
use crate::schema::users::{id, login};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::ExpressionMethods;
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, JoinOnDsl, QueryDsl, Queryable,
    RunQueryDsl, Selectable, SelectableHelper, SqliteConnection,
};
use log::error;
use serde::{Deserialize, Serialize};
use crate::models::jwt_model::Claims;

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
#[derive(Serialize, Deserialize)]
pub(crate) struct SafeUser {
    pub(crate) id: i32,
    pub(crate) login: String,
}
impl User {
    pub(crate) fn create(db: &mut SqliteConnection, u: &NewUser) -> Result<User, ApiError> {
        if u.hash.len() < 4 {
            return Err(ApiError::User);
        }
        let hashed_new_user = NewUser {
            login: u.login.clone(),
            hash: match bcrypt::hash(&u.hash, 12) {
                Err(e) => {
                    error!("{e:?}");
                    return Err(ApiError::Internal);
                }
                Ok(h) => h,
            },
        };
        match insert_into(users)
            .values(hashed_new_user)
            .get_results::<User>(db)
        {
            Ok(mut res) => match res.pop() {
                Some(created_user) => {
                    RoleUser::add_role_to_user(
                        db,
                        &created_user,
                        &Role {
                            role: "user".to_string(),
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
            Err(diesel::NotFound) => Err(ApiError::UserNotFound),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn lookup(
        db: &mut SqliteConnection,
        user_login: &str,
        user_password: &str,
    ) -> Result<User, ApiError> {
        let matching_user = users
            .filter(login.eq(user_login))
            .select(User::as_select())
            .first(db)
            .map_err(|_| ApiError::User)?;
        if let Ok(matching) = bcrypt::verify(user_password, &matching_user.hash) {
            return if matching {
                Ok(matching_user)
            } else {
                Err(ApiError::User)
            };
        }
        Err(ApiError::Internal)
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
        if let Err(e) = diesel::delete(crate::schema::roles_users::dsl::roles_users)
            .filter(crate::schema::roles_users::dsl::user_id.eq(user.id))
            .execute(db)
        {
            error!("{e:?}");
            return Err(ApiError::Internal);
        };
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
impl From<User> for SafeUser {
    fn from(unsafe_user: User) -> Self {
        return Self {
            id: unsafe_user.id,
            login: unsafe_user.login.clone(),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub login: String,
    pub hash: String,
}
