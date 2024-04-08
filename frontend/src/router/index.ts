import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "@/stores/auth_store";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "auth",
      path: "/auth",
      component: () => import("@/views/AuthView.vue"),
    },
    {
      name: "users",
      path: "/users",
      component: () => import("@/views/UsersView.vue"),
    },
    {
      name: "groups",
      path: "/groups",
      component: () => import("@/views/GroupsView.vue"),
    },
  ],
});
router.beforeEach(async (to, from) => {
  const authStore = useAuthStore();
  let authenticated = false;
  try {
    authenticated = await authStore.isAuth();
  } catch (e) {
    console.error(e);
    //TODO redirect to 500
  }
  if (!authenticated && to.name !== "auth") {
    return {
      name: "auth",
    };
  }
});
export default router;
