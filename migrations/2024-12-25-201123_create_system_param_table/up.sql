-- Your SQL goes here

CREATE TABLE system_parameter (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  key_name LONGTEXT NOT NULL,
  key_value LONGTEXT NOT NULL,
  # meta data
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
