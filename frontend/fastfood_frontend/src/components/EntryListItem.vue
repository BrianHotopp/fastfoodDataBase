<template>
    <div class="flex flex-row p-1 m-1 bg-myorange+2 shadow-md rounded-md font-buttonfont font-normal font-white">
        <div class="flex flex-col">
            <div>
                <p class="px-3 font-semibold">{{item.food_title}}</p>
            </div> 
            <div class="flex flex-row">
                <p class="px-3">{{item.restaurant_name}}</p>
                <p class="px-3">Price: {{item.price}}</p>
                <p class="px-3">Calories: {{item.calories}}</p>

                <p class="px-3" v-if="item.rating">Rating: {{item.rating}}</p>
                <p class="px-3" v-else>No Ratings</p>
            </div>
   



        </div>
        <div></div>
        <div></div>
        <div></div>
    </div>
</template>
<script>
export default {
    data(){
        return{

        }
    },
    props:{
        item:Object
    }

}
</script>
