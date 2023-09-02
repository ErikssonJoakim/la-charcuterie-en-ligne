USE la_charcuterie_en_ligne;

INSERT INTO product(name, cost, cost_measure)
VALUES('jambon du pays', 12.00, 'kg');
INSERT INTO label(name)
VALUES('seche');
INSERT INTO product_label(label_id, product_id)
VALUES(1, 1);

INSERT INTO sales_item(name, description, image, price, comparison_price, quantity)
VALUES('jambon du pays (200g)', 'Très bon jambon du pays', 'jambon-du-pays.jpeg', 3.20, 16.00, 20);
INSERT INTO sales_item_product(product_id, sales_item_id, quantity)
VALUES(1, 1, 0.2);

INSERT INTO client(name, email)
VALUES('Joakim Eriksson', 'erikssonjoakim92@gmail.com');
INSERT INTO command(sales_item_id, client_id, quantity, delivery_address, delivery_date, created_on)
VALUES(1, 1, 2, 'Balma, Av. des Mimosas, 31 130', '2023-07-03', '2023-07-01T18:23:55');