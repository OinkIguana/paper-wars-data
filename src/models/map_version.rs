use crate::{map_versions, Map};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "map_versions"]
#[primary_key(map_id, version)]
#[belongs_to(Map)]
pub struct MapVersion {
    pub map_id: Uuid,
    pub version: i32,
    pub script: String,
    pub created_at: DateTime<Utc>,
}
