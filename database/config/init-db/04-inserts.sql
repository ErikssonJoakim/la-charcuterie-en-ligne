USE la_charcuterie_en_ligne;

INSERT INTO measurement(name, abbreviation)
VALUES('kilogram', 'kg');
INSERT INTO product(name, cost, measurement_id)
VALUES('jambon du pays', 12.00, 1);
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
INSERT INTO relay_point(address)
VALUES('Balma, Av. des Mimosas, 31 130');
INSERT INTO command(client_id, relay_point_id, delivery_date, created_on)
VALUES(1, 1, '2023-07-03', '2023-07-01T18:23:55');
