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
        <li v-for="(entry,index) in entry_list" v-bind:key="index"><entry-list-item v-bind:item="entry"></entry-list-item> </li>
      </ul>
    </section>
    

  </div>
</template>

<script>
import axios from 'axios';
import EntryListItem from '../components/EntryListItem.vue';
export default {
  name: "Entries",
  components: {
    EntryListItem
  },
  data(){
    return{
      sort_options: ["price", "calories", "rating"],
      show_options: [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 100],
      entry_list: null,

      query_params: {
        sortby: "price",
        revorder: false,
        results: 20
      },

    }
  },
  methods:{
    hit_db(){
      console.log("here");
    axios
    .get("http://localhost:8000/menuentries",
    {
      params: this.query_params
    }
    )
    .then(response => (this.entry_list = response.data))
    .catch(error => {console.log(error);});
    }
  },

  mounted () {
    this.hit_db();
  } 
};
</script>
<style scoped>
</style>
