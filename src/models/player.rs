use crate::{players, Account, Game};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "players"]
#[primary_key(game_id, account_id)]
#[belongs_to(Game)]
#[belongs_to(Account)]
pub struct Player {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub turn_order: i32,
    pub state: Value,
}
