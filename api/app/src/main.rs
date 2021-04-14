#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use crate::models::*;
pub mod cors;
pub mod models;
pub mod schema; 

use rocket_contrib::json::Json;



#[database("fast_food")]
struct FastFoodConn(diesel::PgConnection);
#[get("/menuentries?<sortby>&<revorder>&<results>&<input_restaurant_id>")]
fn get_menuentries(conn: FastFoodConn, sortby: String, results: i64, revorder: bool, input_restaurant_id: Option<i32>) -> Result<Json<Vec<FullEntry>>, String> {
    // returns a json object of all menu entries
    use schema::menu_entries_complete::dsl::*;
    // join entry creator with entry
    let query_static_portion = menu_entries_complete.limit(results).into_boxed();
    let query_order = match sortby.as_str() {
        "price" => match revorder {
            false => query_static_portion.order(price),
            _ => query_static_portion.order(price.desc())
        }
        "calories" => match revorder {
            false => query_static_portion.order(calories),
            _ => query_static_portion.order(calories.desc()),

        }
        "rating" => match revorder {
            false => query_static_portion.order(rating),
            _ => query_static_portion.order(rating.desc())
        }
        _ => query_static_portion.order(price)
    };
    let query_filter = match input_restaurant_id {
        Some(id) => query_order.filter(restaurant_id.eq(id)),
        None => query_order
    };

    query_filter
    .load(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying menu entries: {:?}", err);
        "Error querying menu entries from the database".into()
    }).map(Json)
}
#[get("/menuentries/<input_entry_id>")]
// returns a json of a single entry whose id is input_entry_id
fn get_menu_entry(conn: FastFoodConn, input_entry_id:i32) -> Result<Json<Vec<MenuEntry>>, String> {
    use schema::menuentries::dsl::*;
    menuentries.filter(food_id.eq(input_entry_id)).load(&conn.0).map_err(|err| -> String {
        println!("Error retrieving single menu entry: {:?}", err);
        "Error retrieving single menu entry from the database".into()
    }).map(Json)
    // TODO handle the case when load returns more than one row, which should never happen
}
#[get("/restaurants?<sortby>&<revorder>&<results>")]
fn get_restaurants(conn: FastFoodConn, sortby: String, revorder: bool, results: i64) -> Result<Json<Vec<FullRestaurant>>, String>{
   use schema::restaurants_complete::dsl::*; 
   let query_static_portion = restaurants_complete.limit(results).into_boxed();
   let query_order = match sortby.as_str() {
        "rating" => match revorder {
            false => query_static_portion.order(restaurant_rating),
            _ => query_static_portion.order(restaurant_rating.desc())
        }
        "name" => match revorder {
            false => query_static_portion.order(name),
            _ => query_static_portion.order(name.desc())
        }
        _ => query_static_portion.order(restaurant_rating)
   };

    query_order
    .load(&conn.0)
    .map_err(|err| -> String {
    println!("Error querying restaurants: {:?}", err);
    "Error querying restaurants from the database".into()
    }).map(Json)

}
#[get("/users/<input_user_id>")]
fn get_user(conn:FastFoodConn, input_user_id:i32) -> Result<Json<Vec<User>>, String> {
    // returns a json object with all the information for user with id input_user_id
    use schema::users::dsl::*;
    users.filter(user_id.eq(input_user_id)).load(&conn.0).map_err(|err| -> String {
        println!("Error retrieving user data: {:?}", err);
        "Error retrieving user data from the database".into()
    }).map(Json)


}
#[get("/food_ratings/<input_entry_id>")]
fn get_ratings(conn: FastFoodConn, input_entry_id:i32) -> Result<Json<Vec<(User, Rating)>>, String> {
    // returns a json object containing all the reviews for the food item with id input_entry_id
    use schema::food_ratings::dsl::*;
    use schema::users::dsl::*;
    users
    .inner_join(food_ratings)
    .filter(food_id.eq(input_entry_id))
    .load(&conn.0)
    .map_err(|err|->String{
        println!("Error retrieving user data: {:?}", err);
        "Error retrieving user data from the database".into()
    }).map(Json)
}



fn main() {
        rocket::ignite()
            .attach(FastFoodConn::fairing())
            .attach(cors::CorsFairing)
            .mount("/", routes![get_menuentries])
            .mount("/", routes![get_menu_entry])
            .mount("/", routes![get_restaurants])
            .mount("/", routes![get_user])
            .mount("/", routes![get_ratings])
            .launch();
}
