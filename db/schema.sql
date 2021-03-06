CREATE TYPE endcon AS ENUM ('inp', 'norm', 'so', 'to', 'kill');  -- in progress, normal, strikeout, timeout, killed

DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id                   SERIAL        PRIMARY KEY,
    username             TEXT          NOT NULL  UNIQUE,
    password             TEXT          NOT NULL, -- Hashed with SHA-256
    last_ip              INET          NULL,
    admin                SMALLINT      NOT NULL  DEFAULT 0,
    -- tester               INT           NOT NULL  DEFAULT 0,
    datetime_created     TIMESTAMP     NOT NULL  DEFAULT NOW(),
    datetime_last_login  TIMESTAMP     NOT NULL  DEFAULT NOW()
);
CREATE INDEX users_index_username ON users (username);

DROP TABLE IF EXISTS games CASCADE;
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
    end_condition      ENDCON        NOT NULL,
    action             jsonb         NOT NULL, /* JSON */
    datetime_created   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_started   TIMESTAMP     NOT NULL  DEFAULT '1970-01-01 00:00:00',
    datetime_finished  TIMESTAMP     NOT NULL  DEFAULT NOW(),
    FOREIGN KEY (owner) REFERENCES users (id)
);
CREATE INDEX games_index_players ON games (players);
CREATE INDEX games_index_variant ON games (variant);
CREATE INDEX games_index_seed ON games (seed);

DROP TABLE IF EXISTS timed_games CASCADE;
CREATE TABLE timed_games (
    id                 INT           NOT NULL  PRIMARY KEY,
    FOREIGN KEY (id) REFERENCES games (id),
    time_base          INT           NOT NULL, -- in seconds
    time_per_turn      INT           NOT NULL -- in seconds
);

/*
DROP TABLE IF EXISTS game_participants CASCADE;
CREATE TABLE game_participants (
    id       INT              NOT NULL  PRIMARY KEY,
    user_id  INT              NOT NULL,
    game_id  INT              NOT NULL,
    notes    TEXT             NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (game_id) REFERENCES games (id) ON DELETE CASCADE
    -- If the game is deleted, automatically delete all of the game participant rows
);
CREATE INDEX game_participants_index_user_id ON game_participants (user_id);
CREATE INDEX game_participants_index_game_id ON game_participants (game_id);

DROP TABLE IF EXISTS chat_log CASCADE;
CREATE TABLE chat_log (
    id             SERIAL         PRIMARY KEY,
    user_id        INT            NOT NULL, -- 0 is a Discord message 
    discord_name   TEXT           NULL, -- only used if it is a Discord message 
    room           TEXT           NOT NULL, -- either "lobby" or "game-####" 
    message        TEXT           NOT NULL,
    datetime_sent  TIMESTAMP      NOT NULL  DEFAULT NOW()
);
CREATE INDEX chat_log_index_user_id ON chat_log (user_id);
CREATE INDEX chat_log_index_datetime_sent ON chat_log (datetime_sent);
*/

DROP TABLE IF EXISTS banned_ips CASCADE;
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

/*
DROP TABLE IF EXISTS discord_metadata CASCADE;
CREATE TABLE discord_metadata (
    id     SERIAL         PRIMARY KEY,
    name   TEXT           NOT NULL  UNIQUE,
    value  TEXT          NOT NULL
);
CREATE INDEX discord_metadata_index_name ON discord_metadata (name);
INSERT INTO discord_metadata (name, value) VALUES ('last_at_here', '2006-01-02T15:04:05Z07:00');
/* The "last_at_here" value is stored as a RFC3339 string */

DROP TABLE IF EXISTS discord_waiters CASCADE;
CREATE TABLE discord_waiters (
    id                SERIAL        PRIMARY KEY,
    username          TEXT          NOT NULL,
    discord_mention   TEXT          NOT NULL,
    datetime_expired  TIMESTAMP     NOT NULL
);
*/
