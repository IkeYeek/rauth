use crate::api_error::ApiError;
use crate::StorageState;
use actix_web::web;
use diesel::SqliteConnection;
use log::error;
use std::sync::MutexGuard;

pub(crate) fn try_get_connection(
    db: &web::Data<StorageState>,
) -> Result<MutexGuard<SqliteConnection>, ApiError> {
    db.db.lock().map_err(|e| {
        error!("2{e:?}");
        ApiError::Internal
    })
}
