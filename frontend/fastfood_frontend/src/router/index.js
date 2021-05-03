import Vue from "vue";
import VueRouter from "vue-router";
import Entries from "../views/Entries.vue";
import Entry from "../views/Entry.vue";
import Restaurants from "../views/Restaurants.vue";
import Restaurant from "../views/Restaurant.vue";
import About from "../views/About.vue";
import AboutMe from "../views/AboutMe.vue";
import NotFoundComponent from "../components/NotFoundComponent.vue";
import Profile from "../views/Profile.vue";
import AddItem from "../views/AddItem.vue";
import EditProfile from "../views/EditProfile.vue";
import { authGuard } from "../auth/authGuard";
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
  {
    path: "/aboutme",
    name: "AboutMe",
    component: AboutMe
  },
  {
  path: "/profile",
  name: "profile",
  component: Profile,
  beforeEnter: authGuard
  },
  {
  path: "/additem",
  name: "additem",
  component: AddItem,
  beforeEnter: authGuard
  },
  {
    path: "/editprofile",
    name: "editprofile",
    component: EditProfile,
    beforeEnter: authGuard
    },
  { path: '*', component: NotFoundComponent }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes
});

export default router;
