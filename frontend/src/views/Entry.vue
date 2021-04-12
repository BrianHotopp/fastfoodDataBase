<template>
  <div v-if="entry" class="entry">
    <!-- food name
    food description
    food price
    food calories
    food rating
    added by
    creation date
    update date
    restaurant name
    link to restaurant page
    trust score
    reviews -->
    <section class="p-1 m-1 bg-white shadow-md">
      <header>
        {{entry.food_title}}
      </header>
      <article>
        {{entry.description}}
      </article>
      <article>
        <ul>
          <li>
            {{entry.price}}
          </li>
          <li>
            {{entry.calories}}
          </li>
          <li>
            {{entry.restaurant_name}}
          </li>
        </ul>
      </article>
      <article>
        <trust-panel></trust-panel>
      </article>
      <footer>
        <ul>
          <li>
            Last Updated: {{entry.update_date | readable_timestamp}}
          </li>
          <li>
            Added by: {{entry.creator_first_name}}
          </li>
          <li>
            On: {{entry.creation_date | readable_timestamp}}
          </li>
        </ul>
      </footer>
    </section>



  </div>
</template>

<script>
import axios from 'axios';
import TrustPanel from '../components/TrustPanel.vue';
export default {
  name: "Entry",
  components: {
    TrustPanel
  },
  data(){
      return {
        entry: null,
        comments: [
        { user: 'John A. Smith', rating: 5, comment: 'CA' },
        { user: 'Joan B. Jones', rating: 5, comment: 'NY' },
        { user: 'Bob C. Uncle', rating: 5, comment: 'CA' }
        ]
      }
  },
  methods:{
    
  },
  mounted: function(){
    axios.get(
      'http://localhost:8000/menuentries/'+this.$route.params.entry_id
    )
    .then(response => (this.entry = response.data[0]))
    .catch(error => {console.log(error);});
  }

};
</script>
