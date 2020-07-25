use crate::{games, Map, Universe};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "games"]
#[belongs_to(Universe)]
#[belongs_to(Map)]
pub struct Game {
    id: Uuid,
    name: String,
    universe_id: Uuid,
    universe_version: i32,
    map_id: Uuid,
    map_seed: Vec<u8>,
    state: Value,
    created_at: DateTime<Utc>,
}
