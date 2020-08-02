use crate::{universes, Searchable, TryAsQuery};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "universes"]
pub struct Universe {
    pub id: Uuid,
    pub name: CiString,
    pub created_at: DateTime<Utc>,
}

#[derive(Default, juniper::GraphQLInputObject)]
pub struct UniverseSearch {
    pub name: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl<'a> TryAsQuery<'a, Universe> for UniverseSearch {
    type SqlType = universes::SqlType;
    type Query = universes::BoxedQuery<'a, diesel::pg::Pg>;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query> {
        let mut query = universes::table.into_boxed();
        if let Some(search_name) = &self.name {
            query =
                query.filter(universes::name.like(CiString::from(format!("%{}%", search_name))));
        }
        query = query.limit(self.limit.unwrap_or(25) as i64);
        if let Some(offset) = &self.cursor {
            query = query.offset(offset.parse()?);
        }
        Ok(query)
    }
}

impl Searchable for Universe {
    type Search = UniverseSearch;

    fn cursor(&self, search: &Self::Search, index: usize) -> String {
        let previous = search
            .cursor
            .as_ref()
            .and_then(|cursor| cursor.parse().ok())
            .unwrap_or(0);
        (previous + index).to_string()
    }
}
