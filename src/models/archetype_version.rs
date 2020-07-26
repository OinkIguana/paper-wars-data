use crate::{archetype_versions, Archetype};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "archetype_versions"]
#[primary_key(archetype_id, version)]
#[belongs_to(Archetype)]
pub struct ArchetypeVersion {
    pub archetype_id: Uuid,
    pub version: i32,
    pub script: String,
    pub created_at: DateTime<Utc>,
}
