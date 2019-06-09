-- Your SQL goes here
CREATE TABLE consoles
(
    _id INTEGER NOT NULL PRIMARY KEY,
    short_name VARCHAR NOT NULL,
    long_name VARCHAR NOT NULL
);

CREATE TABLE games
(
    _id INTEGER NOT NULL PRIMARY KEY,
    title_en VARCHAR NOT NULL,
    title_ja VARCHAR NOT NULL,
    serial_code VARCHAR,
    console_id INTEGER,
    link_url VARCHAR,
    notes VARCHAR,
    FOREIGN KEY (console_id) REFERENCES consoles(id) ON DELETE SET NULL
);

CREATE TABLE genres
(
    _id INTEGER NOT NULL PRIMARY KEY,
    genre VARCHAR NOT NULL
);

CREATE TABLE genre_map
(
    _id INTEGER NOT NULL PRIMARY KEY,
    game_key INTEGER NOT NULL,
    genre_key INTEGER NOT NULL,
    FOREIGN KEY (game_key) REFERENCES games(_id) ON DELETE CASCADE,
    FOREIGN KEY (genre_key) REFERENCES genres(genre) ON DELETE CASCADE
);

CREATE TABLE game_companies
(
    _id INTEGER NOT NULL PRIMARY KEY,
    company_name_en VARCHAR NOT NULL,
    company_name_ja VARCHAR
);

CREATE TABLE developer_map
(
    _id INTEGER NOT NULL PRIMARY KEY,
    game_id INTEGER NOT NULL,
    company_id INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(_id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES game_companies(_id) ON DELETE CASCADE
);

CREATE TABLE publisher_map
(
    _id INTEGER NOT NULL PRIMARY KEY,
    game_id INTEGER NOT NULL,
    company_id INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (_id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES game_companie(_id) ON DELETE CASCADE
);