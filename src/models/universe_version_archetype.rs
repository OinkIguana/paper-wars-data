use crate::{universe_version_archetypes, Archetype, Universe};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_version_archetypes"]
#[primary_key(universe_id, universe_version, archetype_id)]
#[belongs_to(Universe)]
#[belongs_to(Archetype)]
pub struct UniverseVersionArchetype {
    pub universe_id: Uuid,
    pub universe_version: i32,
    pub archetype_id: Uuid,
    pub archetype_version: i32,
}
