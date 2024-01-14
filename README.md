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

-- DROP TYPE todoapp."_categories";

CREATE TYPE todoapp."_categories" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.categories,
	DELIMITER = ',');

-- DROP TYPE todoapp."_roles";

CREATE TYPE todoapp."_roles" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.roles,
	DELIMITER = ',');

-- DROP TYPE todoapp."_todos";

CREATE TYPE todoapp."_todos" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.todos,
	DELIMITER = ',');

-- DROP TYPE todoapp."_tokens";

CREATE TYPE todoapp."_tokens" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.tokens,
	DELIMITER = ',');

-- DROP TYPE todoapp."_user_roles";

CREATE TYPE todoapp."_user_roles" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.user_roles,
	DELIMITER = ',');

-- DROP TYPE todoapp."_users";

CREATE TYPE todoapp."_users" (
	INPUT = array_in,
	OUTPUT = array_out,
	RECEIVE = array_recv,
	SEND = array_send,
	ANALYZE = array_typanalyze,
	ALIGNMENT = 8,
	STORAGE = any,
	CATEGORY = A,
	ELEMENT = todoapp.users,
	DELIMITER = ',');

-- DROP SEQUENCE todoapp.categories_category_id_seq;

CREATE SEQUENCE todoapp.categories_category_id_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 2147483647
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE todoapp.categories_category_id_seq OWNER TO jowtro;
GRANT ALL ON SEQUENCE todoapp.categories_category_id_seq TO jowtro;

-- DROP SEQUENCE todoapp.roles_role_id_seq;

CREATE SEQUENCE todoapp.roles_role_id_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 2147483647
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE todoapp.roles_role_id_seq OWNER TO jowtro;
GRANT ALL ON SEQUENCE todoapp.roles_role_id_seq TO jowtro;

-- DROP SEQUENCE todoapp.todos_todo_id_seq;

CREATE SEQUENCE todoapp.todos_todo_id_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 2147483647
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE todoapp.todos_todo_id_seq OWNER TO jowtro;
GRANT ALL ON SEQUENCE todoapp.todos_todo_id_seq TO jowtro;

-- DROP SEQUENCE todoapp.tokens_token_id_seq;

CREATE SEQUENCE todoapp.tokens_token_id_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 2147483647
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE todoapp.tokens_token_id_seq OWNER TO jowtro;
GRANT ALL ON SEQUENCE todoapp.tokens_token_id_seq TO jowtro;

-- DROP SEQUENCE todoapp.users_user_id_seq;

CREATE SEQUENCE todoapp.users_user_id_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 2147483647
	START 1
	CACHE 1
	NO CYCLE;

-- Permissions

ALTER SEQUENCE todoapp.users_user_id_seq OWNER TO jowtro;
GRANT ALL ON SEQUENCE todoapp.users_user_id_seq TO jowtro;
-- todoapp.categories definition

-- Drop table

-- DROP TABLE todoapp.categories;

CREATE TABLE todoapp.categories (
	category_id serial4 NOT NULL,
	"name" varchar(50) NOT NULL,
	CONSTRAINT pk_categories PRIMARY KEY (category_id)
);

-- Permissions

ALTER TABLE todoapp.categories OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.categories TO jowtro;


-- todoapp.roles definition

-- Drop table

-- DROP TABLE todoapp.roles;

CREATE TABLE todoapp.roles (
	role_id serial4 NOT NULL,
	role_name varchar(50) NOT NULL,
	CONSTRAINT roles_pkey PRIMARY KEY (role_id),
	CONSTRAINT roles_role_name_key UNIQUE (role_name)
);

-- Permissions

ALTER TABLE todoapp.roles OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.roles TO jowtro;


-- todoapp.users definition

-- Drop table

-- DROP TABLE todoapp.users;

CREATE TABLE todoapp.users (
	user_id serial4 NOT NULL,
	username varchar(50) NOT NULL,
	password_hash varchar(256) NOT NULL,
	CONSTRAINT users_pkey PRIMARY KEY (user_id),
	CONSTRAINT users_username_key UNIQUE (username)
);

-- Permissions

ALTER TABLE todoapp.users OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.users TO jowtro;


-- todoapp.todos definition

-- Drop table

-- DROP TABLE todoapp.todos;

CREATE TABLE todoapp.todos (
	todo_id serial4 NOT NULL,
	task varchar(50) NULL,
	completed bool NULL DEFAULT false,
	category_id int4 NULL,
	CONSTRAINT pk_todos PRIMARY KEY (todo_id),
	CONSTRAINT fk_categories FOREIGN KEY (category_id) REFERENCES todoapp.categories(category_id) ON DELETE SET NULL
);

-- Permissions

ALTER TABLE todoapp.todos OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.todos TO jowtro;


-- todoapp.tokens definition

-- Drop table

-- DROP TABLE todoapp.tokens;

CREATE TABLE todoapp.tokens (
	token_id serial4 NOT NULL,
	user_id int4 NOT NULL,
	jwt_token varchar(512) NOT NULL,
	expiration_time int8 NOT NULL,
	CONSTRAINT tokens_pkey PRIMARY KEY (token_id),
	CONSTRAINT tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES todoapp.users(user_id) ON DELETE CASCADE
);
CREATE INDEX idx_tokens_expiration_time ON todoapp.tokens USING btree (expiration_time);

-- Permissions

ALTER TABLE todoapp.tokens OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.tokens TO jowtro;


-- todoapp.user_roles definition

-- Drop table

-- DROP TABLE todoapp.user_roles;

CREATE TABLE todoapp.user_roles (
	user_id int4 NOT NULL,
	role_id int4 NOT NULL,
	CONSTRAINT user_roles_pkey PRIMARY KEY (user_id, role_id),
	CONSTRAINT user_roles_role_id_fkey FOREIGN KEY (role_id) REFERENCES todoapp.roles(role_id) ON DELETE CASCADE,
	CONSTRAINT user_roles_user_id_fkey FOREIGN KEY (user_id) REFERENCES todoapp.users(user_id) ON DELETE CASCADE
);

-- Permissions

ALTER TABLE todoapp.user_roles OWNER TO jowtro;
GRANT ALL ON TABLE todoapp.user_roles TO jowtro;




-- Permissions

GRANT ALL ON SCHEMA todoapp TO postgres;

```
