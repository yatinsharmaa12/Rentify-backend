-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_handle VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL
);
