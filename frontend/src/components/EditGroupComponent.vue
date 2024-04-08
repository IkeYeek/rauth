<script setup lang="ts">
import { type Group, useGroupStore } from "@/stores/group_store";
import { onMounted, ref } from "vue";
import { type User, useUserStore } from "@/stores/user_store";
import type { QTableColumn } from "quasar";

type EditGroupProps = {
  group: Group;
};
const emits = defineEmits<{
  (e: "updated", group: Group): void;
}>();
const props = defineProps<EditGroupProps>();
const group = ref<Group>(props.group);
const error = ref<string | undefined>(undefined);

const groupStore = useGroupStore();
const userStore = useUserStore();

const groupUsers = ref<Array<User>>([]);
const usersModel = ref(null);

const pagination = ref({
  rowsPerPage: 0,
});

const columns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "User ID",
    field: (row) => row.id,
  },
  {
    name: "login",
    label: "User login",
    field: (row) => row.login,
  },
  {
    name: "actions",
    label: "Actions",
    field: () => "Actions",
  },
]);

const filterFn = (val: string, update: (cb: () => void) => void, _: () => void) => {
  update(() => {
    const needle = val.toLowerCase();
    groupUsers.value = groupUsers.value.filter((v) => v.login.toLowerCase().indexOf(needle) > -1);
  });
};

const updateAvailableUsers = async () => {
  const users = await userStore.getAll();
  groupUsers.value = users.filter((user) => {
    return !group.value.users!.some((u) => u.id === user.id);
  });
};

const addGroupToUser = async () => {
  group.value.users!.push(usersModel.value!);
  usersModel.value = null;
  await updateAvailableUsers();
};

const removeGroupFromUser = async (user: User) => {
  group.value.users = group.value.users!.filter((u) => u.id !== user.id);
  await updateAvailableUsers();
};

const updateGroup = async () => {
  error.value = undefined;
  try {
    await groupStore.update(group.value.id, { new_name: group.value.name }, group.value.users!);
    emits("updated", group.value);
  } catch (e) {
    error.value = e as string;
  }
};

onMounted(async () => {
  await updateAvailableUsers();
});
</script>

<template>
  <q-card dark>
    <q-card-section v-if="error">
      <div class="text-right error">{{ error }}</div>
    </q-card-section>
    <q-card-section>
      <q-input dark v-model="group.id" readonly label="Id" />
      <q-input dark v-model="group.name" label="Name" />
    </q-card-section>
    <q-card-section>
      <q-table
        dark
        flat
        bordered
        title="Users"
        :rows="[...group.users]"
        :columns="columns"
        row-key="id"
        virtual-scroll
        :pagination="pagination"
        :rows-per-page-options="[0]"
      >
        <template #bottom-row>
          <q-btn-group>
            <q-select
              filled
              v-model="usersModel"
              dark
              use-input
              hide-selected
              input-debounce="0"
              fill-input
              :options="groupUsers"
              @filter="filterFn"
              hint="user group"
              label="user group"
              option-value="id"
              option-label="login"
            />
            <q-btn icon="add" @click="addGroupToUser" dark />
          </q-btn-group>
        </template>
        <template #body-cell-actions="props">
          <q-td :props="props">
            <q-btn
              icon="remove_circle_outline"
              @click="() => removeGroupFromUser(props.row)"
              dark
            />
          </q-td>
        </template>
      </q-table>
    </q-card-section>
    <q-card-section>
      <q-btn @click="() => updateGroup()">Mettre à jour</q-btn>
    </q-card-section>
  </q-card>
</template>

<style scoped></style>
