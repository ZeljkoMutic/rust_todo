use actix_web::{web, App, HttpServer};
#[macro_use]
extern crate diesel;

// use actix_web::{dev::ServiceRequest,Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};


mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .min_idle(Some(1))
        .build(manager)
        .expect("Failed to create pool.");

        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .route("/users", web::get().to(handlers::get_users))
                .route("/users/{id}", web::get().to(handlers::get_user_by_id))
                .route("/users", web::post().to(handlers::add_user))
                .route("/users/{id}", web::delete().to(handlers::delete_user))
        })
        .bind(("127.0.0.1",8080))?
        .run()
        .await
}