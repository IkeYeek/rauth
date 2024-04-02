use crate::routes::auth_routes::{auth, has_access, is_auth};
use crate::routes::group_routes::{
    add_user_to_group, all_groups, create_group, delete_group, delete_user_from_group,
    list_users_from_group, one_group, update_group,
};
use crate::routes::user_routes::{
    all_users, create_user, delete_user, get_user_groups, one_user, update_user,
};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use env_logger::Env;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::env;
use std::sync::Mutex;

pub(crate) mod api_error;
pub(crate) mod helpers;
pub(crate) mod models;
pub(crate) mod route_guards;
pub(crate) mod routes;
pub(crate) mod schema;

struct StorageState {
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
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();
    let storage = web::Data::new(StorageState {
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
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .wrap(Logger::new("%r - %s - %a %{User-Agent}i"))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(
                                web::resource("/")
                                    .route(web::get().to(is_auth))
                                    .route(web::post().to(auth)),
                            )
                            .service(
                                web::resource("/has_access/").route(web::get().to(has_access)),
                            ),
                    )
                    .service(
                        web::scope("/users")
                            .service(
                                web::resource("/")
                                    .route(web::get().to(all_users))
                                    .route(web::post().to(create_user)),
                            )
                            .service(
                                web::scope("/{user}")
                                    .service(
                                        web::resource("/")
                                            .route(web::get().to(one_user))
                                            .route(web::patch().to(update_user))
                                            .route(web::delete().to(delete_user)),
                                    )
                                    .service(
                                        web::resource("/groups/")
                                            .route(web::get().to(get_user_groups)),
                                    ),
                            ),
                    )
                    .service(
                        web::scope("/groups")
                            .service(
                                web::resource("/")
                                    .route(web::get().to(all_groups))
                                    .route(web::post().to(create_group)),
                            )
                            .service(
                                web::scope("/{group_id}")
                                    .service(
                                        web::resource("/")
                                            .route(web::get().to(one_group))
                                            .route(web::patch().to(update_group))
                                            .route(web::delete().to(delete_group)),
                                    )
                                    .service(
                                        web::resource("/users/")
                                            .route(web::get().to(list_users_from_group))
                                            .route(web::post().to(add_user_to_group))
                                            .route(web::delete().to(delete_user_from_group)),
                                    ),
                            ),
                    ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
