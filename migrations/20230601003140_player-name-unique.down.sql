-- Add down migration script here
DROP TABLE player;

CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    name TEXT
);
