-- Your SQL goes here
CREATE TABLE rustacean_note
(
    id           INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    rustacean_id INTEGER NOT NULL,
    text         VARCHAR   NOT NULL,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(rustacean_id) REFERENCES rustaceans(id)
);
