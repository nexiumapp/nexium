CREATE TABLE IF NOT EXISTS session (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    secret char(256) NOT NULL,
    account uuid NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_ip cidr NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (account) REFERENCES account(id)
);
