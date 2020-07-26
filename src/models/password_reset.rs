use crate::{password_resets, Account};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "password_resets"]
#[belongs_to(Account)]
pub struct PasswordReset {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub consumed: bool,
}
