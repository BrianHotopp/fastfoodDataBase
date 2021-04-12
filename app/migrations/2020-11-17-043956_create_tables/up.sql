

CREATE TABLE public.food_ratings (
    food_id integer NOT NULL,
    user_id integer NOT NULL,
    rating integer NOT NULL,
    comment character varying(280),
    "timestamp" timestamp with time zone NOT NULL,
    CONSTRAINT food_ratings_rating_check CHECK (((rating >= 0) AND (rating <= 100)))
);



CREATE TABLE public.menuentries (
    food_id integer NOT NULL,
    restaurant_id integer NOT NULL,
    creator_id integer NOT NULL,
    name character varying(280) NOT NULL,
    description character varying(280) NOT NULL,
    price float NOT NULL,
    calories integer NOT NULL,
    creation_date timestamp with time zone NOT NULL,
    update_date timestamp with time zone NOT NULL,
    CONSTRAINT menuentries_calories_check CHECK ((calories >= 0)),
    CONSTRAINT menuentries_price_check CHECK ((price >= (0)::numeric))
);


CREATE SEQUENCE public.menuentries_food_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.menuentries_food_id_seq OWNED BY public.menuentries.food_id;


CREATE TABLE public.restaurants (
    restaurant_id integer NOT NULL,
    name character varying(280) NOT NULL,
    google_location character varying(100) NOT NULL,
    creator_id integer NOT NULL
);


CREATE SEQUENCE public.restaurants_restaurant_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;



ALTER SEQUENCE public.restaurants_restaurant_id_seq OWNED BY public.restaurants.restaurant_id;

-- this table is so we can allow users to verify our data
-- a trust rating of -1 indicates a bad (wrong information) menu entry
-- a trust rating of 1 indicates a good (correct) menu entry
CREATE TABLE public.trust_ratings (
    food_id integer NOT NULL,
    user_id integer NOT NULL,
    trust integer NOT NULL,
    CONSTRAINT trust_ratings_trust_check CHECK (((trust = '-1'::integer) OR (trust = 1) OR (trust = 0)))
);

CREATE TABLE public.users (
    user_id integer NOT NULL,
    first_name character varying(100),
    last_name character varying(100),
    first_seen timestamp with time zone NOT NULL,
    last_seen timestamp with time zone NOT NULL,
    CONSTRAINT users_first_name_check CHECK (((first_name)::text <> ''::text)),
    CONSTRAINT users_last_name_check CHECK (((last_name)::text <> ''::text))
);


CREATE SEQUENCE public.users_user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;





ALTER SEQUENCE public.users_user_id_seq OWNED BY public.users.user_id;


ALTER TABLE ONLY public.menuentries ALTER COLUMN food_id SET DEFAULT nextval('public.menuentries_food_id_seq'::regclass);



ALTER TABLE ONLY public.restaurants ALTER COLUMN restaurant_id SET DEFAULT nextval('public.restaurants_restaurant_id_seq'::regclass);



ALTER TABLE ONLY public.users ALTER COLUMN user_id SET DEFAULT nextval('public.users_user_id_seq'::regclass);

ALTER TABLE ONLY public.users ALTER COLUMN first_seen SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE ONLY public.users ALTER COLUMN last_seen SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE ONLY public.food_ratings
    ADD CONSTRAINT food_ratings_pkey PRIMARY KEY (food_id, user_id);

ALTER TABLE ONLY public.menuentries
    ADD CONSTRAINT menuentries_pkey PRIMARY KEY (food_id);

ALTER TABLE ONLY public.restaurants
    ADD CONSTRAINT restaurants_pkey PRIMARY KEY (restaurant_id);


ALTER TABLE ONLY public.trust_ratings
    ADD CONSTRAINT trust_ratings_pkey PRIMARY KEY (food_id, user_id);

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (user_id);


-- if we delete a menu entry we want all the ratings for that menu entry to be deleted
ALTER TABLE ONLY public.food_ratings
    ADD CONSTRAINT food_ratings_food_id_fkey FOREIGN KEY (food_id) REFERENCES public.menuentries(food_id) ON DELETE CASCADE;

ALTER TABLE ONLY public.food_ratings
    ADD CONSTRAINT food_ratings_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);

ALTER TABLE ONLY public.menuentries
    ADD CONSTRAINT menuentries_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.users(user_id);

-- if we delete a restaurant we should delete all the menu entries for that restaurant
ALTER TABLE ONLY public.menuentries
    ADD CONSTRAINT menuentries_restaurant_id_fkey FOREIGN KEY (restaurant_id) REFERENCES public.restaurants(restaurant_id) ON DELETE CASCADE;

ALTER TABLE ONLY public.restaurants
    ADD CONSTRAINT restaurants_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.users(user_id);

-- if a menu entry is deleted we should delete all trust ratings for that menu entry
ALTER TABLE ONLY public.trust_ratings
    ADD CONSTRAINT trust_ratings_food_id_fkey FOREIGN KEY (food_id) REFERENCES public.menuentries(food_id) ON DELETE CASCADE;

ALTER TABLE ONLY public.trust_ratings
    ADD CONSTRAINT trust_ratings_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);


CREATE VIEW menu_entries_complete AS
SELECT 
me.food_id, 
me.name AS food_title,
me.description, 
me.price, 
me.calories, 
CAST(r.avg AS DOUBLE PRECISION) AS rating,
me.restaurant_id, 
re.name AS restaurant_name, 
re.google_location AS restaurant_google_location, 
me.creator_id, 
us.first_name AS creator_first_name, 
us.last_name AS creator_last_name,
me.creation_date, 
me.update_date 


CREATE VIEW foodratings AS
SELECT re2.restaurant_id, me.food_id, CAST(r.avg AS DOUBLE PRECISION) AS rating 
FROM 
menuentries AS me 
JOIN restaurants AS re2 ON me.restaurant_id = re2.restaurant_id 
NATURAL LEFT OUTER JOIN (SELECT food_id, avg(rating) FROM food_ratings GROUP BY food_id) AS r;

CREATE VIEW rratings AS 
SELECT restaurant_id, avg(rating) AS restaurant_rating
FROM foodratings
GROUP BY restaurant_id;


CREATE VIEW restaurants_complete AS
SELECT 
re.restaurant_id,
re.name,
re.google_location,
re.creator_id,
us.first_name,
us.last_name,
rrt.restaurant_rating
FROM 
restaurants AS re NATURAL JOIN users AS us NATURAL LEFT OUTER JOIN rratings AS rrt;
