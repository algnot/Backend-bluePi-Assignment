-- Your SQL goes here

CREATE TABLE sale_order (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    sale_order_name LONGTEXT NOT NULL,
    status INTEGER DEFAULT(0),
    total DOUBLE DEFAULT(0),

    # meta data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE sale_order_line (
   id INTEGER AUTO_INCREMENT PRIMARY KEY,
   product_id VARCHAR(255) NOT NULL,
   sale_order_id INTEGER NOT NULL,
   quantity INTEGER DEFAULT(1),
   total DOUBLE DEFAULT(0),

   # meta data
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
   FOREIGN KEY (sale_order_id) REFERENCES sale_order(id),
   FOREIGN KEY (product_id) REFERENCES product(id)
);