use axum::{
    Extension,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use boutique::UserContext;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::{listing::{self, Filter}, user},
    views::{self, viewer::Viewer},
};

pub async fn show(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Path(username): Path<String>,
    Query(filter): Query<Filter>,
) -> AppResult {
    let profile = user::Entity::find()
        .filter(user::Column::Username.eq(&username))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let all = listing::by_author(&state.db, &profile.id).await?;
    let regions = listing::regions(&all);
    let entries = filter.apply(all);

    Ok(views::profile::page(&Viewer::from(&ctx), &profile, &filter, &regions, &entries).into_response())
}
