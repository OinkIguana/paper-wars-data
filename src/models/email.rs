use crate::{emails, Account};
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[primary_key(address)]
#[table_name = "emails"]
#[belongs_to(Account)]
pub struct Email {
    pub address: CiString,
    pub account_id: Uuid,
    pub verified_at: Option<DateTime<Utc>>,
    pub protected_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
