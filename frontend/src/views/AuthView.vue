<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useAuthStore } from "@/stores/auth_store";

const loading = ref(false);
const error = ref<string | undefined>(undefined);

const login = ref("");
const password = ref("");

const authStore = useAuthStore();

const tryAuth = async (e: MouseEvent) => {
  e.preventDefault();
  error.value = undefined;
  if (login.value.length < 3) {
    error.value = "missing login";
  }
  if (password.value.length < 3) {
    error.value =
      error.value === undefined ? "missing password" : `${error.value} + missing password`;
  }
  if (error.value !== undefined) return;
  loading.value = true;
  try {
    await authStore.tryAuth(login.value, password.value);
    login.value = "";
  } catch (e) {
    error.value = e as unknown as string;
  } finally {
    loading.value = false;
    password.value = "";
  }
};
</script>

<template>
  <div id="parent">
    <img src="https://ike.icu/assets/logo-mT7adExh.png" alt=" logo" id="logo" />
    <form id="form">
      <template v-if="loading"> loading...</template>
      <template v-else-if="authStore.authed"
      ><p>Already authenticated</p>
        <button @click="authStore.logOut()">logout</button>
      </template
      >
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
        <input type="submit" value="try auth" @click="tryAuth" />
      </template>
    </form>
  </div>
</template>

<style scoped>
#parent {
  background-color: #231531bf;
  border-radius: 5px;
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
