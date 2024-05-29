-- Add migration script here

CREATE TABLE translations (
    id INT AUTO_INCREMENT PRIMARY KEY,
    application VARCHAR(255) NOT NULL,
    page VARCHAR(255) NOT NULL,
    key VARCHAR(255) NOT NULL,
    lang VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    context TEXT NOT NULL
);
