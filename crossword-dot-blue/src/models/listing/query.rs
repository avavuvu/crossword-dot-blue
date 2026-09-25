use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, Select};

use super::Entry;
use crate::models::{puzzle, user};

pub(super) fn public_query() -> Select<puzzle::Entity> {
    puzzle::Entity::find()
        .filter(puzzle::Column::IsPublic.eq(true))
        .order_by_desc(puzzle::Column::PublishedAt)
        .order_by_desc(puzzle::Column::UpdatedAt)
}

pub(super) async fn load(query: Select<puzzle::Entity>, db: &DatabaseConnection) -> Result<Vec<Entry>, DbErr> {
    let rows = query.find_also_related(user::Entity).all(db).await?;
    Ok(rows
        .into_iter()
        .filter_map(|(model, author)| author.map(|author| Entry { model, author }))
        .collect())
}

pub async fn public(db: &DatabaseConnection) -> Result<Vec<Entry>, DbErr> {
    load(public_query(), db).await
}

pub async fn by_author(db: &DatabaseConnection, author_id: &str) -> Result<Vec<Entry>, DbErr> {
    load(public_query().filter(puzzle::Column::AuthorId.eq(author_id)), db).await
}
