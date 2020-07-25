use crate::{universe_version_maps, Map, Universe};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_version_maps"]
#[primary_key(universe_id, universe_version, map_id)]
#[belongs_to(Universe)]
#[belongs_to(Map)]
pub struct UniverseVersionMap {
    universe_id: Uuid,
    universe_version: i32,
    map_id: Uuid,
    map_version: i32,
}
