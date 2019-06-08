-- Your SQL goes here
CREATE TABLE retro_games (
    _index INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    title_translation VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0
)