-- drop table cascade drops dependent objects
-- in this case it just means it drops the foreign key constraint of the dependent tables
DROP VIEW menu_entries_complete
DROP TABLE users CASCADE;
DROP TABLE menuentries CASCADE;
DROP TABLE food_ratings;
DROP TABLE restaurants CASCADE;
DROP TABLE trust_ratings CASCADE;
