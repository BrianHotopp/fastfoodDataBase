<template>
  <div id="app" class="lg:px-64 flex flex-col min-h-screen text-myblack bg-bgcolor">
    <header>
      <link rel="preconnect" href="https://fonts.gstatic.com">
      <link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@100;300;400;500;600;700;900&display=swap" rel="stylesheet">
        <div class="m-3 p-2 rounded-md text-3xl" >
          <router-link  to="/menuentries">
            <img src='./assets/fastfooddbsmall.png' srcset='/./assets/fastfooddbmedium.png 2x, /./assets/fastfooddbbig.png 4x' alt='logo'>
          </router-link>
        </div>
      <nav class="bg-bgcolor+1 flex justify-between flex-wrap rounded-lg p-3 sm:mt-3 ">
        <!--Mobile menu button-->
        <ul class="p-3 flex justify-end">
          <li class="py-1 px-5 my-1 mr-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <router-link to="/menuentries">Items</router-link>
          </li>
          <li class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <router-link to="/restaurants">Restaurants</router-link>
          </li>
          <li class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <!-- Check that the SDK client is not currently loading before accessing is methods -->
            <div v-if="!$auth.loading">
              <!-- show login when not authenticated -->
              <button v-if="!$auth.isAuthenticated" @click="login" class='font-semibold'>Log in</button>
              <!-- show logout when authenticated -->
              <button v-if="$auth.isAuthenticated" @click="logout" class='font-semibold'>Log out</button>
            </div>

          </li>
          <li v-if="$auth.isAuthenticated" class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <router-link  to="/profile">Profile</router-link>
          </li>
          <li v-if="$auth.isAuthenticated" class="py-1 px-5  m-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <router-link to="/additem">Add Item</router-link>
          </li>
        </ul>
      </nav>
    </header>
    <main>
    <router-view />
    </main>
    <footer class="flex flex-col flex-grow justify-end sm:mb-3">
      <ul class="flex flex-row bg-bgcolor+1 text-white rounded-lg p-6 flex-wrap font-buttonfont divide-x divide-myorange grid grid-cols-4">
        <li class="p-1 m-1 flex flex-row justify-center">
          <a href="#app" class="hover:underline">Jump to Header</a>
        </li>
        <li class="p-1 m-1 flex flex-row justify-center font-buttonfont">
          <router-link to="/about" class="hover:underline">
            About
          </router-link>
        </li>
        <li class="p-1 m-1 flex flex-row justify-center hover:underline font-buttonfont">
          <router-link to="/aboutme">
            About Me
          </router-link>
        </li>
        <li class="p-1 m-1 flex flex-row justify-center hover:underline font-buttonfont">
          <a href="https://github.com/BrianHotopp/fastfoodatabase">
            Source Code
          </a>
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
