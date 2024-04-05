<script setup lang="ts">

import App from "./App.vue";
</script>

<template>
  <Suspense>
    <template #default>
      <App />
    </template>
    <template #fallback><div id="loading">
      <div>
        Loading...
      </div>
    </div></template>
  </Suspense>
</template>

<style scoped>
#loading {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
#loading > div {
  background-color: #231531bf;
  border-radius: 5px;
  width: 90%;
  height: 90%;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>