use crate::universes;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "universes"]
pub struct Universe {
    id: Uuid,
    name: CiString,
    created_at: DateTime<Utc>,
}
