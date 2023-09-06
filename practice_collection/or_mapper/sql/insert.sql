DELETE FROM product_category;
-- product_category テーブルにデータを挿入
INSERT INTO product_category (name)
VALUES
    ('Category 1'),
    ('Category 2'),
    ('Category 3'),
    ('Category 4'),
    ('ストリーマー'),
    ('アイドル');

DELETE FROM product;
-- product テーブルにデータを挿入
INSERT INTO product (name, price, category_id)
VALUES
    ('Product 1', 100, 1),
    ('Product 2', 150, 2),
    ('Product 3', 200, 3),
    ('Product 4', 120, 1),
    ('Product 5', 180, 2),
    ('Product 6', 250, 3),
    ('Product 7', 90, 1),
    ('Product 8', 130, 2),
    ('Product 9', 210, 3),
    ('Product 10', 110, 1),
    ('花ノ木もえ', 200, 6),
    ('はしちゃん', 200, 5);