use crate::routes::auth_routes::{auth, has_access, is_auth};
use crate::routes::group_routes::{
    add_user_to_group, all_groups, create_group, delete_group, delete_user_from_group, one_group,
    update_group,
};
use crate::routes::user_routes::{
    all_users, create_user, delete_user, get_user_groups, one_user, update_user,
};
use actix_web::{web, App, HttpServer};
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::env;
use std::sync::Mutex;
use actix_web::middleware::Logger;
use env_logger::Env;
use log::{error, info};

pub(crate) mod api_error;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod schema;
mod auth_middleware;

struct AppDatabaseState {
    db: Mutex<SqliteConnection>,
}
#[derive(Clone)]
struct KeySet {
    decoding: DecodingKey,
    encoding: EncodingKey,
}
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();
    let storage = web::Data::new(AppDatabaseState {
        db: Mutex::new(establish_connection()),
    });
    let keyset = KeySet {
        encoding: EncodingKey::from_ed_pem(include_str!("../keys/private.pem").as_bytes())
            .expect("Couldn't load private key"),
        decoding: DecodingKey::from_ed_pem(include_str!("../keys/public.pem").as_bytes())
            .expect("Couldn't load public key"),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .app_data(web::Data::new(keyset.clone()))
            .wrap(Logger::new("%r - %s - %a %{User-Agent}i"))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(is_auth)
                            .service(auth)
                            .service(has_access),
                    )
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
                            .service(delete_user_from_group)
                            .service(all_groups),
                    ),
            )
    })
    .bind("0.0.0.0:8080")?

    .run()
    .await
}
