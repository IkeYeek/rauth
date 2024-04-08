<script setup async lang="ts">
import type { UpdateUserPayload, User } from "@/stores/user_store";
import { onMounted, ref, watch } from "vue";
import { useUserStore } from "@/stores/user_store";
import type { QTableColumn } from "quasar";
import { type Group, useGroupStore } from "@/stores/group_store";

type EditUserProps = {
  user: User;
};
const emits = defineEmits<{
  (e: "updated", user: User): void;
}>();
const props = defineProps<EditUserProps>();
const user = ref<User>(props.user);
const error = ref<string | undefined>(undefined);

const groupStore = useGroupStore();
const userStore = useUserStore();

const userNewPassword = ref<string>("");
const userGroups = ref<Array<Group>>([]);
const groupsModel = ref(null);

const pagination = ref({
  rowsPerPage: 0,
});
const columns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "Group ID",
    field: (row) => row.id,
  },
  {
    name: "name",
    label: "Group Name",
    field: (row) => row.name,
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
    userGroups.value = userGroups.value.filter((v) => v.name.toLowerCase().indexOf(needle) > -1);
  });
};

const updateAvailableGroups = async () => {
  const groups = await groupStore.getAll();
  userGroups.value = groups.filter((group) => {
    return !user.value.groups!.some((g) => g.id === group.id);
  });
};

const addUserToGroup = async () => {
  user.value.groups!.push(groupsModel.value!);
  groupsModel.value = null;
  await updateAvailableGroups();
};

const removeUserFromGroup = async (group: Group) => {
  user.value.groups = user.value.groups!.filter((g) => g.id !== group.id);
  await updateAvailableGroups();
};
const updateUser = async () => {
  error.value = undefined;
  let payload: UpdateUserPayload = {
    new_login: user.value!.login,
    new_hash: undefined,
  };
  if (userNewPassword.value.length > 3) {
    payload["new_hash"] = userNewPassword.value;
  }
  try {
    await userStore.update(user.value.id, payload, user.value.groups!);
    emits("updated", user.value);
  } catch (e) {
    error.value = e as string;
  }
};

onMounted(async () => {
  await updateAvailableGroups();
});
</script>

<template>
  <q-card dark>
    <q-card-section v-if="error">
      <div class="text-right error">{{ error }}</div>
    </q-card-section>
    <q-card-section>
      <q-input dark v-model="user.id" readonly label="Id" />
      <q-input dark v-model="user.login" label="Name" />
      <q-input dark v-model="userNewPassword" label="New Password" />
    </q-card-section>
    <q-card-section>
      <q-table
        dark
        flat
        bordered
        title="Groups"
        :rows="[...user.groups]"
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
              v-model="groupsModel"
              dark
              use-input
              hide-selected
              input-debounce="0"
              fill-input
              :options="userGroups"
              @filter="filterFn"
              hint="user group"
              label="user group"
              option-value="id"
              option-label="name"
            />
            <q-btn icon="add" @click="addUserToGroup" dark />
          </q-btn-group>
        </template>
        <template #body-cell-actions="props">
          <q-td :props="props">
            <q-btn
              icon="remove_circle_outline"
              @click="() => removeUserFromGroup(props.row)"
              dark
            />
          </q-td>
        </template>
      </q-table>
    </q-card-section>
    <q-card-section>
      <q-btn @click="() => updateUser()">Mettre à jour</q-btn>
    </q-card-section>
  </q-card>
</template>

<style scoped></style>
