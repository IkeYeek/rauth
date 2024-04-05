import { defineStore } from "pinia";

import { ref } from "vue";
export const useEnvStore = defineStore("env", () => {
  const app_base = ref<string>("http://localhost:8080/");

  return {
    app_base,
  };
});
