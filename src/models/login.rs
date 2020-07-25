use crate::{logins, Account, Email};
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[primary_key(account_id)]
#[table_name = "logins"]
#[belongs_to(Account)]
#[belongs_to(Email, foreign_key = "email_address")]
pub struct Login {
    account_id: Uuid,
    email_address: CiString,
    password: Vec<u8>,
    disabled_until: Option<DateTime<Utc>>,
}
