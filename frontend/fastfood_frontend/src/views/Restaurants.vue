<template>
  <div class="entries flex flex-col my-6 justify-start font-buttonfont font-semibold">
    <section class="bg-myorange+1 p-1 m-1 shadow-md rounded-md">
      <form class="flex flex-row p-3">
        <label for="sort" class='px-2'>Sort By:</label>
        <select v-model="query_params.sortby" id="sort" name="sort" class='rounded bg-myorange+3'>
          <option v-for="item in sort_options" :key="item">{{item}}</option>
        </select>

        <label for="show" class='px-2'>Show:</label>
        <select v-model="query_params.results" id="show" name="show" class='rounded bg-myorange+3'>
          <option v-for="item in show_options" :key="item">{{item}}</option>
        </select>
        <label for="revorder" class='px-2'>Reverse Order</label>
        <input v-model="query_params.revorder" type="checkbox" id="revorder">
        <p class='px-5'></p>
        <button v-on:click.prevent="hit_db()" class="bg-myorange+3 rounded px-2 text-black border border-black hover:bg-myorange+4 font-semibold">Apply</button>

      </form> 

          </section>
    <section> 
      <ul class="flex flex-col">
        <li v-for="(restaurant,index) in restaurant_list" v-bind:key="index"><restaurant-list-item v-bind:item="restaurant"></restaurant-list-item> </li>
      </ul>
    </section>
  </div>
</template>
<script>
import axios from "axios";
import RestaurantListItem from "../components/RestaurantListItem.vue";
export default {
  name: "Restaurants",
  components: {
    RestaurantListItem
  },
  data(){
    return{
      sort_options: ["rating", "name"],
      show_options: [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 100],
      restaurant_list: null,

      query_params: {
        sortby: "rating",
        revorder: false,
        results: 20
      },

    }
  },
  methods:{
    hit_db(){
    axios
    .get("http://localhost:8000/restaurants",
    {
      params: this.query_params
    }
    )
    .then(response => (this.restaurant_list = response.data))
    .catch(error => {console.log(error);});
    }
  },

  mounted () {
      this.hit_db();
  }
}

</script>
