use crate::{entities, Account, Archetype, Game};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "entities"]
#[belongs_to(Archetype)]
#[belongs_to(Game)]
#[belongs_to(Account)]
pub struct Entity {
    id: Uuid,
    game_id: Uuid,
    archetype_id: Uuid,
    account_id: Option<Uuid>,
    state: Value,
}
