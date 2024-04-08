<script async setup lang="ts">
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth_store";

const authStore = useAuthStore();
await authStore.isAuth();
</script>

<template>
  <main>
    <div>
      <RouterView />
    </div>
  </main>
</template>

<style scoped>
main {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

main > div {
  background-color: #231531bf;
  border-radius: 5px;
  padding: 10px;
}
</style>
