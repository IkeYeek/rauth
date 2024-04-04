use crate::api_error::ApiError;
use crate::models::role_model::Role;
use crate::models::user_model::User;
use diesel::result::DatabaseErrorKind;
use diesel::ExpressionMethods;
use diesel::{
    insert_into, Associations, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, SqliteConnection,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::roles_users)]
#[diesel(primary_key(role, user_id))]
pub(crate) struct RoleUser {
    pub(crate) role: String,
    pub(crate) user_id: i32,
}
impl RoleUser {
    pub(crate) fn add_role_to_user(
        db: &mut SqliteConnection,
        user: &User,
        role: &Role,
    ) -> Result<(), ApiError> {
        let new_role_user = NewRoleUser {
            user_id: user.id,
            role: role.role.clone(),
        };
        match insert_into(crate::schema::roles_users::dsl::roles_users)
            .values(&new_role_user)
            .execute(db)
        {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(e, _)) => match e {
                DatabaseErrorKind::UniqueViolation
                | DatabaseErrorKind::ForeignKeyViolation
                | DatabaseErrorKind::NotNullViolation
                | DatabaseErrorKind::CheckViolation => Err(ApiError::Role),
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

    pub(crate) fn roles_from_user(
        db: &mut SqliteConnection,
        user: &User,
    ) -> Result<Role, ApiError> {
        match crate::schema::roles_users::dsl::roles_users
            .filter(crate::schema::roles_users::dsl::user_id.eq(user.id))
            .select(RoleUser::as_select())
            .first(&mut *db)
        {
            Ok(role) => Ok(Role::from(&role.role)?),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Role)
            }
        }
    }
}
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles_users)]
pub struct NewRoleUser {
    pub(crate) role: String,
    pub(crate) user_id: i32,
}
