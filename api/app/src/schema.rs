table! {
    food_ratings (food_id, user_id) {
        food_id -> Int4,
        user_id -> Int4,
        rating -> Int4,
        comment -> Nullable<Varchar>,
        timestamp -> Timestamptz,
    }
}

table! {
    menuentries (food_id) {
        food_id -> Int4,
        restaurant_id -> Int4,
        creator_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        price -> Float8,
        calories -> Int4,
        creation_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}

table! {
    restaurants (restaurant_id) {
        restaurant_id -> Int4,
        name -> Varchar,
        google_location -> Varchar,
        creator_id -> Int4,
    }
}

table! {
    trust_ratings (food_id, user_id) {
        food_id -> Int4,
        user_id -> Int4,
        trust -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        first_seen -> Timestamptz,
        last_seen -> Timestamptz,
    }
}

table! {
    menu_entries_complete (food_id) {
        food_id -> Int4,
        food_title -> Varchar,
        description -> Varchar,
        price -> Float8,
        calories -> Int4,
        rating -> Nullable<Float8>,
        restaurant_id -> Int4,
        restaurant_name -> Varchar,
        restaurant_google_location -> Varchar,
        creator_id -> Int4,
        creator_first_name -> Nullable<Varchar>,
        creator_last_name -> Nullable<Varchar>,
        creation_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}
table! {
    restaurants_complete (restaurant_id) {
        restaurant_id -> Int4,
        name -> Varchar,
        google_location -> Varchar,
        creator_id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        restaurant_rating -> Nullable<Float8>,
    }
}

joinable!(food_ratings -> menuentries (food_id));
joinable!(food_ratings -> users (user_id));
joinable!(menuentries -> restaurants (restaurant_id));
joinable!(menuentries -> users (creator_id));
joinable!(restaurants -> users (creator_id));
joinable!(trust_ratings -> menuentries (food_id));
joinable!(trust_ratings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    food_ratings,
    menuentries,
    restaurants,
    trust_ratings,
    users,
);
