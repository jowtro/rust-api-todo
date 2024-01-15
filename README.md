# Wip - Adding token authentication

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

-- DROP TYPE todoapp.categories;

CREATE TYPE todoapp.categories AS (
	category_id serial4,
	"name" varchar(50));

-- DROP TYPE todoapp.roles;

CREATE TYPE todoapp.roles AS (
	role_id serial4,
	role_name varchar(50));

-- DROP TYPE todoapp.todos;

CREATE TYPE todoapp.todos AS (
	todo_id serial4,
	task varchar(50),
	completed bool,
	category_id int4);

-- DROP TYPE todoapp.tokens;

CREATE TYPE todoapp.tokens AS (
	token_id serial4,
	user_id int4,
	jwt_token varchar(512),
	expiration_time int8);

-- DROP TYPE todoapp.user_roles;

CREATE TYPE todoapp.user_roles AS (
	user_id int4,
	role_id int4);

-- DROP TYPE todoapp.users;

CREATE TYPE todoapp.users AS (
	user_id serial4,
	username varchar(50),
	password_hash varchar(256));


```
