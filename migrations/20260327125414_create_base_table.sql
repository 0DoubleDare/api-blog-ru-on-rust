-- Add migration script here
CREATE TABLE users(
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE posts(
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    author_id INT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id)
);
