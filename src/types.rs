use diesel_derive_enum::DbEnum;
use juniper::GraphQLEnum;

#[derive(Copy, Clone, Eq, PartialEq, Debug, DbEnum, GraphQLEnum)]
#[DieselType = "Contributor_role"]
#[PgType = "contributor_role"]
pub enum ContributorRole {
    Owner,
    Contributor,
    Pending,
    Declined,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, DbEnum, GraphQLEnum)]
#[DieselType = "Player_engagement"]
#[PgType = "player_engagement"]
pub enum PlayerEngagement {
    Host,
    Player,
    Pending,
    Declined,
}

impl PlayerEngagement {
    pub fn any() -> Vec<Self> {
        vec![Self::Host, Self::Player, Self::Pending, Self::Declined]
    }
}
