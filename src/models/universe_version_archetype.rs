use crate::{universe_version_archetypes, Archetype, Universe};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_version_archetypes"]
#[primary_key(universe_id, universe_version, archetype_id)]
#[belongs_to(Universe)]
#[belongs_to(Archetype)]
pub struct UniverseVersionArchetype {
    universe_id: Uuid,
    universe_version: i32,
    archetype_id: Uuid,
    archetype_version: i32,
}
