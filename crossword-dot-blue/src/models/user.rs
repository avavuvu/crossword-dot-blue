use boutique::AuthUser;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::uuid;

use crate::cloudinary;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub display_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub bio: Option<String>,
    pub avatar_public_id: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

pub const AVATAR_TRANSFORM: &str = "c_fill,g_face,w_256,h_256,f_auto,q_auto";

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl AuthUser for Model {
    type UserEntity = Entity;
    type UserColumn = Column;
    type Active = ActiveModel;

    const ID: Column = Column::Id;
    const EMAIL: Column = Column::Email;
    const PASSWORD: Column = Column::Password;

    fn id(&self) -> &str { &self.id }
    fn email(&self) -> &str { &self.email }
    fn password_hash(&self) -> &str { &self.password }
}

impl Model {
    pub fn display_name(&self) -> &str {
        self.display_name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or(&self.username)
    }

    pub fn path(&self) -> String {
        format!("/@{}", self.username)
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.avatar_public_id
            .as_deref()
            .map(|public_id| cloudinary::url(public_id, AVATAR_TRANSFORM))
    }

    pub fn avatar_public_id(&self) -> String {
        format!("avatars/{}", self.id)
    }
}

pub fn new(email: &str, username: &str, plain_password: &str, is_admin: bool) -> Result<ActiveModel, boutique::password::argon2::password_hash::Error> {
    Ok(ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(email.to_string()),
        username: Set(username.to_string()),
        password: Set(boutique::password::hash(plain_password)?),
        is_admin: Set(is_admin),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    })
}
