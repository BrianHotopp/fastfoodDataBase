import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import '@/assets/css/tailwind.css';


// Import the Auth0 configuration
import { domain, clientId } from "../auth_config.json";

// Import the plugin here
import { Auth0Plugin } from "./auth";

// Install the authentication plugin here
Vue.use(Auth0Plugin, {
  domain,
  clientId,
  onRedirectCallback: appState => {
    router.push(
      appState && appState.targetUrl
        ? appState.targetUrl
        : window.location.pathname
    );
  }
});


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
