<script setup lang="ts">
import {
  type DomainRule,
  type NewDomainRule,
  type NewUrlRule,
  type UrlRule,
  useRulesStore,
} from "@/stores/rules_store";
import { onMounted, ref } from "vue";
import type { QTableColumn } from "quasar";
import CreateRuleComponent from "@/components/CreateRuleComponent.vue";

const rulesStore = useRulesStore();
const urlRules = ref<Array<UrlRule>>([]);
const domainRules = ref<Array<DomainRule>>([]);
const typeToCreate = ref<"DomainRule" | "UrlRule" | undefined>(undefined);
const error = ref<string | undefined>(undefined);
const createRuleDialog = ref(false);
const urlRulesColumns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "Rule ID",
    field: "id",
    align: "left",
    sortable: true,
  },
  {
    name: "url",
    label: "URL",
    field: "url",
    align: "left",
    sortable: true,
  },
  {
    name: "group",
    label: "Groups",
    field: "group_id",
    align: "left",
    sortable: true,
  },
  {
    name: "actions",
    label: "Actions",
    align: "center",
    field: () => "actions",
  },
]);

const domainRulesColumns = ref<QTableColumn[]>([
  {
    name: "id",
    label: "Rule ID",
    field: "id",
    align: "left",
    sortable: true,
  },
  {
    name: "domain",
    label: "Domain",
    field: "domain",
    align: "left",
    sortable: true,
  },
  {
    name: "group",
    label: "Groups",
    field: "group_id",
    align: "left",
    sortable: true,
  },
  {
    name: "actions",
    label: "Actions",
    align: "center",
    field: () => "actions",
  },
]);

const urlRulesPagination = ref({
  rowsPerPage: 0,
});
const domainRulesPagination = ref({
  rowsPerPage: 0,
});

const handleDeleteUrlRule = async (rule: UrlRule) => {
  error.value = undefined;
  try {
    await rulesStore.removeUrlRule(rule.id);
    urlRules.value = urlRules.value.filter((r) => r.id !== rule.id);
  } catch (e) {
    error.value = e as string;
  }
};

const handleDeleteDomainRule = async (rule: DomainRule) => {
  error.value = undefined;
  try {
    await rulesStore.removeDomainRule(rule.id);
    domainRules.value = domainRules.value.filter((r) => r.id !== rule.id);
  } catch (e) {
    error.value = e as string;
  }
};

const handleCreateUrlRule = async (r: NewUrlRule) => {
  error.value = undefined;
  try {
    const rule = await rulesStore.addUrlRule(r);
    urlRules.value.push(rule);
  } catch (e) {
    error.value = e as string;
  }
};

const handleCreateDomainRule = async (r: NewDomainRule) => {
  error.value = undefined;
  try {
    const rule = await rulesStore.addDomainRule(r);
    domainRules.value.push(rule);
  } catch (e) {
    error.value = e as string;
  }
};

const handleCreateRule = (type: "DomainRule" | "UrlRule") => {
  createRuleDialog.value = true;
  typeToCreate.value = type;
};

const createRuleCallback = (rule: DomainRule | UrlRule, type: "DomainRule" | "UrlRule") => {
  createRuleDialog.value = false;
  typeToCreate.value = undefined;
  if (type === "DomainRule") {
    domainRules.value.push(rule as DomainRule);
  } else {
    urlRules.value.push(rule as UrlRule);
  }
};

onMounted(async () => {
  urlRules.value = await rulesStore.getUrlRules();
  domainRules.value = await rulesStore.getDomainRules();
});
</script>

<template>
  <div id="parent">
    <q-table
      style="height: 400px"
      dark
      flat
      bordered
      title="URL Rules"
      :rows="urlRules"
      :columns="urlRulesColumns"
      row-key="id"
      virtual-scroll
      :pagination="urlRulesPagination"
      :rows-per-page-options="[0]"
    >
      <template #bottom>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="() => handleCreateRule('UrlRule')" />
            </div>
          </q-td>
        </q-tr>
      </template>
      <template #no-data>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="() => handleCreateRule('UrlRule')" />
            </div>
          </q-td>
        </q-tr>
      </template>
      <template #body-cell-actions="props">
        <q-td :props="props">
          <q-btn flat color="danger" icon="delete" @click="() => handleDeleteUrlRule(props.row)" />
        </q-td>
      </template>
    </q-table>
    <q-table
      style="height: 400px"
      dark
      flat
      bordered
      title="Domain Rules"
      :rows="domainRules"
      :columns="domainRulesColumns"
      row-key="id"
      virtual-scroll
      :pagination="domainRulesPagination"
      :rows-per-page-options="[0]"
    >
      <template #bottom>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="() => handleCreateRule('DomainRule')" />
            </div>
          </q-td>
        </q-tr>
      </template>
      <template #no-data>
        <q-tr>
          <q-td colspan="100%">
            <div class="sectionToTheRight">
              <q-btn icon="add" @click="() => handleCreateRule('DomainRule')" />
            </div>
          </q-td>
        </q-tr>
      </template>
      <template #body-cell-actions="props">
        <q-td :props="props">
          <q-btn flat color="danger" icon="delete" @click="handleDeleteDomainRule(props.row)" />
        </q-td>
      </template>
    </q-table>
    <q-dialog v-model="createRuleDialog" v-if="typeToCreate">
      <CreateRuleComponent :type="typeToCreate" @created="createRuleCallback" />
    </q-dialog>
  </div>
</template>

<style scoped>
#parent {
  display: flex;
}

@media (max-width: 800px) {
  #parent {
    flex-direction: column;
  }
}

.sectionToTheRight {
  width: 100%;
  display: flex;
  justify-content: flex-end;
}
</style>
