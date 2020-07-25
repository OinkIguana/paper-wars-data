use crate::{players, Account, Game};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "players"]
#[primary_key(game_id, account_id)]
#[belongs_to(Game)]
#[belongs_to(Account)]
pub struct Player {
    game_id: Uuid,
    account_id: Uuid,
    turn_order: i32,
    state: Value,
}
