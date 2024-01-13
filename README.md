# rust-api-todo
Testing rocket crate and SQLx

## Note
DBMS: POSTGRESQL
DB:testbox->Schema->todoapp->table->todos

## .env
```bash
DATABASE_URL=postgres://youruser:passwd@localhost:5432/testbox
```


```SQL
CREATE DATABASE testbox;

-- DROP SCHEMA todoapp;

CREATE SCHEMA todoapp AUTHORIZATION postgres;


-- Permissions

GRANT ALL ON SCHEMA todoapp TO postgres;

-- todoapp.todos definition
-- Drop table
-- DROP TABLE todoapp.todos;

CREATE TABLE todoapp.todos (
	todo_id serial4 NOT NULL,
	task varchar(50) NULL,
	completed bool NULL DEFAULT false,
	category_id int4 NULL,
	CONSTRAINT pk_todos PRIMARY KEY (todo_id)
);

-- DROP TABLE IF EXISTS todoapp.categories;

CREATE TABLE todoapp.categories (
    category_id serial4 NOT NULL,
    name varchar(50) NOT NULL,
    CONSTRAINT pk_categories PRIMARY KEY (category_id)
);


-- todoapp.todos foreign keys

ALTER TABLE todoapp.todos ADD CONSTRAINT fk_categories FOREIGN KEY (category_id) REFERENCES todoapp.categories(category_id) ON DELETE SET NULL;

-- Table for storing user information
CREATE TABLE IF NOT EXISTS todoapp.users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(256) NOT NULL
);

-- Table for storing roles
CREATE TABLE IF NOT EXISTS todoapp.roles (
    role_id SERIAL PRIMARY KEY,
    role_name VARCHAR(50) UNIQUE NOT NULL
);

-- Table for mapping users to roles
CREATE TABLE IF NOT EXISTS todoapp.user_roles (
    user_id INTEGER REFERENCES todoapp.users(user_id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES todoapp.roles(role_id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- Table for storing JWT tokens
CREATE TABLE IF NOT EXISTS todoapp.tokens (
    token_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES todoapp.users(user_id) ON DELETE CASCADE,
    jwt_token VARCHAR(512) NOT NULL,
    expiration_time TIMESTAMPTZ NOT NULL
);

-- Corrected Index Statement
CREATE INDEX IF NOT EXISTS idx_tokens_expiration_time ON todoapp.tokens(expiration_time);

```
