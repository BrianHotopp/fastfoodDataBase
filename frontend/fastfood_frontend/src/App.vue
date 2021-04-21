<template>
  <div id="app" class="lg:px-64 flex flex-col h-screen text-myblack">
    <header>
      <nav class="bg-myorange flex justify-between flex-wrap rounded-lg p-3 sm:mt-3">
        <!--Mobile menu button-->
        <div class="m-3 p-2 bg-white rounded-md text-3xl">
          FastFoodDB
        </div>
        <ul class="p-3 flex justify-end">
          <li
           class="py-1 px-5 my-1 mr-1 rounded-md flex flex-col justify-center hover:bg-mywhite bg-white ">
            <router-link to="/menuentries">Items</router-link>
          </li>
          <li class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-mywhite bg-white">
            <router-link to="/restaurants">Restaurants</router-link>
          </li>
          <li class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-mywhite bg-white">
            <!-- Check that the SDK client is not currently loading before accessing is methods -->
            <div v-if="!$auth.loading">
              <!-- show login when not authenticated -->
              <button v-if="!$auth.isAuthenticated" @click="login">Log in</button>
              <!-- show logout when authenticated -->
              <button v-if="$auth.isAuthenticated" @click="logout">Log out</button>
            </div>

          </li>
          <li v-if="$auth.isAuthenticated" class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-mywhite bg-white">
            <router-link  to="/profile">Profile</router-link>
          </li>
          <li v-if="$auth.isAuthenticated" class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-mywhite bg-white">
            <router-link to="/additem">Add Item</router-link>
          </li>
        </ul>
      </nav>
    </header>
    <main>
    <router-view />
    </main>
    <footer class="flex flex-col flex-grow justify-end sm:mb-3">
      <ul class="flex flex-row bg-myorange text-white rounded-lg p-6 flex-wrap">
        <li class="p-1 m-1 flex flex-col justify-center">
          <a href="#app" class="hover:underline">Jump to Header</a>
        </li>
        <li class="p-1 m-1 flex flex-col justify-center">
          <router-link to="/about" class="hover:underline">
            About
          </router-link>
        </li>
        <li class="p-1 m-1 flex flex-col justify-center hover:underline">
          <router-link to="/me">
            About Me
          </router-link>
        </li>
        <li class="p-1 m-1 flex flex-col justify-center hover:underline">
          <router-link to="/me">
            Source Code
          </router-link>
        </li>
      </ul>
    </footer>
  </div>
</template>


<script>
export default {
  name: "App",
  components: {
  },
  methods:{
    // Log the user in
    login() {
      this.$auth.loginWithRedirect();
    },
    // Log the user out
    logout() {
      this.$auth.logout({
        returnTo: window.location.origin
      });
    }
  }
};
</script>

<style>


</style>
