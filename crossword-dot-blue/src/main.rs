mod cloudinary;
mod components;
mod config;
mod handlers;
mod models;
mod player;
mod router;
mod theme;
mod views;

use axum::extract::FromRef;
use migration::{Migrator, MigratorTrait};
use boutique::{AuthState, server::{Cache, Server}, session, store::sea_orm::SeaOrmStore};
use sea_orm::{Database, DatabaseConnection};
use std::{env, time::Duration};
use tracing_subscriber::EnvFilter;

use router::create_router;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub auth: AuthState<models::user::Model>,
}

impl FromRef<AppState> for AuthState<models::user::Model> {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}

const ASSETS_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);

async fn serve((state, port): (AppState, String)) {
    Server::new()
        .debug(cfg!(debug_assertions))
        .static_dir_with("/assets", "public/assets", Cache::MaxAge(ASSETS_MAX_AGE))
        .file_with("/favicon.ico", "public/assets/favicon.ico", Cache::MaxAge(ASSETS_MAX_AGE))
        .error_page_with_state(state.clone(), handlers::error::render)
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

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")))
        .init();

    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    config::init();
    cloudinary::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("migration failed");

    let port = env::var("PORT").unwrap_or("3000".into());

    let auth = AuthState::new(SeaOrmStore::new(db.clone()), secret);
    let state = AppState { db, auth };
    let _session_cleanup = session::spawn_cleanup(state.auth.clone(), Duration::from_secs(60 * 60));

    println!("listening on http://localhost:{port}");
    boutique::run((state.clone(), port), serve).await;
}
