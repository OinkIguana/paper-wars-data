use crate::{archetypes, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "archetypes"]
#[belongs_to(Universe)]
pub struct Archetype {
    pub id: Uuid,
    pub universe_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
