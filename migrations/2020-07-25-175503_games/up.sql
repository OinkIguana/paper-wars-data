CREATE TABLE games (
    id               UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    name             VARCHAR(256)             NOT NULL,
    universe_id      UUID                     NOT NULL REFERENCES universes,
    universe_version INT                      NOT NULL,
    map_id           UUID                     NOT NULL REFERENCES maps,
    map_seed         BIT(256)                 NOT NULL,
    state            JSONB                    NOT NULL DEFAULT '{}'::JSONB,
    created_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    FOREIGN KEY (universe_id, universe_version) REFERENCES universe_versions (universe_id, version),
    FOREIGN KEY (universe_id, universe_version, map_id) REFERENCES universe_version_maps (universe_id, universe_version, map_id)
);
COMMENT ON TABLE games                   IS 'All the games that are being played/have been played.';
COMMENT ON COLUMN games.id               IS 'Unique ID for each game.';
COMMENT ON COLUMN games.name             IS 'User chosen identifier for the game.';
COMMENT ON COLUMN games.universe_id      IS 'The universe in which this game takes place.';
COMMENT ON COLUMN games.universe_version IS 'The version of the universe this game takes place in.';
COMMENT ON COLUMN games.map_id           IS 'The ID of the map this game takes place on.';
COMMENT ON COLUMN games.map_seed         IS 'The seed passed to the map functions.';
COMMENT ON COLUMN games.state            IS 'State information, accessible by the archetype/map scripts and the game engine.';
COMMENT ON COLUMN games.created_at       IS 'The date and time at which this game was started.';

CREATE TYPE player_engagement AS ENUM ('host', 'player', 'pending', 'declined');
COMMENT ON TYPE player_engagement IS 'The engagement of a player in a game.';

CREATE TABLE players (
    game_id    UUID              NOT NULL REFERENCES games ON DELETE CASCADE,
    account_id UUID              NOT NULL REFERENCES accounts,
    turn_order INT               NOT NULL,
    engagement player_engagement NOT NULL,
    state      JSONB             NOT NULL DEFAULT '{}'::JSONB,
    PRIMARY KEY (game_id, account_id),
    UNIQUE (game_id, turn_order)
);
COMMENT ON TABLE players             IS 'The accounts who are part of each game.';
COMMENT ON COLUMN players.game_id    IS 'The game the player is a part of.';
COMMENT ON COLUMN players.turn_order IS 'The turn order of this player during the game.';
COMMENT ON COLUMN players.account_id IS 'The ID of the account that is playing this game.';
COMMENT ON COLUMN players.state      IS 'State information, accessible to the archetype/map scripts and the game engine.';

CREATE FUNCTION delete_player_games_procedure() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM games WHERE EXISTS (
        SELECT 1 FROM players
            WHERE game_id = games.id
            AND account_id = OLD.account_id
    );
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_player_games
    AFTER DELETE ON accounts
    FOR EACH ROW
    EXECUTE PROCEDURE delete_player_games_procedure();
COMMENT ON TRIGGER delete_player_games ON accounts IS 'Delete the games where a deleted account is one of the players.';

CREATE FUNCTION delete_declined_games_procedure() RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM games WHERE NOT EXISTS (
        SELECT 1 FROM players 
            WHERE engagement = 'host'
            OR engagement = 'player'
    );
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_declined_games
    AFTER UPDATE OF engagement ON players
    FOR EACH ROW
    WHEN (NEW.engagement = 'declined')
    EXECUTE PROCEDURE delete_declined_games_procedure();
COMMENT ON TRIGGER delete_declined_games ON players IS 'Delete games where all players have declined/acknowledged that the game has been declined.';

CREATE TABLE entities (
    id           UUID  PRIMARY KEY DEFAULT gen_random_uuid(),
    game_id      UUID  NOT NULL REFERENCES games ON DELETE CASCADE,
    archetype_id UUID  NOT NULL REFERENCES archetypes,
    account_id   UUID           REFERENCES accounts,
    state        JSONB NOT NULL DEFAULT '{}'::JSONB,
    FOREIGN KEY (game_id, account_id) REFERENCES players
);
COMMENT ON TABLE entities               IS 'The entities, created from archetypes, which are currently active in the game.';
COMMENT ON COLUMN entities.id           IS 'Unique ID for each entity.';
COMMENT ON COLUMN entities.game_id      IS 'The game the entity is a part of.';
COMMENT ON COLUMN entities.archetype_id IS 'The ID of the archetype which this entity is an instance of.';
COMMENT ON COLUMN entities.account_id   IS 'The ID of the player that this entity belongs to, if any.';
COMMENT ON COLUMN entities.state        IS 'State information, accessible to the archetype/map scripts and the game engine.';
