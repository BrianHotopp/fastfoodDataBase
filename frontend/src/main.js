import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import '@/assets/css/tailwind.css';
Vue.config.productionTip = false;

Vue.filter('readable_timestamp', function (datestring) {
  let date = new Date(datestring);
  return date.toLocaleDateString();
})
new Vue({
  router,
  store,
  
  render: h => h(App)
}).$mount("#app");
