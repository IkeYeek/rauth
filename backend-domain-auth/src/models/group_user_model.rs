use crate::api_error::ApiError;
use crate::models::group_model::Group;
use crate::models::jwt_model::JWTInternal;
use crate::models::user_model::User;
use diesel::result::DatabaseErrorKind;
use diesel::{
    insert_into, Associations, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SqliteConnection,
};
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, TextExpressionMethods};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::groups_users)]
#[diesel(primary_key(group_id, user_id))]
pub(crate) struct GroupUser {
    pub(crate) group_id: i32,
    pub(crate) user_id: i32,
}
impl GroupUser {
    pub(crate) fn add_user_to_group(
        db: &mut SqliteConnection,
        user: &User,
        group: &Group,
    ) -> Result<(), ApiError> {
        let group_user = NewGroupUser {
            group_id: group.id,
            user_id: user.id,
        };

        match insert_into(crate::schema::groups_users::dsl::groups_users)
            .values(&group_user)
            .execute(&mut *db)
        {
            Ok(_) => {
                JWTInternal::refresh_for_user(db, user)?;
                Ok(())
            }
            Err(diesel::result::Error::DatabaseError(e, _)) => match e {
                DatabaseErrorKind::UniqueViolation
                | DatabaseErrorKind::NotNullViolation
                | DatabaseErrorKind::ForeignKeyViolation => Err(ApiError::Group),
                _ => Err(ApiError::Internal),
            },
            _ => Err(ApiError::Internal),
        }
    }

    pub(crate) fn remove_user_from_group(
        db: &mut SqliteConnection,
        user: &User,
        group: &Group,
    ) -> Result<(), ApiError> {
        let group_user_entry = crate::schema::groups_users::dsl::groups_users
            .filter(crate::schema::groups_users::dsl::group_id.eq(group.id))
            .filter(crate::schema::groups_users::dsl::user_id.eq(user.id));
        match diesel::delete(group_user_entry).execute(db) {
            Ok(res) => {
                if res > 0 {
                    JWTInternal::refresh_for_user(db, user)?;
                    Ok(())
                } else {
                    Err(ApiError::Group)
                }
            }
            Err(diesel::result::Error::NotFound) => Err(ApiError::Group),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn user_allowed_to_origin(
        db: &mut SqliteConnection,
        origin: &str,
        host: &str,
        groups: &Vec<i32>,
    ) -> Result<(), ApiError> {
        match crate::schema::domain_rules::dsl::domain_rules
            .left_join(
                crate::schema::url_rules::dsl::url_rules
                    .on(crate::schema::domain_rules::dsl::group_id
                        .eq(crate::schema::url_rules::dsl::group_id)),
            )
            .filter(
                crate::schema::domain_rules::dsl::domain
                    .like(host)
                    .or(crate::schema::url_rules::dsl::url.like(origin))
                    .and(
                        crate::schema::domain_rules::dsl::group_id
                            .eq_any(groups)
                            .or(crate::schema::url_rules::dsl::group_id.eq_any(groups)),
                    ),
            )
            .count()
            .get_result::<i64>(&mut *db)
        {
            Ok(n) => {
                if n > 0 {
                    Ok(())
                } else {
                    Err(ApiError::Group)
                }
            }
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }
}
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::groups_users)]
pub struct NewGroupUser {
    pub(crate) group_id: i32,
    pub(crate) user_id: i32,
}
