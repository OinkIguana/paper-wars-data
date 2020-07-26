use crate::{universe_version_maps, Map, Universe};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "universe_version_maps"]
#[primary_key(universe_id, universe_version, map_id)]
#[belongs_to(Universe)]
#[belongs_to(Map)]
pub struct UniverseVersionMap {
    pub universe_id: Uuid,
    pub universe_version: i32,
    pub map_id: Uuid,
    pub map_version: i32,
}
