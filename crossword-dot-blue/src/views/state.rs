use boutique::UserContext;

use crate::models::user;

#[derive(Clone, Debug, Default)]
pub struct ViewState {
    pub user_id: Option<String>,
    pub username: Option<String>,
}

impl ViewState {
    pub fn guest() -> Self {
        Self::default()
    }

    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

impl From<&UserContext> for ViewState {
    fn from(ctx: &UserContext) -> Self {
        Self {
            user_id: ctx.user_id.clone(),
            username: None,
        }
    }
}

impl From<&user::Model> for ViewState {
    fn from(user: &user::Model) -> Self {
        Self {
            user_id: Some(user.id.clone()),
            username: Some(user.username.clone()),
        }
    }
}
