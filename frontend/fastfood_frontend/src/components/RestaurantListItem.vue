<template>
    <div class="flex flex-row p-1 m-1 bg-myorange+2 shadow-md rounded-md font-buttonfont font-normal font-white">
        <div class="flex flex-col">
            <div>
                <p class="px-3 font-semibold">{{item.name}}</p>
            </div> 
            <div class="flex flex-row">
                <p class="px-3">Google Location Code: {{item.location}}</p>
                <p class="px-3">Creator: {{item.first_name}} {{item.last_name}}</p>
                <p class="px-3">Average Rating: {{item.restaurant_rating}}</p>
                <!-- <p class="px-3">Average Price</p> -->
            </div>
   



        </div>
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