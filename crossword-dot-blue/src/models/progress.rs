use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, TransactionTrait,
    entity::prelude::*,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};

pub const VERSION: i16 = 1;
const COMPLETIONS: [&str; 3] = ["incomplete", "complete-but-wrong", "won"];

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "progress")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub player_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub puzzle_id: String,
    pub user_id: Option<String>,
    pub version: i16,
    #[sea_orm(column_type = "JsonBinary")]
    pub state: Json,
    pub completion: String,
    pub elapsed_ms: i64,
    pub solved_at: Option<DateTimeWithTimeZone>,
    pub saved_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::puzzle::Entity",
        from = "Column::PuzzleId",
        to = "super::puzzle::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Puzzle,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::puzzle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Puzzle.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stored {
    pub version: i16,
    pub puzzle_key: String,
    pub saved_at: i64,
    pub solved_at: Option<i64>,
    pub completion: String,
    pub elapsed_ms: i64,
    #[serde(flatten)]
    pub state: serde_json::Map<String, serde_json::Value>,
}

impl Stored {
    pub fn is_valid_for(&self, puzzle_key: &str) -> bool {
        self.version == VERSION && self.puzzle_key == puzzle_key && COMPLETIONS.contains(&self.completion.as_str())
    }
}

impl Model {
    pub fn stored(&self, puzzle_key: &str) -> Stored {
        Stored {
            version: self.version,
            puzzle_key: puzzle_key.to_string(),
            saved_at: self.saved_at.timestamp_millis(),
            solved_at: self.solved_at.map(|time| time.timestamp_millis()),
            completion: self.completion.clone(),
            elapsed_ms: self.elapsed_ms,
            state: match &self.state {
                serde_json::Value::Object(map) => map.clone(),
                _ => serde_json::Map::new(),
            },
        }
    }
}

pub struct Owner<'a> {
    pub player_id: &'a str,
    pub user_id: Option<&'a str>,
}

impl Owner<'_> {
    fn condition(&self) -> Condition {
        match self.user_id {
            Some(user_id) => Condition::all().add(Column::UserId.eq(user_id)),
            None => Condition::all().add(Column::PlayerId.eq(self.player_id)).add(Column::UserId.is_null()),
        }
    }
}

pub enum Saved {
    Ok { saved_at: i64, solved_at: Option<i64> },
    Stale(Model),
}

pub async fn find_for(db: &impl ConnectionTrait, owner: &Owner<'_>, puzzle_id: &str) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(Column::PuzzleId.eq(puzzle_id))
        .filter(owner.condition())
        .order_by_desc(Column::SavedAt)
        .one(db)
        .await
}

pub async fn save(db: &DatabaseConnection, owner: &Owner<'_>, puzzle_id: &str, stored: Stored) -> Result<Saved, DbErr> {
    let transaction = db.begin().await?;

    let newest = find_for(&transaction, owner, puzzle_id).await?;
    if let Some(newest) = newest.as_ref().filter(|row| row.saved_at.timestamp_millis() > stored.saved_at) {
        return Ok(Saved::Stale(newest.clone()));
    }

    let now = now_ms();
    let solved_at = match stored.completion.as_str() {
        "won" => newest.as_ref().and_then(|row| row.solved_at).or(Some(now)),
        _ => None,
    };

    if let Some(user_id) = owner.user_id {
        Entity::delete_many()
            .filter(Column::PuzzleId.eq(puzzle_id))
            .filter(Column::UserId.eq(user_id))
            .filter(Column::PlayerId.ne(owner.player_id))
            .exec(&transaction)
            .await?;
    }

    let row = ActiveModel {
        player_id: Set(owner.player_id.to_string()),
        puzzle_id: Set(puzzle_id.to_string()),
        user_id: Set(owner.user_id.map(str::to_string)),
        version: Set(stored.version),
        state: Set(serde_json::Value::Object(stored.state)),
        completion: Set(stored.completion),
        elapsed_ms: Set(stored.elapsed_ms.max(0)),
        solved_at: Set(solved_at),
        saved_at: Set(now),
    };

    Entity::insert(row)
        .on_conflict(
            OnConflict::columns([Column::PlayerId, Column::PuzzleId])
                .update_columns([
                    Column::UserId,
                    Column::Version,
                    Column::State,
                    Column::Completion,
                    Column::ElapsedMs,
                    Column::SolvedAt,
                    Column::SavedAt,
                ])
                .to_owned(),
        )
        .exec(&transaction)
        .await?;

    transaction.commit().await?;
    Ok(Saved::Ok { saved_at: now.timestamp_millis(), solved_at: solved_at.map(|time| time.timestamp_millis()) })
}

pub async fn delete_for(db: &DatabaseConnection, owner: &Owner<'_>, puzzle_id: &str) -> Result<(), DbErr> {
    Entity::delete_many()
        .filter(Column::PuzzleId.eq(puzzle_id))
        .filter(owner.condition())
        .exec(db)
        .await?;
    Ok(())
}

pub async fn claim(db: &impl ConnectionTrait, player_id: &str, user_id: &str) -> Result<(), DbErr> {
    let guest_rows = Entity::find()
        .filter(Column::PlayerId.eq(player_id))
        .filter(Column::UserId.is_null())
        .all(db)
        .await?;

    for guest in guest_rows {
        let owner = Owner { player_id, user_id: Some(user_id) };
        let existing = find_for(db, &owner, &guest.puzzle_id).await?;

        match existing {
            Some(other) if other.saved_at >= guest.saved_at => {
                Entity::delete_by_id((guest.player_id, guest.puzzle_id)).exec(db).await?;
            }
            other => {
                if let Some(other) = other {
                    Entity::delete_by_id((other.player_id, other.puzzle_id)).exec(db).await?;
                }
                let mut active: ActiveModel = guest.into();
                active.user_id = Set(Some(user_id.to_string()));
                active.update(db).await?;
            }
        }
    }

    Ok(())
}

fn now_ms() -> DateTime<FixedOffset> {
    let millis = Utc::now().timestamp_millis();
    DateTime::from_timestamp_millis(millis).unwrap_or_default().fixed_offset()
}
