mod add_username_to_users;
mod create_puzzles;

use boutique::sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = boutique::migrations::all();
        migrations.push(Box::new(add_username_to_users::Migration));
        migrations.push(Box::new(create_puzzles::Migration));
        migrations
    }
}
