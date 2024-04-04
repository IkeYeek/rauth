use std::future::{Future, ready};
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest, web};
use actix_web::dev::Payload;
use crate::api_error::ApiError;
use crate::models::group_user_model::GroupUser;
use crate::models::user_model::User;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, SqliteConnection,
};
use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl};
use futures::future::{err, ok};
use log::error;
use serde::{Deserialize, Serialize};
use crate::helpers::try_get_connection;
use crate::models::jwt_model::JWTInternal;
use crate::{KeySet, StorageState};

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
        match insert_into(crate::schema::groups::dsl::groups)
            .values(g)
            .get_results::<Group>(db)
        {
            Ok(mut res) => match res.pop() {
                Some(created_group) => Ok(created_group),
                None => Err(ApiError::Internal),
            },
            Err(_) => Err(ApiError::GroupCreation),
        }
    }
    pub(crate) fn read_all(db: &mut SqliteConnection) -> Result<Vec<Group>, ApiError> {
        match crate::schema::groups::dsl::groups
            .select(Group::as_select())
            .load(db)
        {
            Ok(all_groups) => Ok(all_groups),
            Err(e) => {
                error!("7$i{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn read_by_id(db: &mut SqliteConnection, group_id: i32) -> Result<Group, ApiError> {
        match crate::schema::groups::dsl::groups
            .filter(crate::schema::groups::dsl::id.eq(group_id))
            .select(Group::as_select())
            .first(db)
        {
            Ok(g) => Ok(g),
            Err(diesel::result::Error::NotFound) => Err(ApiError::Group),
            Err(e) => {
                error!("{e:?}");
                Err(ApiError::Internal)
            }
        }
    }

    pub(crate) fn update_group(db: &mut SqliteConnection, group: &Group) -> Result<(), ApiError> {
        match diesel::update(crate::schema::groups::dsl::groups)
            .filter(crate::schema::groups::dsl::id.eq(group.id))
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

        match diesel::delete(
            crate::schema::groups::dsl::groups.filter(crate::schema::groups::dsl::id.eq(group.id)),
        )
        .execute(db)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiError::Internal),
        }
    }
}
#[derive(Debug)]
pub(crate) struct Groups(pub(crate) Vec<Group>);

impl FromRequest for Groups {
    type Error = ApiError;
    type Future =  Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let mut db = if let Some(storage) = req.app_data::<web::Data<StorageState>>() {
            match try_get_connection(storage) {
                Ok(db) => db,
                Err(e) => return Box::pin(ready(Err(e))),
            }
        } else {
            error!("couldn't access storage");
            return Box::pin(ready(Err(ApiError::Internal)));
        };
        let Some(key_set) = req.app_data::<web::Data<KeySet>>() else {
            error!("couldn't access key set");
            return Box::pin(ready(Err(ApiError::Internal)));
        };
        let mut refresh_cookie = false;
        let req = req.clone();
        match req.cookie("jwt") {
            Some(jwt) => {
                let claims = match JWTInternal::validate_jwt(&mut db, &jwt.value(), &key_set.decoding) {
                    Ok(claims) => {
                        let needs_refresh = match JWTInternal::needs_refresh(&mut db, &claims) {
                            Ok(needs_refresh) => {
                                if needs_refresh {
                                    refresh_cookie = true;
                                }
                                needs_refresh
                            }
                            Err(e) => {
                                error!("{e:?}");
                                return Box::pin(ready(Err(e)));
                            }
                        };
                        if needs_refresh {
                            match JWTInternal::refresh(
                                &mut db,
                                &claims.user,
                                &claims.jti,
                                &key_set.encoding,
                            ) {
                                Ok(refresh) => match JWTInternal::register(&mut db, &refresh) {
                                    Ok(()) => refresh,
                                    Err(e) => return Box::pin(ready(Err(e))),
                                },
                                Err(e) => return Box::pin(ready(Err(e))),
                            }
                        } else {
                            match JWTInternal::from(&claims, &key_set.encoding) {
                                Ok(token) => token,
                                Err(e) => return Box::pin(ready(Err(e))),
                            }
                        }
                    }
                    Err(e) => return Box::pin(ready(Err(e))),
                };
                return Box::pin(ok(Groups(claims.claims.groups)));
            },
            None => return Box::pin(ok(Groups(vec!(Group {
                id: 1,
                name: "public".to_string()
            }))))  // TODO make this cleaner
        }
    }
}


#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::groups)]
pub struct NewGroup {
    pub(crate) name: String,
}
