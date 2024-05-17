<script setup lang="ts">
import {
  type DomainRule,
  type NewDomainRule,
  type NewUrlRule,
  type UrlRule,
  useRulesStore,
} from "@/stores/rules_store";
import { onMounted, ref } from "vue";
import { type Group, useGroupStore } from "@/stores/group_store";

const rulesStore = useRulesStore();
const groupStore = useGroupStore();
const groups = ref<Array<Group>>([]);
const groupModel = ref(null);
const createRuleError = ref<undefined | string>(undefined);
const props = defineProps<{
  type: "DomainRule" | "UrlRule";
}>();
const ruleValue = ref("");

const emits = defineEmits<{
  (e: "created", rule: DomainRule | UrlRule, kind: "DomainRule" | "UrlRule"): void;
}>();
const handleCreateRule = async () => {
  createRuleError.value = undefined;
  try {
    if (props.type === "DomainRule") {
      let rule = await rulesStore.addDomainRule({
        domain: ruleValue.value,
        group_id: groupModel.value!.id,
      });
      emits("created", rule, "DomainRule");
    } else {
      let rule = await rulesStore.addUrlRule({
        url: ruleValue.value,
        group_id: groupModel.value!.id,
      });
      emits("created", rule, "UrlRule");
    }
  } catch (e) {
    createRuleError.value = e as string;
  }
};
const filterFn = (val: string, update: (cb: () => void) => void, _: () => void) => {
  update(() => {
    const needle = val.toLowerCase();
    groups.value = groups.value.filter((v) => v.name.toLowerCase().indexOf(needle) > -1);
  });
};
onMounted(async () => {
  groups.value = await groupStore.getAll();
});
</script>

<template>
  <q-card dark>
    <q-card-section v-if="createRuleError !== undefined">
      {{ createRuleError }}
    </q-card-section>
    <q-card-section>
      Rule
      <template v-if="props.type === 'DomainRule'">Domain</template>
      <template v-else>URL</template>
      <q-input dark type="text" v-model="ruleValue" />
    </q-card-section>
    <q-card-section>
      <q-select
        v-model="groupModel"
        filled
        dark
        use-input
        hide-selected
        input-debounce="0"
        fill-input
        :options="groups"
        @filter="filterFn"
        hint="rule group"
        label="rule group"
        option-value="id"
        option-label="name"
      />
    </q-card-section>
    <q-card-section class="sectionToTheRight">
      <q-btn icon="add" @click="handleCreateRule" />
    </q-card-section>
  </q-card>
</template>

<style scoped></style>
