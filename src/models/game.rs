use crate::{games, Map, Universe};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "games"]
#[belongs_to(Universe)]
#[belongs_to(Map)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub universe_id: Uuid,
    pub universe_version: i32,
    pub map_id: Uuid,
    pub map_seed: Vec<u8>,
    pub state: Value,
    pub created_at: DateTime<Utc>,
}
