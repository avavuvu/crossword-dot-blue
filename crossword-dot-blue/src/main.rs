mod assets;
mod cloudinary;
mod components;
mod error;
mod handlers;
mod migrations;
mod models;
mod router;
mod views;

use boutique::sea_orm_migration::MigratorTrait;
use boutique::server::Server;
use sea_orm::Database;
use std::env;

use migrations::Migrator;
use router::create_router;

pub type AppState = boutique::AuthState<models::user::Model>;

static SITE_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn site_url() -> &'static str {
    SITE_URL.get().map(String::as_str).unwrap_or("")
}

static ADMIN_EMAILS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

pub fn is_admin_email(email: &str) -> bool {
    let email = email.trim().to_ascii_lowercase();
    ADMIN_EMAILS
        .get()
        .is_some_and(|emails| emails.iter().any(|admin| *admin == email))
}

async fn serve((state, port): (AppState, String)) {
    Server::new(state.clone())
        .debug(cfg!(debug_assertions))
        .static_dir("/assets", "public/assets")
        .file("/favicon.ico", "public/assets/favicon.ico")
        .static_dir(assets::ROUTE, assets::DIR)
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
    let site_url = env::var("SITE_URL").expect("SITE_URL must be set");
    SITE_URL.set(site_url.trim_end_matches('/').to_string()).ok();

    let admin_emails = env::var("ADMIN_EMAILS")
        .unwrap_or_default()
        .split(',')
        .map(|email| email.trim().to_ascii_lowercase())
        .filter(|email| !email.is_empty())
        .collect();
    ADMIN_EMAILS.set(admin_emails).ok();

    cloudinary::init();
    assets::init();
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
