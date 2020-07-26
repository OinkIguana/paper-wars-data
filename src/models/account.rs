use crate::accounts;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "accounts"]
pub struct Account {
    pub id: Uuid,
    pub name: CiString,
    pub created_at: DateTime<Utc>,
}
