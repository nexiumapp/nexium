CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS accounts (
    id uuid DEFAULT uuid_generate_v4(),
    username varchar(50) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS accounts_username ON accounts(username);
