USE la_charcuterie_en_ligne;

CREATE TABLE IF NOT EXISTS label (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(80) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS measurement (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(80) NOT NULL,
    abbreviation VARCHAR(20) NOT NULL,
    PRIMARY KEY (id)
 );

CREATE TABLE IF NOT EXISTS product (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    measurement_id INT UNSIGNED NOT NULL,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(255),
    cost DECIMAL(8,2) UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK__measurement_id__product FOREIGN KEY (measurement_id) REFERENCES measurement(id)
 );

CREATE TABLE IF NOT EXISTS product_label (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    label_id INT UNSIGNED NOT NULL,
    product_id INT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK__label_id__product_label FOREIGN KEY (label_id) REFERENCES label(id) ON DELETE CASCADE,
    CONSTRAINT FK__product_id__product_label FOREIGN KEY (product_id) REFERENCES product(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sales_item (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(80) NOT NULL,
    image VARCHAR(255) NOT NULL,
    price DECIMAL(8,2) UNSIGNED NOT NULL,
    quantity INT UNSIGNED NOT NULL,
    description VARCHAR(255),
    comparison_price DECIMAL(8,2) UNSIGNED,
    PRIMARY KEY (id)
 );

CREATE TABLE IF NOT EXISTS sales_item_product (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    product_id INT UNSIGNED NOT NULL,
    sales_item_id INT UNSIGNED NOT NULL,
    quantity INT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK__product_id__sales_item_product FOREIGN KEY (product_id) REFERENCES product(id) ON DELETE CASCADE,
    CONSTRAINT FK__sales_item_id__sales_item_product FOREIGN KEY (sales_item_id) REFERENCES sales_item(id) ON DELETE CASCADE
 );

 CREATE TABLE IF NOT EXISTS relay_point (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    address VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
 );

 CREATE TABLE IF NOT EXISTS command (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    client_id INT UNSIGNED NOT NULL,
    relay_point_id INT UNSIGNED NOT NULL,
    delivery_date DATE NOT NULL,
    created_on DATETIME NOT NULL DEFAULT NOW(),
    updated_on DATETIME ON UPDATE NOW(),
    PRIMARY KEY (id),
    CONSTRAINT FK__client_id__command FOREIGN KEY (client_id) REFERENCES client(id),
    CONSTRAINT FK__relay_point_id__command FOREIGN KEY (relay_point_id) REFERENCES relay_point(id),
 );

 CREATE TABLE IF NOT EXISTS command_sales_item (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    sales_item_id INT UNSIGNED NOT NULL,
    command_id INT UNSIGNED NOT NULL,
    quantity INT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK__sales_item_id__command_sales_item FOREIGN KEY (sales_item_id) REFERENCES sales_item(id),
    CONSTRAINT FK__command_id__command_sales_item FOREIGN KEY (command_id) REFERENCES command(id)
 );

 CREATE TABLE IF NOT EXISTS client (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(80) NOT NULL,
    email VARCHAR(80) NOT NULL,
    PRIMARY KEY (id)
 );
