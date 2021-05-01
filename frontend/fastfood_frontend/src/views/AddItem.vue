<template>
  <div class="entries flex flex-col my-6 justify-start bg-bgcolor+1 rounded-md">
   <form id="demo">
  <!-- text -->
  <p class="px-5 py-3">
    <label for='itemname' class='font-buttonfont font-medium text-myorange'>Item Name:</label><br>
    <input v-model="msg" type="text" id="itemname" class='font-buttonfont font-medium text-myorange bg-bgcolor border border-black rounded-md'><br>
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
  <div class="px-2 py-10">
  <p class='font-buttonfont font-semibold text-myorange'>Raw Data<br><pre class="bg-bgcolor text-white rounded-md text-base font-normal">{{$data | json 2}}</pre></p>
  </div>
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
        msg      : '',
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

