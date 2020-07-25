use crate::accounts;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "accounts"]
pub struct Account {
    id: Uuid,
    name: CiString,
    created_at: DateTime<Utc>,
}
