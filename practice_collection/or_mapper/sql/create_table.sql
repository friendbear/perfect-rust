CREATE DATABASE rust_sample;
DROP TABLE product_category;
CREATE TABLE product_category (
	id SERIAL PRIMARY KEY,
	name VARCHAR ( 250 ) UNIQUE NOT NULL
);

DROP TABLE product;
CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name VARCHAR (250) NOT NULL,
    price INTEGER NOT NULL,
    category_id INTEGER REFERENCES product_category(id)
);

DROP SEQUENCE product_seq;
CREATE SEQUENCE product_seq
    START 1
    INCREMENT 1
    MINVALUE 1
    MAXVALUE 100000
    CYCLE;

DROP SEQUENCE product_category_seq;
CREATE SEQUENCE product_category_seq
    START 1
    INCREMENT 1
    MINVALUE 1
    MAXVALUE 100000
    CYCLE;
