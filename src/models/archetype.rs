use crate::{archetypes, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "archetypes"]
#[belongs_to(Universe)]
pub struct Archetype {
    id: Uuid,
    universe_id: Uuid,
    created_at: DateTime<Utc>,
}
