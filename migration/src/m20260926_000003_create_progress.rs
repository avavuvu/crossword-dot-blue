use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Progress::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Progress::PlayerId).string_len(32).not_null())
                    .col(ColumnDef::new(Progress::PuzzleId).string_len(32).not_null())
                    .col(ColumnDef::new(Progress::UserId).string().null())
                    .col(ColumnDef::new(Progress::Version).small_integer().not_null())
                    .col(ColumnDef::new(Progress::State).json_binary().not_null())
                    .col(ColumnDef::new(Progress::Completion).string_len(32).not_null())
                    .col(ColumnDef::new(Progress::ElapsedMs).big_integer().not_null())
                    .col(ColumnDef::new(Progress::SolvedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Progress::SavedAt).timestamp_with_time_zone().not_null())
                    .primary_key(Index::create().col(Progress::PlayerId).col(Progress::PuzzleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progress_puzzle_id")
                            .from(Progress::Table, Progress::PuzzleId)
                            .to(Puzzles::Table, Puzzles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progress_user_id")
                            .from(Progress::Table, Progress::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_progress_puzzle_id")
                    .table(Progress::Table)
                    .col(Progress::PuzzleId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_progress_user_id_puzzle_id")
                    .table(Progress::Table)
                    .col(Progress::UserId)
                    .col(Progress::PuzzleId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Progress::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Progress {
    Table,
    PlayerId,
    PuzzleId,
    UserId,
    Version,
    State,
    Completion,
    ElapsedMs,
    SolvedAt,
    SavedAt,
}

#[derive(DeriveIden)]
enum Puzzles {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
