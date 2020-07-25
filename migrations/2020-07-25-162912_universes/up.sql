CREATE TABLE universes (
    id         UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    name       CITEXT                   UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
COMMENT ON TABLE universes             IS 'Universes which games can be played in. These are typically created and owned by accounts.';
COMMENT ON COLUMN universes.id         IS 'Unique ID for the universe.';
COMMENT ON COLUMN universes.name       IS 'The name of the universe.';
COMMENT ON COLUMN universes.created_at IS 'The date and time at which the universe was created.';

CREATE TABLE universe_versions (
    universe_id UUID                     NOT NULL REFERENCES universes,
    version     INT                      NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    released_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (universe_id, version)
);
COMMENT ON TABLE universe_versions              IS 'Tracks released versions of universes, so existing games can continue without being changed by updates.';
COMMENT ON COLUMN universe_versions.universe_id IS 'The ID of the universe.';
COMMENT ON COLUMN universe_versions.version     IS 'The version number of the universe.';
COMMENT ON COLUMN universe_versions.created_at  IS 'The date and time at which this version was created.';
COMMENT ON COLUMN universe_versions.released_at IS 'The date and time at which this version was released.';

CREATE TABLE archetypes (
    id          UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    universe_id UUID                     NOT NULL REFERENCES universes ON DELETE CASCADE,
    name        VARCHAR(256)             NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (universe_id, name)
);
COMMENT ON TABLE archetypes              IS 'The types of things which exist in each universe.';
COMMENT ON COLUMN archetypes.id          IS 'Unique ID for the archetype.';
COMMENT ON COLUMN archetypes.universe_id IS 'The ID of the universe which this archetype exists in.';
COMMENT ON COLUMN archetypes.name        IS 'The development name of this archetype. Its in-game name comes from the script.';
COMMENT ON COLUMN archetypes.created_at  IS 'The date and time at which this archetype was crated.';

CREATE TABLE archetype_versions (
    archetype_id UUID                     NOT NULL REFERENCES archetypes ON DELETE CASCADE,
    version      INT                      NOT NULL DEFAULT 0,
    script       TEXT                     NOT NULL,
    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (archetype_id, version)
);
COMMENT ON TABLE archetype_versions             IS 'Tracks changes made to the archetype. Once released, the archetype should not change so as not to affect games in progress.';
COMMENT ON COLUMN archetype_versions.version    IS 'The version number.';
COMMENT ON COLUMN archetype_versions.script     IS 'Describes the behaviours and attributes of this archetype.';
COMMENT ON COLUMN archetype_versions.created_at IS 'The date and time at which this version was created.';

CREATE TABLE maps (
    id          UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    universe_id UUID                     NOT NULL REFERENCES universes ON DELETE CASCADE,
    name        VARCHAR(256)             NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (universe_id, name)
);
COMMENT ON TABLE maps              IS 'Maps that games can be played on in each universe.';
COMMENT ON COLUMN maps.id          IS 'Unique ID for the map.';
COMMENT ON COLUMN maps.universe_id IS 'The ID of the universe which this map exists in.';
COMMENT ON COLUMN maps.name        IS 'The development name of this map. Its in-game name comes from the script.';
COMMENT ON COLUMN maps.created_at  IS 'The date and time at which this map was crated.';

CREATE TABLE map_versions (
    map_id     UUID                     NOT NULL REFERENCES maps ON DELETE CASCADE,
    version    INT                      NOT NULL DEFAULT 0,
    script     TEXT                     NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (map_id, version)
);
COMMENT ON TABLE map_versions             IS 'Tracks changes made to the map. Once released, the map should not change so as not to affect games in progress.';
COMMENT ON COLUMN map_versions.version    IS 'The version number.';
COMMENT ON COLUMN map_versions.script     IS 'The function that is used to generate the map.';
COMMENT ON COLUMN map_versions.created_at IS 'The date and time at which this version was created.';

CREATE TABLE universe_version_archetypes (
    universe_id       UUID NOT NULL REFERENCES universes,
    universe_version  INT  NOT NULL,
    archetype_id      UUID NOT NULL REFERENCES archetypes,
    archetype_version INT  NOT NULL,
    FOREIGN KEY (universe_id, universe_version)   REFERENCES universe_versions ON DELETE CASCADE,
    FOREIGN KEY (archetype_id, archetype_version) REFERENCES archetype_versions ON DELETE CASCADE,
    PRIMARY KEY (universe_id, universe_version, archetype_id)
);
COMMENT ON TABLE universe_version_archetypes                    IS 'Lists the archetypes and the versions of those archetypes that exist within each version of the universe.';
COMMENT ON COLUMN universe_version_archetypes.universe_id       IS 'The ID of the universe the entity is in.';
COMMENT ON COLUMN universe_version_archetypes.universe_version  IS 'The version of the universe the entity is part of.';
COMMENT ON COLUMN universe_version_archetypes.archetype_id      IS 'The ID of the archetype in the universe.';
COMMENT ON COLUMN universe_version_archetypes.archetype_version IS 'The version of the archetype that is part of this universe version.';

CREATE TABLE universe_version_maps (
    universe_id       UUID NOT NULL REFERENCES universes,
    universe_version  INT  NOT NULL,
    map_id            UUID NOT NULL REFERENCES maps,
    map_version       INT  NOT NULL,
    FOREIGN KEY (universe_id, universe_version)   REFERENCES universe_versions ON DELETE CASCADE,
    FOREIGN KEY (map_id, map_version) REFERENCES map_versions ON DELETE CASCADE,
    PRIMARY KEY (universe_id, universe_version, map_id)
);
COMMENT ON TABLE universe_version_maps                   IS 'Lists the maps and the versions of those maps that exist within each version of the universe.';
COMMENT ON COLUMN universe_version_maps.universe_id      IS 'The ID of the universe the entity is in.';
COMMENT ON COLUMN universe_version_maps.universe_version IS 'The version of the universe the entity is part of.';
COMMENT ON COLUMN universe_version_maps.map_id           IS 'The ID of the map in the universe.';
COMMENT ON COLUMN universe_version_maps.map_version      IS 'The version of the map that is part of this universe version.';

CREATE TYPE contributor_role AS ENUM ('owner', 'contributor');
COMMENT ON TYPE contributor_role IS 'The relationship between a universe and an account.';

CREATE TABLE contributors (
    universe_id UUID                     NOT NULL REFERENCES universes ON DELETE CASCADE,
    account_id  UUID                     NOT NULL REFERENCES accounts ON DELETE CASCADE,
    role        contributor_role         NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (universe_id, account_id)
);
COMMENT ON TABLE contributors              IS 'Describes the relationships between accounts and the universes which they contribute to.';
COMMENT ON COLUMN contributors.universe_id IS 'The universe in the relationship.';
COMMENT ON COLUMN contributors.account_id  IS 'The account in the relationship.';
COMMENT ON COLUMN contributors.role        IS 'The role the account has in this universe.';
COMMENT ON COLUMN contributors.created_at  IS 'The date and time at which this account was given permission to this universe.';
