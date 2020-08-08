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

impl ContributorRole {
    /// Whether the account is allowed to contribute to this universe, or if this
    /// relationship does not permit that.
    pub fn can_contribute(self) -> bool {
        match self {
            ContributorRole::Owner | ContributorRole::Contributor => true,
            _ => false,
        }
    }
}
