use crate::{password_resets, Account};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "password_resets"]
#[belongs_to(Account)]
pub struct PasswordReset {
    id: Uuid,
    account_id: Uuid,
    created_at: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    consumed: bool,
}
