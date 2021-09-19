CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS account (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    username varchar(50) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS account_username ON account(username);
