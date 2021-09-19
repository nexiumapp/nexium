CREATE TABLE IF NOT EXISTS auth_password (
    account uuid NOT NULL UNIQUE,
    hash text NOT NULL,
    FOREIGN KEY (account) REFERENCES account(id)
);
