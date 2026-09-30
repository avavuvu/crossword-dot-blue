pub use sea_orm_migration::prelude::*;

mod m00000000_000001_boutique_create_users;
mod m00000000_000002_boutique_create_refresh_tokens;
mod m20260917_000001_add_username_to_users;
mod m20260917_000002_create_puzzles;
mod m20260926_000003_create_progress;
mod m20260926_000004_add_key_to_puzzles;
mod m20260929_000005_create_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m00000000_000001_boutique_create_users::Migration),
            Box::new(m00000000_000002_boutique_create_refresh_tokens::Migration),
            Box::new(m20260917_000001_add_username_to_users::Migration),
            Box::new(m20260917_000002_create_puzzles::Migration),
            Box::new(m20260926_000003_create_progress::Migration),
            Box::new(m20260926_000004_add_key_to_puzzles::Migration),
            Box::new(m20260929_000005_create_sessions::Migration),
        ]
    }
}
