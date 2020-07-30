use super::DbConnection;
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::query_dsl::methods::LoadQuery;

pub trait TryAsQuery<'a, U> {
    type SqlType;
    type Query: AsQuery + RunQueryDsl<DbConnection> + LoadQuery<DbConnection, U> + 'a;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query>;
}

#[async_trait::async_trait]
pub trait Searchable: Sized {
    type Search: for<'a> TryAsQuery<'a, Self>;
    fn cursor(
        &self,
        search: &Self::Search,
        index: usize,
    ) -> String;
}
