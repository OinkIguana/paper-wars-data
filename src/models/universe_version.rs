use crate::{universe_versions, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_versions"]
#[primary_key(universe_id, version)]
#[belongs_to(Universe)]
pub struct UniverseVersion {
    pub universe_id: Uuid,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub released_at: DateTime<Utc>,
}
