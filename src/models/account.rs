use crate::{accounts, emails, Searchable, TryAsQuery};
use chrono::{DateTime, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "accounts"]
pub struct Account {
    pub id: Uuid,
    pub name: CiString,
    pub created_at: DateTime<Utc>,
}

#[derive(Default, juniper::GraphQLInputObject)]
pub struct AccountSearch {
    name: Option<String>,
    email: Option<String>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl<'a> TryAsQuery<'a, Account> for AccountSearch {
    type SqlType = accounts::SqlType;
    type Query = accounts::BoxedQuery<'a, diesel::pg::Pg>;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query> {
        let mut query = accounts::table.into_boxed();
        if let Some(search_name) = &self.name {
            query = query.filter(accounts::name.like(CiString::from(format!("%{}%", search_name))));
        }
        if let Some(search_email) = &self.email {
            query = query.filter(exists(
                emails::table
                    .filter(emails::address.eq(CiString::from(search_email.as_str())))
                    .filter(emails::account_id.eq(accounts::id)),
            ));
        }
        query = query.limit(self.limit.unwrap_or(25) as i64);
        if let Some(offset) = &self.cursor {
            query = query.offset(offset.parse()?);
        }
        Ok(query)
    }
}

impl Searchable for Account {
    type Search = AccountSearch;

    fn cursor(&self, search: &Self::Search, index: usize) -> String {
        let previous = search
            .cursor
            .as_ref()
            .and_then(|cursor| cursor.parse().ok())
            .unwrap_or(0);
        (previous + index).to_string()
    }
}
