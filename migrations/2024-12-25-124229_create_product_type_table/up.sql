-- Your SQL goes here
CREATE TABLE product_type (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    active BOOLEAN DEFAULT TRUE,

    # meta data
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);
