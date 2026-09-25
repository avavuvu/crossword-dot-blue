use crossword_tools::puzzle::Puzzle;
use sea_orm::entity::prelude::*;

pub use super::category::Category;
pub use super::region::Region;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "puzzles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub author_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,

    pub content: Json,
    #[sea_orm(column_type = "Text")]
    pub xd: String,

    pub title: Option<String>,
    pub notes: Option<String>,
    pub difficulty: Option<i16>,
    #[sea_orm(column_type = "String(StringLen::N(16))", nullable)]
    pub region: Option<Region>,
    pub themed: bool,
    pub is_cryptic: bool,
    pub is_public: bool,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub featured_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub share_token: Option<String>,
}

pub const MINI_MAX: usize = 7;
pub const MIDI_MAX: usize = 13;

pub fn get_category(width: usize, height: usize) -> Category {
    match width.max(height) {
        0..=MINI_MAX => Category::Mini,
        ..=MIDI_MAX => Category::Midi,
        _ => Category::Big,
    }
}

pub const KEY_LEN: usize = 8;

pub fn new_id() -> String {
    boutique::uuid::Uuid::new_v4().simple().to_string()
}

pub fn new_share_token() -> String {
    boutique::uuid::Uuid::new_v4().simple().to_string()
}

pub fn is_key(text: &str) -> bool {
    text.len() == KEY_LEN && text.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn key_from_slug(slug: &str) -> Option<&str> {
    let key = slug.rsplit('-').next()?;
    is_key(key).then_some(key)
}

fn kind(themed: bool) -> &'static str {
    if themed { "Themed" } else { "Themeless" }
}

pub fn display_title(title: Option<&str>, themed: bool, width: usize, height: usize) -> String {
    match title.map(str::trim).filter(|t| !t.is_empty()) {
        Some(title) => title.to_string(),
        None => format!("{} {width}×{height}", kind(themed)),
    }
}

pub fn slug_for(title: Option<&str>, themed: bool, width: usize, height: usize, username: &str, key: &str) -> String {
    let title = title
        .map(|t| slugify::slugify(t, "", "-", None))
        .filter(|t| !t.is_empty());

    match title {
        Some(title) => format!("{title}-{key}"),
        None => format!("{}-{width}x{height}-{username}-{key}", kind(themed).to_ascii_lowercase()),
    }
}

impl Model {
    pub fn key(&self) -> &str {
        &self.id[..KEY_LEN]
    }

    pub fn display_title(&self) -> String {
        let (width, height) = self.dimensions();
        display_title(self.title.as_deref(), self.themed, width, height)
    }

    pub fn slug(&self, username: &str) -> String {
        let (width, height) = self.dimensions();
        slug_for(self.title.as_deref(), self.themed, width, height, username, self.key())
    }

    pub fn path(&self, username: &str) -> String {
        format!("/crossword/{}", self.slug(username))
    }

    pub fn edit_path(&self) -> String {
        format!("/app/edit/{}", self.key())
    }

    pub fn url(&self, username: &str) -> String {
        format!("{}{}", crate::site_url(), self.path(username))
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
        let read = |field: &str| self.content.get(field).and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        (read("width"), read("height"))
    }

    pub fn puzzle(&self) -> Result<Puzzle, serde_json::Error> {
        serde_json::from_value(self.content.clone())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
