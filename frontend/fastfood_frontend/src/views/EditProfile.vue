<template>
  <div>
    <div class='px-5 py-5'>
      <div class='bg-bgcolor+1 px-5 py-5 m-1 rounded-md'>
        <img :src="$auth.user.picture" class='border border-myorange'>
        <br>
        <p class='font-buttonfont font-semibold text-myorange text-xl'>Name</p>
        <p class='font-buttonfont text-white'>{{ $auth.user.name }}</p>
        <br>
        <p class='font-buttonfont font-semibold text-myorange text-xl'>Email</p>
        <p class='font-buttonfont text-white'>{{ $auth.user.email }}</p>
        <br>
        <p class='font-buttonfont font-semibold text-myorange text-xl'>Nickname</p>
        <p class='font-buttonfont text-white'>{{ $auth.user.nickname }}</p>
        <br>
        <p class='font-buttonfont font-semibold text-myorange text-xl'>Raw Data</p>
        <div class='bg-bgcolor text-white rounded-md'>
        <pre>{{ JSON.stringify($auth.user, null, 2) }}</pre>
        </div>
        <ul class="p-3 flex justify-left">
          <li class="py-1 px-5 my-1 mr-1 rounded-md flex flex-col justify-center hover:bg-myorange+1 bg-myorange font-buttonfont font-semibold">
            <router-link to="/profile">Save and Exit</router-link>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>