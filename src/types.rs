use diesel_derive_enum::DbEnum;

#[derive(Copy, Clone, Eq, PartialEq, Debug, DbEnum)]
#[DieselType = "Contributor_role"]
#[PgType = "contributor_role"]
pub enum ContributorRole {
    Owner,
    Contributor,
}
