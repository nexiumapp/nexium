-- Enable the UUID extension.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the account table and add an index for the username.
CREATE TABLE IF NOT EXISTS account (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    username varchar(50) NOT NULL UNIQUE,
    full_name text NOT NULL,
    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS account_username ON account(username);

-- Create an table for authentication with an password.
CREATE TABLE IF NOT EXISTS auth_password (
    account uuid NOT NULL UNIQUE,
    hash text NOT NULL,
    FOREIGN KEY (account) REFERENCES account(id)
);
