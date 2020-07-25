use crate::{universe_versions, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_versions"]
#[primary_key(universe_id, version)]
#[belongs_to(Universe)]
pub struct UniverseVersion {
    universe_id: Uuid,
    version: i32,
    created_at: DateTime<Utc>,
    released_at: DateTime<Utc>,
}
