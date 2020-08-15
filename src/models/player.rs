use crate::{players, Account, Game, PlayerEngagement, Searchable, TryAsQuery};
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "players"]
#[primary_key(game_id, account_id)]
#[belongs_to(Game)]
#[belongs_to(Account)]
pub struct Player {
    pub game_id: Uuid,
    pub account_id: Uuid,
    pub turn_order: i32,
    pub engagement: PlayerEngagement,
    pub state: Value,
}

#[derive(Default, juniper::GraphQLInputObject)]
pub struct PlayerSearch {
    pub engagement: Option<Vec<PlayerEngagement>>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl<'a> TryAsQuery<'a, Player> for PlayerSearch {
    type SqlType = players::SqlType;
    type Query = players::BoxedQuery<'a, diesel::pg::Pg>;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query> {
        let mut query = players::table.into_boxed();
        if let Some(engagements) = &self.engagement {
            for engagement in engagements {
                query = query.or_filter(players::engagement.eq(*engagement));
            }
        }
        query = query.limit(self.limit.unwrap_or(25) as i64);
        if let Some(offset) = &self.cursor {
            query = query.offset(offset.parse()?);
        }
        Ok(query)
    }
}

impl Searchable for Player {
    type Search = PlayerSearch;

    fn cursor(&self, search: &Self::Search, index: usize) -> String {
        let previous = search
            .cursor
            .as_ref()
            .and_then(|cursor| cursor.parse().ok())
            .unwrap_or(0);
        (previous + index).to_string()
    }
}
