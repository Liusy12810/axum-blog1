DROP TABLE authblog.admins;
CREATE TABLE authblog.admins (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE
);