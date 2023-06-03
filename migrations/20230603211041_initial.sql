-- Add migration script here
CREATE TABLE difficulty (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE score (
    id INTEGER PRIMARY KEY,
    player_id INTEGER NOT NULL,
    difficulty_id INTEGER NOT NULL,
    time INTEGER NOT NULL,
    inserted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES player (id),
    FOREIGN KEY (difficulty_id) REFERENCES difficulty (id)
);

INSERT INTO difficulty (description) VALUES ('beginner');
INSERT INTO difficulty (description) VALUES ('intermediate');
INSERT INTO difficulty (description) VALUES ('expert');
INSERT INTO difficulty (description) VALUES ('custom');
