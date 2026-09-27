pub use sea_orm_migration::prelude::*;

mod m20260917_000001_add_username_to_users;
mod m20260917_000002_create_puzzles;
mod m20260926_000003_create_progress;
mod m20260926_000004_add_key_to_puzzles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = boutique::migrations::all();
        migrations.extend([
            Box::new(m20260917_000001_add_username_to_users::Migration) as Box<dyn MigrationTrait>,
            Box::new(m20260917_000002_create_puzzles::Migration),
            Box::new(m20260926_000003_create_progress::Migration),
            Box::new(m20260926_000004_add_key_to_puzzles::Migration),
        ]);
        migrations
    }
}
