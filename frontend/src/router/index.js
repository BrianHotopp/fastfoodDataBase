import Vue from "vue";
import VueRouter from "vue-router";
import Entries from "../views/Entries.vue";
import Entry from "../views/Entry.vue";
import Restaurants from "../views/Restaurants.vue";
import Restaurant from "../views/Restaurant.vue";
import About from "../views/About.vue";
import NotFoundComponent from "../components/NotFoundComponent.vue";
Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    redirect: "/menuentries"
  },
  {
    path: "/menuentries/:entry_id",
    name: "Entry",
    component: Entry
  },
  {
    path: "/menuentries",
    name: "Entries",
    component: Entries
  },
  {
    path: "/restaurants",
    name: "Restaurants",
    component: Restaurants
  },
  {
    path: "/restaurants/:restaurant_id",
    name: "Restaurant",
    component: Restaurant
  },
  {
    path: "/about",
    name: "About",
    component: About
  },
  { path: '*', component: NotFoundComponent }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes
});

export default router;
