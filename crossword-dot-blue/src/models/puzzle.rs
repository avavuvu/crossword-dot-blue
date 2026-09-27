use crossword_tools::puzzle::Puzzle;
use sea_orm::{ActiveValue::Set, SqlErr, entity::prelude::*};
use serde::Deserialize;

use crate::{config, error::{AppError, AppResult}, ids};

use super::{category::Category, region::Region, user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "puzzles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub key: String,
    pub author_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,

    #[sea_orm(column_type = "JsonBinary")]
    pub content: Json,
    #[sea_orm(column_type = "Text")]
    pub xd: String,

    pub title: Option<String>,
    pub notes: Option<String>,
    pub difficulty: Option<i16>,
    pub region: Option<Region>,
    pub themed: bool,
    pub is_cryptic: bool,
    pub is_public: bool,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub featured_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub share_token: Option<String>,
}

pub const KEY_LEN: usize = 8;

pub fn get_category(width: usize, height: usize) -> Category {
    match width.max(height) {
        0..=7 => Category::Mini,
        ..=13 => Category::Midi,
        _ => Category::Big,
    }
}

pub fn is_key(text: &str) -> bool {
    ids::is_hex(text, KEY_LEN)
}

pub fn key_from_slug(slug: &str) -> Option<&str> {
    let key = slug.rsplit('-').next()?;
    is_key(key).then_some(key)
}

fn kind(themed: bool) -> &'static str {
    if themed { "Themed" } else { "Themeless" }
}

pub fn display_title(title: Option<&str>, themed: bool, width: usize, height: usize) -> String {
    match title.map(str::trim).filter(|title| !title.is_empty()) {
        Some(title) => title.to_string(),
        None => format!("{} {width}×{height}", kind(themed)),
    }
}

pub fn slug_for(title: Option<&str>, themed: bool, width: usize, height: usize, username: &str, key: &str) -> String {
    let title = title
        .map(|title| slugify::slugify(title, "", "-", None))
        .filter(|title| !title.is_empty());

    match title {
        Some(title) => format!("{title}-{key}"),
        None => format!("{}-{width}x{height}-{username}-{key}", kind(themed).to_ascii_lowercase()),
    }
}

pub async fn create(db: &DatabaseConnection, author_id: &str, parsed: &Puzzle) -> AppResult<Model> {
    loop {
        match new(author_id, parsed)?.insert(db).await {
            Ok(model) => return Ok(model),
            Err(error) if matches!(error.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => continue,
            Err(error) => return Err(error.into()),
        }
    }
}

fn new(author_id: &str, parsed: &Puzzle) -> Result<ActiveModel, serde_json::Error> {
    let now = chrono::Utc::now().fixed_offset();
    let id = ids::new();

    Ok(ActiveModel {
        key: Set(id[..KEY_LEN].to_string()),
        id: Set(id),
        author_id: Set(author_id.to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        content: Set(serde_json::to_value(parsed)?),
        xd: Set(crossword_tools::xd::write::write_xd(parsed)),
        title: Set(parsed.meta.title.clone()),
        notes: Set(parsed.meta.notes.clone()),
        share_token: Set(Some(ids::new())),
        ..Default::default()
    })
}

#[derive(Deserialize)]
struct Dimensions {
    width: usize,
    height: usize,
}

impl Model {
    pub fn display_title(&self) -> String {
        let (width, height) = self.dimensions();
        display_title(self.title.as_deref(), self.themed, width, height)
    }

    pub fn slug(&self, username: &str) -> String {
        let (width, height) = self.dimensions();
        slug_for(self.title.as_deref(), self.themed, width, height, username, &self.key)
    }

    pub fn path(&self, username: &str) -> String {
        format!("/crossword/{}", self.slug(username))
    }

    pub fn edit_path(&self) -> String {
        format!("/app/edit/{}", self.key)
    }

    pub fn url(&self, username: &str) -> String {
        format!("{}{}", config::get().site_url, self.path(username))
    }

    pub fn share_url(&self, username: &str) -> Option<String> {
        self.share_token.as_ref().map(|token| format!("{}?share={token}", self.url(username)))
    }

    pub fn link(&self, username: &str) -> Option<String> {
        if self.is_public { Some(self.url(username)) } else { self.share_url(username) }
    }

    pub fn accepts_share(&self, token: Option<&str>) -> bool {
        matches!((&self.share_token, token), (Some(stored), Some(given)) if stored == given)
    }

    pub fn category(&self) -> Category {
        let (width, height) = self.dimensions();
        get_category(width, height)
    }

    pub fn is_featured(&self) -> bool {
        self.featured_at.is_some()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        match serde_json::from_value::<Dimensions>(self.content.clone()) {
            Ok(dimensions) => (dimensions.width, dimensions.height),
            Err(error) => {
                eprintln!("[puzzle] {} has no dimensions: {error}", self.key);
                (0, 0)
            }
        }
    }

    pub fn puzzle(&self) -> Result<Puzzle, serde_json::Error> {
        serde_json::from_value(self.content.clone())
    }

    pub fn is_playable_by(&self, viewer_id: Option<&str>, share: Option<&str>) -> bool {
        self.is_public || viewer_id == Some(self.author_id.as_str()) || self.accepts_share(share)
    }
}

pub async fn find_with_author(db: &DatabaseConnection, key: &str) -> AppResult<(Model, user::Model)> {
    if !is_key(key) {
        return Err(AppError::NotFound);
    }

    match Entity::find().filter(Column::Key.eq(key)).find_also_related(user::Entity).one(db).await? {
        Some((model, Some(author))) => Ok((model, author)),
        _ => Err(AppError::NotFound),
    }
}

pub async fn load_playable(
    db: &DatabaseConnection,
    key: &str,
    viewer_id: Option<&str>,
    share: Option<&str>,
) -> AppResult<(Model, user::Model)> {
    let (model, author) = find_with_author(db, key).await?;
    if !model.is_playable_by(viewer_id, share) {
        return Err(AppError::NotFound);
    }

    Ok((model, author))
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::progress::Entity")]
    Progress,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Progress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
