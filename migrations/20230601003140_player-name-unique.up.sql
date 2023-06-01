-- Add up migration script here
DROP TABLE player;

CREATE TABLE player (
    id INTEGER PRIMARY KEY,
    name VARCHAR UNIQUE
);
