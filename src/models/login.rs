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
    pub account_id: Uuid,
    pub email_address: CiString,
    pub password: Vec<u8>,
    pub disabled_until: Option<DateTime<Utc>>,
}
