use boutique::sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_000002_create_puzzles"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Puzzles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Puzzles::Id).string_len(32).not_null().primary_key())
                    .col(ColumnDef::new(Puzzles::AuthorId).string().not_null())
                    .col(ColumnDef::new(Puzzles::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Puzzles::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Puzzles::Content).json_binary().not_null())
                    .col(ColumnDef::new(Puzzles::Xd).text().not_null())
                    .col(ColumnDef::new(Puzzles::Title).string().null())
                    .col(ColumnDef::new(Puzzles::Notes).text().null())
                    .col(ColumnDef::new(Puzzles::Difficulty).small_integer().null())
                    .col(ColumnDef::new(Puzzles::Region).string_len(16).null())
                    .col(ColumnDef::new(Puzzles::Themed).boolean().not_null().default(false))
                    .col(ColumnDef::new(Puzzles::IsCryptic).boolean().not_null().default(false))
                    .col(ColumnDef::new(Puzzles::IsPublic).boolean().not_null().default(false))
                    .col(ColumnDef::new(Puzzles::PublishedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Puzzles::FeaturedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Puzzles::ShareToken).string_len(32).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_puzzles_author_id")
                            .from(Puzzles::Table, Puzzles::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_puzzles_author_id")
                    .table(Puzzles::Table)
                    .col(Puzzles::AuthorId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Puzzles::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Puzzles {
    Table,
    Id,
    AuthorId,
    CreatedAt,
    UpdatedAt,
    Content,
    Xd,
    Title,
    Notes,
    Difficulty,
    Region,
    Themed,
    IsCryptic,
    IsPublic,
    PublishedAt,
    FeaturedAt,
    ShareToken,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
