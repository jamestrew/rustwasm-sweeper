-- Difficulty table
CREATE TABLE difficulty (
    id INTEGER PRIMARY KEY,
    description TEXT
);

-- Inserting difficulties
INSERT INTO difficulty (description) VALUES ('beginner');
INSERT INTO difficulty (description) VALUES ('intermediate');
INSERT INTO difficulty (description) VALUES ('expert');
INSERT INTO difficulty (description) VALUES ('custom');

-- Player table
CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    name TEXT
);

-- Score table
CREATE TABLE score (
    id INTEGER PRIMARY KEY,
    player_id INTEGER,
    difficulty_id INTEGER,
    time INTEGER,
    inserted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES player (id),
    FOREIGN KEY (difficulty_id) REFERENCES difficulty (id)
);
