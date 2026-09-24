use boutique::sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_000001_add_username_to_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Username).string_len(20).not_null().unique_key())
                    .add_column(ColumnDef::new(Users::IsAdmin).boolean().not_null().default(false))
                    .add_column(ColumnDef::new(Users::DisplayName).string_len(60).null())
                    .add_column(ColumnDef::new(Users::Bio).text().null())
                    .add_column(ColumnDef::new(Users::AvatarPublicId).string_len(200).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Username)
                    .drop_column(Users::IsAdmin)
                    .drop_column(Users::DisplayName)
                    .drop_column(Users::Bio)
                    .drop_column(Users::AvatarPublicId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Username,
    IsAdmin,
    DisplayName,
    Bio,
    AvatarPublicId,
}
