mod cloudinary;
mod components;
mod config;
mod handlers;
mod models;
mod player;
mod router;
mod theme;
mod views;

use migration::{Migrator, MigratorTrait};
use boutique::{assets::manifest, server::Server};
use sea_orm::Database;
use std::env;

use router::create_router;

pub type AppState = boutique::AuthState<models::user::Model>;

const BUILD_ROUTE: &str = "/build";
const BUILD_DIR: &str = "public/build";

async fn serve((state, port): (AppState, String)) {
    Server::new(state.clone())
        .debug(cfg!(debug_assertions))
        .static_dir("/assets", "public/assets")
        .file("/favicon.ico", "public/assets/favicon.ico")
        .static_dir(BUILD_ROUTE, BUILD_DIR)
        .serve(create_router(state), &port)
        .await;
}

fn load_env() {
    if dotenvy::dotenv().is_ok() {
        return;
    }

    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env");
    dotenvy::from_path(workspace).ok();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    load_env();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    config::init();
    cloudinary::init();
    manifest::init(BUILD_ROUTE, BUILD_DIR);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("migration failed");

    let port = env::var("PORT").unwrap_or("3000".into());

    let state = AppState::new(db, jwt_secret);

    println!("listening on http://localhost:{port}");
    boutique::run((state.clone(), port), serve).await;
}
