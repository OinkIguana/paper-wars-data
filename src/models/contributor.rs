use crate::{contributors, Account, ContributorRole, Searchable, TryAsQuery, Universe};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[primary_key(universe_id, account_id)]
#[table_name = "contributors"]
#[belongs_to(Universe)]
#[belongs_to(Account)]
pub struct Contributor {
    pub universe_id: Uuid,
    pub account_id: Uuid,
    pub role: ContributorRole,
    pub created_at: DateTime<Utc>,
}

#[derive(Default, juniper::GraphQLInputObject)]
pub struct ContributorSearch {
    role: Option<Vec<ContributorRole>>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl ContributorSearch {
    pub fn for_account(self, account_id: Uuid) -> ScopedContributorSearch {
        ScopedContributorSearch {
            scope: ContributorSearchScope::Account(account_id),
            search: self,
        }
    }

    pub fn for_universe(self, universe_id: Uuid) -> ScopedContributorSearch {
        ScopedContributorSearch {
            scope: ContributorSearchScope::Universe(universe_id),
            search: self,
        }
    }
}

enum ContributorSearchScope {
    Account(Uuid),
    Universe(Uuid),
}

pub struct ScopedContributorSearch {
    scope: ContributorSearchScope,
    search: ContributorSearch,
}

impl<'a> TryAsQuery<'a, Contributor> for ScopedContributorSearch {
    type SqlType = contributors::SqlType;
    type Query = contributors::BoxedQuery<'a, diesel::pg::Pg>;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query> {
        let mut query = match self.scope {
            ContributorSearchScope::Account(account_id) => contributors::table
                .filter(contributors::account_id.eq(account_id))
                .into_boxed(),
            ContributorSearchScope::Universe(universe_id) => contributors::table
                .filter(contributors::universe_id.eq(universe_id))
                .into_boxed(),
        };
        if let Some(roles) = &self.search.role {
            for role in roles {
                query = query.or_filter(contributors::role.eq(*role));
            }
        }
        query = query.limit(self.search.limit.unwrap_or(25) as i64);
        if let Some(offset) = &self.search.cursor {
            query = query.offset(offset.parse()?);
        }
        Ok(query)
    }
}

impl Searchable for Contributor {
    type Search = ScopedContributorSearch;

    fn cursor(&self, search: &Self::Search, index: usize) -> String {
        let previous = search
            .search
            .cursor
            .as_ref()
            .and_then(|cursor| cursor.parse().ok())
            .unwrap_or(0);
        (previous + index).to_string()
    }
}
