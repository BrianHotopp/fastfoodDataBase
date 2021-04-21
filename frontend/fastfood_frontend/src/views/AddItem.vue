<template>
  <div class="entries flex flex-col my-6 justify-start bg-white">
   <form id="demo">
  <!-- text -->
  <p>
    <input type="text" v-model="msg">
    {{msg}}
  </p>
  <!-- checkbox -->
  <p>
    <input type="checkbox" v-model="checked">
    {{checked ? "yes" : "no"}}
  </p>
  <!-- radio buttons -->
  <p>
    <input type="radio" name="picked" value="one" v-model="picked">
    <input type="radio" name="picked" value="two" v-model="picked">
    {{picked}}
  </p>
  <!-- select -->
  <p>
    <select v-model="selected">
      <option>one</option>
      <option>two</option>
    </select>
    {{selected}}
  </p>
  <!-- multiple select -->
  <p>
    <select v-model="multiSelect" multiple>
      <option>one</option>
      <option>two</option>
      <option>three</option>
    </select>
    {{multiSelect}}
  </p>
  <p><pre>data: {{$data | json 2}}</pre></p>
</form> 
    

  </div>
</template>

<script>
import axios from 'axios';
export default {
  name: "AddItem",
  components: {
  },
  data(){
      return{
        msg      : 'hi!',
        checked  : true,
        picked   : 'one',
        selected : 'two',
        multiSelect: ['one', 'three']
      }
  },
  methods:{
    hit_db(){
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
    //this.hit_db();
  } 
};
</script>
<style scoped>
</style>

