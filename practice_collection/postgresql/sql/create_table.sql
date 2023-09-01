CREATE DATABASE rust_sample;
CREATE TABLE product_category (
	id INTEGER PRIMARY KEY,
	name VARCHAR ( 250 ) UNIQUE NOT NULL
);

CREATE TABLE product (
	id INTEGER PRIMARY KEY,
	name VARCHAR ( 250 ) NOT NULL,
    price INTEGER NOT NULL,
    category_id INTEGER
);

CREATE SEQUENCE product_seq
    START 10
    INCREMENT 1
    MINVALUE 10
    MAXVALUE 100000
    CYCLE;

CREATE SEQUENCE product_category_seq
    START 10
    INCREMENT 1
    MINVALUE 10
    MAXVALUE 100000
    CYCLE;