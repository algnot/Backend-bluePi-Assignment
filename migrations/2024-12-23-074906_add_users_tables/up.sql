-- Your SQL goes here

CREATE TABLE users (
   id VARCHAR(255) NOT NULL PRIMARY KEY,
   username BLOB NOT NULL,
   email BLOB NOT NULL,
   hashed_password BLOB NOT NULL,
   active BOOLEAN DEFAULT TRUE,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
