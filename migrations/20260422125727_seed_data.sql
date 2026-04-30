-- Add migration script here
-- Сначала категории
INSERT INTO categories (name) VALUES
    ('Электроника'),
    ('Одежда'),
    ('Продукты питания');

-- Потом продукты (category_id ссылается на категории выше)
INSERT INTO products (name, description, price, stock, image_url, category_id) VALUES
    ('iPhone 15', 'Смартфон Apple iPhone 15 128GB', 450000.00, 10, 'https://example.com/iphone15.jpg', 1),
    ('Samsung Galaxy S24', 'Смартфон Samsung 256GB', 380000.00, 15, 'https://example.com/s24.jpg', 1),
    ('Куртка зимняя', 'Тёплая куртка на синтепоне', 25000.00, 50, 'https://example.com/jacket.jpg', 2),
    ('Футболка базовая', 'Хлопковая футболка', 5000.00, 100, 'https://example.com/tshirt.jpg', 2),
    ('Гречка 1кг', 'Гречневая крупа ядрица', 800.00, 200, 'https://example.com/buckwheat.jpg', 3),
    ('Молоко 1л', 'Молоко пастеризованное 3.2%', 450.00, 150, 'https://example.com/milk.jpg', 3);
