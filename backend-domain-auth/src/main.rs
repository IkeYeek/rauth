use crate::routes::group_routes::{
    add_user_to_group, create_group, delete_group, delete_user_from_group, one_group, update_group,
};
use crate::routes::user_routes::{
    all_users, create_user, delete_user, get_user_groups, one_user, update_user,
};
use actix_web::{web, App, HttpServer};
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;

pub(crate) mod api_error;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod schema;

struct AppDatabaseState {
    db: Mutex<SqliteConnection>,
}
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = web::Data::new(AppDatabaseState {
        db: Mutex::new(establish_connection()),
    });
    HttpServer::new(move || {
        App::new().app_data(db.clone()).service(
            web::scope("/api")
                .service(
                    web::scope("/users")
                        .service(all_users)
                        .service(create_user)
                        .service(one_user)
                        .service(update_user)
                        .service(delete_user)
                        .service(get_user_groups),
                )
                .service(
                    web::scope("/groups")
                        .service(create_group)
                        .service(one_group)
                        .service(update_group)
                        .service(delete_group)
                        .service(add_user_to_group)
                        .service(delete_user_from_group),
                ),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
