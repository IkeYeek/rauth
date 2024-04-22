<script setup async lang="ts">
import { useUserStore } from "@/stores/user_store";
import { ref } from "vue";

const userStore = useUserStore();
const userData = await userStore.getUserData();

const newLogin = ref(userData.user.login);
const newPassword = ref("");

console.log(userData);
</script>

<template>
  <q-card dark>
    <q-card-section>
      My Account
    </q-card-section>
    <q-card-section>
      <q-input v-model="newLogin" label="Login" dark disable />
    </q-card-section>
    <q-card-section>
      <q-input v-model="newPassword" label="Password" dark />
    </q-card-section>
    
  </q-card>
</template>

<style scoped>

</style>