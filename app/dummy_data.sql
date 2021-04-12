insert into users(first_name, last_name) values('Brian', 'Hotopp');
insert into restaurants(name, google_location, creator_id) values('McDonalds', 'mcdlocation', 1);
insert into menuentries(restaurant_id, creator_id, name, description, price, calories, creation_date, update_date) values(1, 1, 'test menuitem 2', 'tasty menu item', 1.89, 180, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

insert into menuentries(restaurant_id, creator_id, name, description, price, calories, creation_date, update_date) values(1, 1, 'le hamburger', 'tasty hamburger', 1.90, 260, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

insert into menuentries(restaurant_id, creator_id, name, description, price, calories, creation_date, update_date) values(1, 1, 'shredded chicken quesadilla', 'rest in peace sweet prince', 9.00, 300, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

insert into food_ratings(food_id, user_id, rating, comment, timestamp) values(1, 1, 45, 'pretty good food', CURRENT_TIMESTAMP);
