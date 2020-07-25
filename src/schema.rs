table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    accounts (id) {
        id -> Uuid,
        name -> Citext,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    archetype_versions (archetype_id, version) {
        archetype_id -> Uuid,
        version -> Int4,
        script -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    archetypes (id) {
        id -> Uuid,
        universe_id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    contributors (universe_id, account_id) {
        universe_id -> Uuid,
        account_id -> Uuid,
        role -> Contributor_role,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    emails (address) {
        address -> Citext,
        account_id -> Uuid,
        verified_at -> Nullable<Timestamptz>,
        protected_until -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    entities (id) {
        id -> Uuid,
        game_id -> Uuid,
        archetype_id -> Uuid,
        account_id -> Nullable<Uuid>,
        state -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    games (id) {
        id -> Uuid,
        name -> Varchar,
        universe_id -> Uuid,
        universe_version -> Int4,
        map_id -> Uuid,
        map_seed -> Bpchar,
        state -> Jsonb,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    logins (account_id) {
        account_id -> Uuid,
        email_address -> Citext,
        password -> Bpchar,
        disabled_until -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    map_versions (map_id, version) {
        map_id -> Uuid,
        version -> Int4,
        script -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    maps (id) {
        id -> Uuid,
        universe_id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    password_resets (id) {
        id -> Uuid,
        account_id -> Uuid,
        created_at -> Timestamptz,
        valid_until -> Timestamptz,
        consumed -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    players (game_id, account_id) {
        game_id -> Uuid,
        turn_order -> Int4,
        account_id -> Uuid,
        state -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    universe_version_archetypes (universe_id, universe_version, archetype_id) {
        universe_id -> Uuid,
        universe_version -> Int4,
        archetype_id -> Uuid,
        archetype_version -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    universe_version_maps (universe_id, universe_version, map_id) {
        universe_id -> Uuid,
        universe_version -> Int4,
        map_id -> Uuid,
        map_version -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    universe_versions (universe_id, version) {
        universe_id -> Uuid,
        version -> Int4,
        created_at -> Timestamptz,
        released_at -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;
    use crate::types::*;

    universes (id) {
        id -> Uuid,
        name -> Citext,
        created_at -> Timestamptz,
    }
}

joinable!(archetype_versions -> archetypes (archetype_id));
joinable!(archetypes -> universes (universe_id));
joinable!(contributors -> accounts (account_id));
joinable!(contributors -> universes (universe_id));
joinable!(emails -> accounts (account_id));
joinable!(entities -> accounts (account_id));
joinable!(entities -> archetypes (archetype_id));
joinable!(entities -> games (game_id));
joinable!(games -> maps (map_id));
joinable!(games -> universes (universe_id));
joinable!(logins -> accounts (account_id));
joinable!(map_versions -> maps (map_id));
joinable!(maps -> universes (universe_id));
joinable!(password_resets -> accounts (account_id));
joinable!(players -> accounts (account_id));
joinable!(players -> games (game_id));
joinable!(universe_version_archetypes -> archetypes (archetype_id));
joinable!(universe_version_archetypes -> universes (universe_id));
joinable!(universe_version_maps -> maps (map_id));
joinable!(universe_version_maps -> universes (universe_id));
joinable!(universe_versions -> universes (universe_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    archetype_versions,
    archetypes,
    contributors,
    emails,
    entities,
    games,
    logins,
    map_versions,
    maps,
    password_resets,
    players,
    universe_version_archetypes,
    universe_version_maps,
    universe_versions,
    universes,
);
