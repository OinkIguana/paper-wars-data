use crate::{map_versions, Map};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "map_versions"]
#[primary_key(map_id, version)]
#[belongs_to(Map)]
pub struct MapVersion {
    map_id: Uuid,
    version: i32,
    script: String,
    created_at: DateTime<Utc>,
}
