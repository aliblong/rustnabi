CREATE TYPE endcon AS ENUM ('Inp', 'Norm', 'Strike', 'Time', 'Kill');  -- in progress, normal, strikeout, timeout, killed
CREATE TYPE privilege AS ENUM ('Restart', 'Ban');  -- privileges to restart the server/pull from GitLab 

CREATE TABLE users (
    id                   SERIAL        PRIMARY KEY,
    name                 TEXT          NOT NULL  UNIQUE,
    pw                   BYTEA         NOT NULL, -- Hashed and salted with SHA-256
    salt                 BYTEA         NOT NULL, -- 32-byte like the hash
    last_ip              INET          NULL, -- Used to assist in IP banning
    privilege            privilege[]   NULL  DEFAULT array[]::privilege[],
    -- tester               INT           NOT NULL  DEFAULT 0,
    datetime_created     TIMESTAMP     NOT NULL  DEFAULT NOW(),
    datetime_last_login  TIMESTAMP     NOT NULL  DEFAULT NOW()
);
CREATE INDEX users_index_name ON users (name);

CREATE TABLE games (
    id                 SERIAL        PRIMARY KEY,
    name               TEXT          NOT NULL,
    --num_players        SMALLINT      NOT NULL  DEFAULT 2,
    players            INT[]         NOT NULL,
    owner              INT           NOT NULL,
    variant            SMALLINT      NOT NULL,
    timed              BOOLEAN       NOT NULL,
    seed               TEXT          NOT NULL, -- like "p2v0s1"
    score              INT           NOT NULL,
    endcon             endcon        NOT NULL,
    action             jsonb         NOT NULL, /* JSON */
    datetime_created   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_started   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_finished  TIMESTAMP     NOT NULL  DEFAULT NOW(),
    FOREIGN KEY (owner) REFERENCES users (id)
);
CREATE INDEX games_index_players ON games (players);
CREATE INDEX games_index_variant ON games (variant);
CREATE INDEX games_index_seed ON games (seed);

CREATE TABLE timed_games (
    id                 INT           NOT NULL  PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES games (id),
    time_base          INT           NOT NULL, -- in seconds
    time_per_turn      INT           NOT NULL -- in seconds
);

CREATE TABLE banned_ips (
    id                 SERIAL         PRIMARY KEY,
    ip                 INET           NOT NULL,
    user_id            INT            NULL      DEFAULT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    admin_responsible  INT            NOT NULL,
    FOREIGN KEY(admin_responsible) REFERENCES users(id),
    reason             TEXT           NULL      DEFAULT NULL,
    datetime_banned    TIMESTAMP      NOT NULL  DEFAULT NOW()
);
