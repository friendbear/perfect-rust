CREATE DATABASE rust_sample;
CREATE TABLE product_category (
	id INTEGER PRIMARY KEY,
	name VARCHAR ( 250 ) UNIQUE NOT NULL
);

CREATE TABLE product (
    id INTEGER PRIMARY KEY,
    name VARCHAR (250) NOT NULL,
    price INTEGER NOT NULL,
    category_id INTEGER REFERENCES product_category(id)
);

CREATE SEQUENCE product_seq
    START 1
    INCREMENT 1
    MINVALUE 1
    MAXVALUE 100000
    CYCLE;

CREATE SEQUENCE product_category_seq
    START 1
    INCREMENT 1
    MINVALUE 1
    MAXVALUE 100000
    CYCLE;
