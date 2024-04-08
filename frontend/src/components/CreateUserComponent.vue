<script setup lang="ts">
import { ref } from "vue";
import { type User, useUserStore } from "@/stores/user_store";

const userStore = useUserStore();

const createUserLogin = ref("");
const createUserPassword = ref("");
const createUserError = ref<undefined | string>(undefined);

const emits = defineEmits<{ (e: "created", user: User): void }>();
const handleCreateUser = async () => {
  createUserError.value = undefined;
  try {
    let newUser = await userStore.create({
      login: createUserLogin.value,
      hash: createUserPassword.value,
    });
    newUser.groups = await userStore.getUserGroups(newUser.id);
    createUserLogin.value = "";
    createUserPassword.value = "";
    emits("created", newUser);
  } catch (e) {
    createUserError.value = e as string;
  }
};
</script>

<template>
  <q-card dark>
    <q-card-section v-if="createUserError !== undefined">
      {{ createUserError }}
    </q-card-section>
    <q-card-section>
      User login
      <q-input dark type="text" v-model="createUserLogin" />
    </q-card-section>
    <q-card-section>
      User passord
      <q-input dark type="password" v-model="createUserPassword" />
    </q-card-section>
    <q-card-section class="sectionToTheRight">
      <q-btn icon="add" @click="handleCreateUser" />
    </q-card-section>
    <q-card-section>
    </q-card-section>
  </q-card>
</template>

<style scoped>
.sectionToTheRight {
  width: 100%;
  display: flex;
  justify-content: end;
}
</style>