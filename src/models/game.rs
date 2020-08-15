use crate::{
    games, players, Map, PlayerEngagement, PlayerSearch, Searchable, TryAsQuery, Universe,
};
use chrono::{DateTime, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Associations, Queryable, QueryableByName)]
#[table_name = "games"]
#[belongs_to(Universe)]
#[belongs_to(Map)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub universe_id: Uuid,
    pub universe_version: i32,
    pub map_id: Uuid,
    pub map_seed: Vec<u8>,
    pub state: Value,
    pub created_at: DateTime<Utc>,
}

pub struct GamePlayerSearch {
    pub id: Uuid,
    pub engagement: Option<Vec<PlayerEngagement>>,
}

pub struct GameSearch {
    pub player: Option<GamePlayerSearch>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl GameSearch {
    pub fn from_player_search(
        id: Uuid,
        PlayerSearch {
            engagement,
            limit,
            cursor,
        }: PlayerSearch,
    ) -> Self {
        Self {
            player: Some(GamePlayerSearch { id, engagement }),
            limit,
            cursor,
        }
    }
}

impl<'a> TryAsQuery<'a, Game> for GameSearch {
    type SqlType = games::SqlType;
    type Query = games::BoxedQuery<'a, diesel::pg::Pg>;

    fn try_as_query(&'a self) -> anyhow::Result<Self::Query> {
        let mut query = games::table.into_boxed();
        if let Some(player) = &self.player {
            let player_query = players::table
                .filter(players::account_id.eq(player.id))
                .filter(players::game_id.eq(games::id))
                // This is not pretty, but at least it works.
                .filter(players::engagement.eq(any(
                    player.engagement.clone().unwrap_or(PlayerEngagement::any()),
                )));
            query = query.filter(exists(player_query));
        }
        query = query.limit(self.limit.unwrap_or(25) as i64);
        if let Some(offset) = &self.cursor {
            query = query.offset(offset.parse()?);
        }
        Ok(query)
    }
}

impl Searchable for Game {
    type Search = GameSearch;

    fn cursor(&self, search: &Self::Search, index: usize) -> String {
        let previous = search
            .cursor
            .as_ref()
            .and_then(|cursor| cursor.parse().ok())
            .unwrap_or(0);
        (previous + index).to_string()
    }
}
