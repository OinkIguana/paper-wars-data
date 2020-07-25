use crate::{emails, Account};
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[primary_key(address)]
#[table_name = "emails"]
#[belongs_to(Account)]
pub struct Email {
    address: CiString,
    account_id: Uuid,
    verified_at: Option<DateTime<Utc>>,
    protected_until: DateTime<Utc>,
    created_at: DateTime<Utc>,
}
