<script setup lang="ts">
import { ref } from "vue";
import { useAuthStore } from "@/stores/auth_store";
import { useRoute, useRouter } from "vue-router";

const loading = ref(false);
const error = ref<string | undefined>(undefined);

const login = ref("");
const password = ref("");

const authStore = useAuthStore();

const waitForAuth = (e: MouseEvent, iterations = 5) => {
  setTimeout(async () => {
    if (!await authStore.isAuth()) {
      if (iterations > 0) waitForAuth(e, iterations - 1);
      else throw new Error("auth timeout");
    }
  }, 250);
};
</script>

<template>
  <div id="parent">
    <img src="https://ike.icu/assets/logo-mT7adExh.png" alt="logo" id="logo" />
    <form id="form" action="http://localhost.dummy:8080/auth" method="post" target="_blank" @click="waitForAuth">
      <template v-if="loading"> loading...</template>
      <template v-else-if="authStore.authed"
      ><p>Already authenticated</p>
        <button @click="authStore.logOut()" type="button">logout</button>
      </template>
      <template v-else>
        <p class="error" v-if="error !== undefined">{{ error }}</p>
        <label for="login">login: </label>
        <input type="text" name="login" placeholder="login" id="login" v-model="login" />
        <label for="password">Password: </label>
        <input
          type="password"
          name="password"
          placeholder="password"
          id="password"
          v-model="password"
        />
        <input type="submit" value="try auth" />
      </template>
    </form>
  </div>
</template>

<style scoped>
#parent {
  display: flex;
  flex-direction: column;
  max-width: 400px;
  max-height: 400px;
}

#form {
  display: flex;
  flex-direction: column;
  padding: 0 50px 20px 50px;
}

#form > * {
  margin-bottom: 10px;
}

#logo {
  width: 50px;
  height: 50px;
  align-self: end;
  padding: 5px 5px 0 0;
}
</style>
