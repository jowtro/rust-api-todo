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
```
