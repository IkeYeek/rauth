import { createRouter, createWebHistory } from "vue-router";
import AuthView from "@/views/AuthView.vue";
import { useUserStore } from "@/stores/user_store";
import { useAuthStore } from "@/stores/auth_store";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "auth",
      path: "/auth",
      component: AuthView,
    },
  ],
});
router.beforeEach(async (to, from) => {
  const authStore = useAuthStore();
  const authenticated = await authStore.isAuth();
  if (!authenticated && to.name !== "auth") {
    return {
      name: "auth",
    };
  }
});
export default router;
