<script async setup lang="ts">
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth_store";
import { ref } from "vue";
import Header from "@/components/Header.vue";

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
  <header>
    <Header />
  </header>
  <main>
    <div>
      <RouterView v-if="!hasError" />
      <template v-else> Couldn't connect to API.</template>
    </div>
  </main>
</template>

<style scoped>
header {
  height: 10vh;
}

main {
  width: 100vw;
  height: 90vh;
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
