use crate::{archetype_versions, Archetype};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "archetype_versions"]
#[primary_key(archetype_id, version)]
#[belongs_to(Archetype)]
pub struct ArchetypeVersion {
    archetype_id: Uuid,
    version: i32,
    script: String,
    created_at: DateTime<Utc>,
}
