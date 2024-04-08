import { createRouter, createWebHistory } from "vue-router";
import AuthView from "@/views/AuthView.vue";
import { useAuthStore } from "@/stores/auth_store";
import UsersView from "@/views/UsersView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "auth",
      path: "/auth",
      component: AuthView,
    },
    {
      name: "users",
      path: "/users",
      component: UsersView,
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
