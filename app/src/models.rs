use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct MenuEntry {
    pub food_id: i32,
    pub restaurant_id: i32,
    pub creator_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub calories: i32,
    pub creation_date: NaiveDateTime, 
    pub update_date: NaiveDateTime 
}
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub first_seen: NaiveDateTime,
    pub last_seen: NaiveDateTime
}
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Rating {
    pub food_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub timestamp: NaiveDateTime
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Restaurant{
    restaurant_id: i32,
    name: String,
    google_location: String,
    creator_id: i32
}

// END ROW STRUCTS


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullEntry{
    food_id: i32,
    food_title: String,
    description: String,
    price: f64,
    calories: i32,
    // rating is optional because we can have entries for which there are no ratings
    rating: Option<f64>,
    restaurant_id: i32,
    restaurant_name: String,
    restaurant_google_location: String,
    creator_id: i32,
    creator_first_name: Option<String>,
    creator_last_name: Option<String>,
    creation_date: NaiveDateTime,
    update_date: NaiveDateTime
}


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullRestaurant{
    restaurant_id: i32,
    name: String,
    google_location: String,
    creator_id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    restaurant_rating: Option<f64>
}
