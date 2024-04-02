use std::sync::MutexGuard;
use actix_web::web;
use diesel::SqliteConnection;
use log::error;
use crate::api_error::ApiError;
use crate::AppDatabaseState;

pub(crate) fn try_get_connection(db: &web::Data<AppDatabaseState>) -> Result<MutexGuard<SqliteConnection>, ApiError> {
    db.db.try_lock().map_err(|e| {
        error!("{e:?}");
        ApiError::Internal
    })
}