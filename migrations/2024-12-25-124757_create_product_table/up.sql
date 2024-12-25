-- Your SQL goes here

CREATE TABLE product (
      id VARCHAR(255) NOT NULL PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      price DECIMAL DEFAULT 0,
      quantity DECIMAL DEFAULT 0,
      type_id VARCHAR(255),
      recommend BOOLEAN DEFAULT FALSE,
      active BOOLEAN DEFAULT TRUE,

      # meta data
      created_by VARCHAR(255) NOT NULL,
      updated_by VARCHAR(255) NOT NULL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
      FOREIGN KEY (created_by) REFERENCES users(id),
      FOREIGN KEY (updated_by) REFERENCES users(id),
      FOREIGN KEY (type_id) REFERENCES product_type(id)
);
