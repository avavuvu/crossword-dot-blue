use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder};

use super::{Entry, query::load};
use crate::models::{category::Category, puzzle};

#[derive(Default)]
pub struct Featured {
    pub big: Option<Entry>,
    pub midi: Option<Entry>,
    pub mini: Option<Entry>,
}

impl Featured {
    pub fn get(&self, category: Category) -> Option<&Entry> {
        match category {
            Category::Big => self.big.as_ref(),
            Category::Midi => self.midi.as_ref(),
            Category::Mini => self.mini.as_ref(),
        }
    }

    fn slot(&mut self, category: Category) -> &mut Option<Entry> {
        match category {
            Category::Big => &mut self.big,
            Category::Midi => &mut self.midi,
            Category::Mini => &mut self.mini,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.big.is_none() && self.midi.is_none() && self.mini.is_none()
    }
}

pub async fn featured(db: &DatabaseConnection) -> Result<Featured, DbErr> {
    let query = puzzle::Entity::find()
        .filter(puzzle::Column::IsPublic.eq(true))
        .filter(puzzle::Column::FeaturedAt.is_not_null())
        .order_by_desc(puzzle::Column::FeaturedAt);

    let mut featured = Featured::default();
    for entry in load(query, db).await? {
        let slot = featured.slot(entry.model.category());
        if slot.is_none() {
            *slot = Some(entry);
        }
    }
    Ok(featured)
}
