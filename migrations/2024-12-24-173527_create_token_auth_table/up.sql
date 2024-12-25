-- Your SQL goes here
CREATE TABLE auth_token (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    uid VARCHAR(255) NOT NULL,
    token_type INT NOT NULL,
    iat INT NOT NULL,
    exp INT NOT NULL,
    is_revoke BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (uid) REFERENCES users(id)
);
