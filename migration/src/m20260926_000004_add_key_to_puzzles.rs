use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Puzzles::Table)
                    .add_column(ColumnDef::new(Puzzles::Key).string_len(8).null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Puzzles::Table)
                    .value(Puzzles::Key, Func::cust(Left).arg(Expr::col(Puzzles::Id)).arg(8))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Puzzles::Table)
                    .modify_column(ColumnDef::new(Puzzles::Key).string_len(8).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_puzzles_key")
                    .table(Puzzles::Table)
                    .col(Puzzles::Key)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_puzzles_key").table(Puzzles::Table).to_owned())
            .await?;

        manager
            .alter_table(Table::alter().table(Puzzles::Table).drop_column(Puzzles::Key).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Puzzles {
    Table,
    Id,
    Key,
}

#[derive(DeriveIden)]
struct Left;
