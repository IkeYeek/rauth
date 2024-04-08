<script setup lang="ts">
import { ref } from "vue";
import { type Group, useGroupStore } from "@/stores/group_store";

const groupStore = useGroupStore();

const createGroupName = ref("");
const createGroupError = ref<undefined | string>(undefined);

const emits = defineEmits<{ (e: "created", group: Group): void }>();

const handleCreateGroup = async () => {
  createGroupError.value = undefined;
  try {
    let newGroup = await groupStore.create({ name: createGroupName.value });
    newGroup.users = await groupStore.listUsersFrom(newGroup.id);
    createGroupName.value = "";
    emits("created", newGroup);
  } catch (e) {
    createGroupError.value = e as string;
  }
};
</script>

<template>
  <q-card dark>
    <q-card-section v-if="createGroupError !== undefined">
      {{ createGroupError }}
    </q-card-section>
    <q-card-section>
      Group name
      <q-input dark type="text" v-model="createGroupName" />
    </q-card-section>
    <q-card-section class="sectionToTheRight">
      <q-btn icon="group_add" @click="handleCreateGroup" />
    </q-card-section>
  </q-card>
</template>

<style scoped></style>
