use crate::{maps, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "maps"]
#[belongs_to(Universe)]
pub struct Map {
    id: Uuid,
    universe_id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}
