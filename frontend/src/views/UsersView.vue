<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useUserStore } from "@/stores/user_store";
import type { User } from "@/stores/user_store";
import type { QTableColumn } from "quasar";
import type { Group } from "@/stores/group_store";
import CreateUserComponent from "@/components/CreateUserComponent.vue";
import EditUserComponent from "@/components/EditUserComponent.vue";

const userStore = useUserStore();

const createUserDialog = ref(false);
const deleteUserError = ref<undefined | string>(undefined);

const editUserDialog = ref(false);
const editedUser = ref<undefined | User>(undefined);

const error = ref<string | undefined>(undefined);
const users = ref<Array<User>>([]); // Initialize with an empty array
const columns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "User ID",
    field: "id",
    align: "left",
    sortable: true,
  },
  {
    name: "login",
    label: "Login",
    align: "left",
    field: (row: User) => row.login,
    format: (val: string) => `${val}`,
    sortable: true,
  },
  {
    name: "groups",
    label: "Groups",
    align: "center",
    field: (row: User) => row.groups,
    format: (gs: Array<Group>) => gs.reduce((acc, curr, idx) => {
      return acc + `${idx > 0 ? ", " : ""} ${curr.name}`;
    }, ""),
  },
  {
    name: "actions",
    label: "Actions",
    align: "center",
    field: () => "actions",
  },
]);

const pagination = ref({
  rowsPerPage: 0,
});

watch(() => createUserDialog.value, (value) => {
  if (!value) {
    editedUser.value = undefined;
  }
});

onMounted(async () => {
  users.value = await userStore.getAll();
});

const handleDeleteUser = async (toRemove: User) => {
  deleteUserError.value = undefined;
  try {
    await userStore.remove(toRemove.id);
    users.value = users.value.filter(user => user !== toRemove);
  } catch (e) {
    error.value = (e as Error).message;
  }
};

const handleUpdatedUser = async (updatedUser: User) => {
  const newValue = await userStore.get(updatedUser.id);
  users.value = users.value.map(user => user.id === updatedUser.id ? newValue : user);
  editUserDialog.value = false;
};

const handleCreatedUser = (newUser: User) => {
  users.value.push(newUser);
  createUserDialog.value = false;
};

const openUser = async (user_id: number) => {
  try {
    editedUser.value = await userStore.get(user_id);
    editUserDialog.value = true;
  } catch (e) {
    error.value = (e as Error).message;
  }
};

</script>

<template>
  <div id="parent">
    <template v-if="users.length > 0">
      <q-table
        style="height: 400px"
        dark
        flat bordered
        title="Users"
        :rows="users"
        :columns="columns"
        row-key="id"
        virtual-scroll
        :pagination="pagination"
        :rows-per-page-options="[0]">
        <template #top-right>
          <p class="error" v-if="error">{{ error }}</p>
        </template>
        <template v-slot:body-cell-actions="props">
          <q-td :props="props">
            <q-btn icon="edit" @click="() => {openUser(props.row.id)}">admin</q-btn>
            <q-btn icon="delete" @click="handleDeleteUser(props.row)">delete user</q-btn>
          </q-td>
        </template>
        <template #bottom>
          <div class="sectionToTheRight">
            <q-btn icon="add" @click="createUserDialog = true"></q-btn>
          </div>
        </template>
      </q-table>
    </template>
  </div>
  <q-dialog v-model="createUserDialog">
    <CreateUserComponent @created="user => handleCreatedUser(user)" />
  </q-dialog>
  <q-dialog v-model="editUserDialog">
    <EditUserComponent :user="editedUser" v-if="editedUser !== undefined" @updated="user => handleUpdatedUser(user)" />
    <template v-else>Error, user to edit is undefined !</template>
  </q-dialog>
</template>

<style scoped>
#parent {
  min-width: 400px;
}

.sectionToTheRight {
  width: 100%;
  display: flex;
  justify-content: end;
}
</style>
