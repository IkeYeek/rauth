<script async setup lang="ts">
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth_store";
import { ref } from "vue";

const authStore = useAuthStore();
const hasError = ref(false);
try {
  await authStore.isAuth();
} catch (e) {
  hasError.value = true;
  console.error(e);
}
</script>

<template>
  <main>
    <div>
      <RouterView v-if="!hasError" />
      <template v-else>
        Couldn't connect to API.
      </template>
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
