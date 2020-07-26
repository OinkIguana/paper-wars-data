use crate::{contributors, Account, ContributorRole, Universe};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[primary_key(universe_id, account_id)]
#[table_name = "contributors"]
#[belongs_to(Universe)]
#[belongs_to(Account)]
pub struct Contributor {
    pub universe_id: Uuid,
    pub account_id: Uuid,
    pub role: ContributorRole,
    pub created_at: DateTime<Utc>,
}
