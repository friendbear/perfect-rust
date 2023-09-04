-- product_category テーブルにデータを挿入
INSERT INTO product_category (id, name)
VALUES
    (nextval('product_category_seq'), 'Category 1'),
    (nextval('product_category_seq'), 'Category 2'),
    (nextval('product_category_seq'), 'Category 3'),
    (nextval('product_category_seq'), 'Category 4'),
    (nextval('product_category_seq'), 'Category 5');

-- product テーブルにデータを挿入
INSERT INTO product (id, name, price, category_id)
VALUES
    (nextval('product_seq'), 'Product 1', 100, 1),
    (nextval('product_seq'), 'Product 2', 150, 2),
    (nextval('product_seq'), 'Product 3', 200, 3),
    (nextval('product_seq'), 'Product 4', 120, 1),
    (nextval('product_seq'), 'Product 5', 180, 2),
    (nextval('product_seq'), 'Product 6', 250, 3),
    (nextval('product_seq'), 'Product 7', 90, 1),
    (nextval('product_seq'), 'Product 8', 130, 2),
    (nextval('product_seq'), 'Product 9', 210, 3),
    (nextval('product_seq'), 'Product 10', 110, 1);