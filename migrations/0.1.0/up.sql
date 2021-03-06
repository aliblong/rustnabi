CREATE TYPE endcon AS ENUM ('inp', 'norm', 'strike', 'time', 'kill');  -- in progress, normal, strikeout, timeout, killed
CREATE TYPE variant AS ENUM ('normal', 'orange', 'black', 'rainbow', 'dual', 'dual_rainbow', 'white_rainbow', 'wild_crazy', 'ambiguous', 'red_blue', 'acid_trip', 'dark_rainbow', 'dark_rainbow_black');

-- https://stackoverflow.com/questions/8443716/postgres-unique-constraint-for-array
--CREATE OR REPLACE FUNCTION sort_ip(INET[])  RETURNS INET[] AS
--$$
--SELECT CASE WHEN $1[1] > $1[2] THEN ARRAY[$1[2], $1[1]] ELSE $1 END;
--$$ LANGUAGE sql IMMUTABLE;

CREATE TABLE users (
    id                   SERIAL        PRIMARY KEY,
    name                 TEXT          NOT NULL  UNIQUE,
    pw                   BYTEA         NOT NULL, -- Hashed and salted with SHA-256
    salt                 BYTEA         NOT NULL, -- 32-byte like the hash
    restart_privilege    BOOLEAN       NOT NULL  DEFAULT false,
    ban_privilege        BOOLEAN       NOT NULL  DEFAULT false,
    datetime_last_login  TIMESTAMP     NOT NULL  DEFAULT NOW(),
    datetime_created     TIMESTAMP     NOT NULL  DEFAULT NOW()
);
CREATE INDEX users_index_name ON users (name);
-- Prevents duplicate IP addresses being added for a given user
--CREATE UNIQUE INDEX unique_ip ON users (sort_ip(ip));

CREATE TABLE ips (
    id  SERIAL  PRIMARY KEY
  , ip  INET  NOT NULL
);

CREATE TABLE user_ips (
    user_id  INT  NOT NULL
  , FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
  , ip_id  INT  NOT NULL
  , FOREIGN KEY(ip_id) REFERENCES ips(id) ON DELETE CASCADE
  , PRIMARY KEY(user_id, ip_id)
);

CREATE TABLE hands_sizes (
    id  SERIAL  PRIMARY KEY
  , hand_sizes  SMALLINT[]  NOT NULL
);

CREATE TABLE games (
    id                 SERIAL        PRIMARY KEY,
    name               TEXT          NOT NULL,
    --num_players        SMALLINT      NOT NULL  DEFAULT 2,
    --players            INT[]         NOT NULL,
    owner              INT           NOT NULL,
    FOREIGN KEY (owner) REFERENCES users (id),
    variant            variant       NOT NULL,
    hand_sizes_id      INT           NOT NULL,
    FOREIGN KEY (hand_sizes_id) REFERENCES hands_sizes(id),
    timed              BOOLEAN       NOT NULL,
    seed               TEXT          NOT NULL, -- like "p2v0s1"
    score              SMALLINT      NOT NULL,
    endcon             endcon        NOT NULL,
    action             jsonb         NOT NULL, /* JSON */
    datetime_created   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_started   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_finished  TIMESTAMP     NOT NULL  DEFAULT NOW()
);
CREATE INDEX games_index_variant ON games (variant);
CREATE INDEX games_index_seed ON games (seed);

CREATE TABLE participants (
    game_id  INT  NOT NULL
  , FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
  , user_id  INT  NOT NULL
  , FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
  , PRIMARY KEY(game_id, user_id)
);

CREATE TABLE game_time_controls (
    id                 INT           NOT NULL  PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES games (id),
    time_base          INT           NOT NULL, -- in seconds
    time_per_turn      INT           NOT NULL -- in seconds
);

CREATE VIEW timed_games AS
SELECT * FROM games NATURAL JOIN game_time_controls;

CREATE TABLE banned_ips (
    id                 INT         PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES ips(id),
    admin_responsible  INT            NOT NULL,
    FOREIGN KEY(admin_responsible) REFERENCES users(id),
    reason             TEXT           NULL      DEFAULT NULL,
    datetime_banned    TIMESTAMP      NOT NULL  DEFAULT NOW()
);

CREATE TABLE variants (
    id  SERIAL  PRIMARY KEY
  , name  TEXT  NOT NULL
);

CREATE TABLE dists (
    id  SERIAL  PRIMARY KEY
  , dist  SMALLINT[]  NOT NULL
);

CREATE TABLE suits (
    id  SERIAL  PRIMARY KEY
  , variant_id  INT  NOT NULL
  , FOREIGN KEY(variant_id) REFERENCES variants(id) ON DELETE CASCADE
  , suit_index  SMALLINT  NOT NULL
  , colors  SMALLINT[]  NOT NULL
  , dist_id  INT  NOT NULL
  , FOREIGN KEY(dist_id) REFERENCES dists(id) ON DELETE CASCADE
  , dist  SMALLINT[]  NOT NULL
);
