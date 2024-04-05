<script setup lang="ts">
import {computed, ref} from "vue";
import {useAuthStore} from "@/stores/auth_store";

const loading = ref(false);
const error = ref<string | undefined>(undefined);

const login = ref("");
const password = ref("");

const authStore = useAuthStore();

const tryAuth = async () => {
  if (login.value.length + password.value.length > 1) {
    loading.value = true;
    try {
      await authStore.tryAuth(login.value, password.value);
    } catch (e) {
      error.value = e as unknown as string;
    } finally {
      loading.value = false;
    }
  }
}
</script>

<template>
  <div id="parent">
    <img src="https://ike.icu/assets/logo-mT7adExh.png" alt=" logo" id="logo"/>
    <div id="form">
      <template v-if="loading">
        loading...
      </template>
      <template v-else>
        <p class="error" v-if="error !== undefined">{{ error }}</p>
        <label for="login">login: </label>
        <input type="text" name="login" placeholder="login" id="login" v-model="login"/>
        <label for="password">Password: </label>
        <input type="password" name="password" placeholder="password" id="password" v-model="password"/>
        <input type="submit" value="try auth" @click="tryAuth"/>
      </template>
    </div>
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
