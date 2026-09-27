use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    if dotenvy::dotenv().is_err() {
        dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/../.env")).ok();
    }
    cli::run_cli(migration::Migrator).await;
}
