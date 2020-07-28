use diesel_derive_enum::DbEnum;
use juniper::GraphQLEnum;

#[derive(Copy, Clone, Eq, PartialEq, Debug, DbEnum, GraphQLEnum)]
#[DieselType = "Contributor_role"]
#[PgType = "contributor_role"]
pub enum ContributorRole {
    Owner,
    Contributor,
}
