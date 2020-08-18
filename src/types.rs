use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::query_builder::QueryId;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::SqlType;
use juniper::GraphQLEnum;
use std::io::Write;

#[allow(non_camel_case_types)]
#[derive(SqlType, QueryId)]
#[postgres(type_name = "contributor_role")]
pub struct Contributor_role;

#[derive(Copy, Clone, Eq, PartialEq, Debug, FromSqlRow, AsExpression, GraphQLEnum)]
#[sql_type = "Contributor_role"]
pub enum ContributorRole {
    Owner,
    Contributor,
    Pending,
    Declined,
}

impl FromSql<Contributor_role, Pg> for ContributorRole {
    fn from_sql(bytes: diesel::backend::RawValue<Pg>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"owner" => Ok(Self::Owner),
            b"contributor" => Ok(Self::Contributor),
            b"pending" => Ok(Self::Pending),
            b"declined" => Ok(Self::Declined),
            v => Err(format!(
                "Unrecognized contributor_role variant: '{}'",
                String::from_utf8_lossy(v)
            )
            .into()),
        }
    }
}

impl ToSql<Contributor_role, Pg> for ContributorRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        match *self {
            Self::Owner => out.write_all(b"owner")?,
            Self::Contributor => out.write_all(b"contributor")?,
            Self::Pending => out.write_all(b"pending")?,
            Self::Declined => out.write_all(b"declined")?,
        }
        Ok(IsNull::No)
    }
}

#[allow(non_camel_case_types)]
#[derive(SqlType, QueryId)]
#[postgres(type_name = "player_engagement")]
pub struct Player_engagement;

#[derive(Copy, Clone, Eq, PartialEq, Debug, FromSqlRow, AsExpression, GraphQLEnum)]
#[sql_type = "Player_engagement"]
pub enum PlayerEngagement {
    Host,
    Player,
    Pending,
    Declined,
}

impl FromSql<Player_engagement, Pg> for PlayerEngagement {
    fn from_sql(bytes: diesel::backend::RawValue<Pg>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"host" => Ok(Self::Host),
            b"player" => Ok(Self::Player),
            b"pending" => Ok(Self::Pending),
            b"declined" => Ok(Self::Declined),
            v => Err(format!(
                "Unrecognized player_engagement variant: '{}'",
                String::from_utf8_lossy(v)
            )
            .into()),
        }
    }
}

impl ToSql<Player_engagement, Pg> for PlayerEngagement {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        match *self {
            Self::Host => out.write_all(b"host")?,
            Self::Player => out.write_all(b"player")?,
            Self::Pending => out.write_all(b"pending")?,
            Self::Declined => out.write_all(b"declined")?,
        }
        Ok(IsNull::No)
    }
}

impl PlayerEngagement {
    pub fn any() -> Vec<Self> {
        vec![Self::Host, Self::Player, Self::Pending, Self::Declined]
    }
}
