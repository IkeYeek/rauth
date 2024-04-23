<script setup lang="ts">
import { type Group, useGroupStore } from "@/stores/group_store";
import { onMounted, ref } from "vue";
import type { QTableColumn } from "quasar";
import type { User } from "@/stores/user_store";
import CreateGroupComponent from "@/components/CreateGroupComponent.vue";
import EditGroupComponent from "@/components/EditGroupComponent.vue";

const groupStore = useGroupStore();
const groups = ref<Array<Group>>([]);
const error = ref<string | undefined>(undefined);
const createGroupDialog = ref(false);
const columns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "Group ID",
    field: "id",
    align: "left",
    sortable: true,
  },
  {
    name: "name",
    label: "Group Name",
    field: "name",
    align: "left",
    sortable: true,
  },
  {
    name: "users",
    label: "Users",
    align: "center",
    field: (row: Group) => row.users,
    format: (us: Array<User>) => {
      const maxUsersToShow = 7;
      const usersToShow = us.slice(0, maxUsersToShow);
      const remainingUsers = us.length - maxUsersToShow;
      const formattedUsers = usersToShow.reduce((acc, curr, idx) => {
        return acc + `${idx > 0 ? ", " : ""} ${curr.login}`;
      }, "");

      return remainingUsers > 0 ? `${formattedUsers}, ...` : formattedUsers;
    },
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

const editGroupDialog = ref(false);
const editedGroup = ref<undefined | Group>(undefined);

const handleDeleteGroup = async (group: Group) => {
  error.value = undefined;
  try {
    await groupStore.remove(group.id);
    groups.value = groups.value.filter((g) => g.id !== group.id);
  } catch (e) {
    error.value = e as string;
  }
};

const handleUpdatedGroup = (updatedGroup: Group) => {
  const index = groups.value.findIndex((g) => g.id === updatedGroup.id);
  groups.value[index] = updatedGroup;
  editGroupDialog.value = false;
};
const handleCreateGroup = (newGroup: Group) => {
  groups.value.push(newGroup);
  createGroupDialog.value = false;
};
const openGroup = async (group_id: number) => {
  try {
    editedGroup.value = await groupStore.get(group_id);
    editGroupDialog.value = true;
  } catch (e) {
    error.value = e as string;
  }
};

onMounted(async () => {
  groups.value = await groupStore.getAll();
});
</script>

<template>
  <div id="parent">
    <q-table
      style="height: 400px"
      dark
      flat
      bordered
      title="Groups"
      :rows="groups"
      :columns="columns"
      row-key="id"
      virtual-scroll
      :pagination="pagination"
      :rows-per-page-options="[0]"
    >
      <template v-slot:body-cell-actions="props">
        <q-td :props="props">
          <q-btn flat color="warning" icon="edit" @click="openGroup(props.row.id)" />
          <q-btn flat color="negative" icon="delete" @click="handleDeleteGroup(props.row)" />
        </q-td>
      </template>
      <template #top-right>
        <p class="error" v-if="error">{{ error }}</p>
      </template>
      <template #no-data>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="createGroupDialog = true" />
            </div>
          </q-td>
        </q-tr>
      </template>
      <template #bottom>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="createGroupDialog = true" />
            </div>
          </q-td>
        </q-tr>
      </template>
    </q-table>
    <q-dialog v-model="createGroupDialog">
      <CreateGroupComponent @created="handleCreateGroup" />
    </q-dialog>
    <q-dialog v-model="editGroupDialog">
      <EditGroupComponent :group="editedGroup!" @updated="handleUpdatedGroup" />
    </q-dialog>
  </div>
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
