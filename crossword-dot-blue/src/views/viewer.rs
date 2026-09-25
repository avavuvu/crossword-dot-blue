use boutique::UserContext;

use crate::models::user;

#[derive(Clone, Debug, Default)]
pub struct Viewer {
    pub user_id: Option<String>,
}

impl Viewer {
    pub fn guest() -> Self {
        Self::default()
    }

    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

impl From<&UserContext> for Viewer {
    fn from(ctx: &UserContext) -> Self {
        Self { user_id: ctx.user_id.clone() }
    }
}

impl From<&user::Model> for Viewer {
    fn from(user: &user::Model) -> Self {
        Self { user_id: Some(user.id.clone()) }
    }
}
