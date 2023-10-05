USE la_charcuterie_en_ligne;

CREATE USER 'admin_username'@'%' IDENTIFIED BY 'admin_password';
GRANT 
CREATE, 
ALTER, 
DROP, 
INSERT, 
UPDATE, 
DELETE, 
SELECT, 
REFERENCES ON `la_charcuterie_en_ligne`.* TO 'admin_username'@'%' WITH GRANT OPTION;
