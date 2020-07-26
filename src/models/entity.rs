use crate::{entities, Account, Archetype, Game};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "entities"]
#[belongs_to(Archetype)]
#[belongs_to(Game)]
#[belongs_to(Account)]
pub struct Entity {
    pub id: Uuid,
    pub game_id: Uuid,
    pub archetype_id: Uuid,
    pub account_id: Option<Uuid>,
    pub state: Value,
}
