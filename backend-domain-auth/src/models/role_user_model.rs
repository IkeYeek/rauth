use crate::api_error::ApiError;
use crate::models::role_model::Role;
use crate::models::role_user_model::roles_users::user_id;
use crate::models::user_model::User;
use crate::schema::*;
use diesel::result::DatabaseErrorKind;
use diesel::ExpressionMethods;
use diesel::{
    insert_into, Associations, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SqliteConnection,
};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(User))]
#[diesel(table_name = roles_users)]
#[diesel(primary_key(role_id, user_id))]
pub(crate) struct RoleUser {
    pub(crate) role_id: i32,
    pub(crate) user_id: i32,
}
impl RoleUser {
    pub(crate) fn add_user_to_role(
        db: &mut SqliteConnection,
        user: &User,
        role: &Role,
    ) -> Result<(), ApiError> {
        let role_user = NewRoleUser {
            role_id: role.id,
            user_id: user.id,
        };

        match insert_into(roles_users::dsl::roles_users)
            .values(&role_user)
            .execute(&mut *db)
        {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(e, _)) => match e {
                DatabaseErrorKind::UniqueViolation
                | DatabaseErrorKind::NotNullViolation
                | DatabaseErrorKind::ForeignKeyViolation => Err(ApiError::Role),
                _ => Err(ApiError::Internal),
            },
            _ => Err(ApiError::Internal),
        }
    }

    pub(crate) fn remove_user_from_role(
        db: &mut SqliteConnection,
        user: &User,
        role: &Role,
    ) -> Result<(), ApiError> {
        let role_user_entry = roles_users::dsl::roles_users
            .filter(roles_users::dsl::role_id.eq(role.id))
            .filter(user_id.eq(user.id));
        match diesel::delete(role_user_entry).execute(db) {
            Ok(res) => return if res > 0 { Ok(()) } else { Err(ApiError::Role) },
            Err(diesel::result::Error::NotFound) => Err(ApiError::Role),
            Err(e) => {
                eprintln!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }
}
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles_users)]
pub struct NewRoleUser {
    pub(crate) role_id: i32,
    pub(crate) user_id: i32,
}
