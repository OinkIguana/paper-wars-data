use crate::universes;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "universes"]
pub struct Universe {
    pub id: Uuid,
    pub name: CiString,
    pub created_at: DateTime<Utc>,
}
