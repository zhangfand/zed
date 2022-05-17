CREATE TABLE invite_codes (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER REFERENCES users (id) NOT NULL,
    code VARCHAR(64),
    remaining_count INTEGER
);

CREATE TABLE invite_code_usages (
    id SERIAL PRIMARY KEY,
    invite_code_id INTEGER REFERENCES invite_codes (id) NOT NULL,
    invitee_id INTEGER REFERENCES users (id) NOT NULL,
    usage_time TIMESTAMP
);

CREATE INDEX index_invite_codes_owner_id ON invite_codes (owner_id);
CREATE UNIQUE INDEX index_invite_codes_code ON invite_codes (code);
