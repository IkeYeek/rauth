<script setup async lang="ts">
import { useAuthStore } from "@/stores/auth_store";
import { ref } from "vue";

const authStore = useAuthStore();
</script>

<template>
  <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
    rauth
    <template v-if="authStore.authed">
      <ul>
        <li>
          <RouterLink to="/">Home</RouterLink>
        </li>
        <template v-if="authStore.isSuper">
          <li>
            <RouterLink to="/users">Users</RouterLink>
          </li>
          <li>
            <RouterLink to="/groups">Groups</RouterLink>
          </li>
          <li>
            <RouterLink to="/rules">Rules</RouterLink>
          </li>
        </template>
      </ul>
      <ul>
        <li>
          <RouterLink to="/account">Account</RouterLink>
        </li>
        <li>
          <RouterLink to="/logout">Log Out</RouterLink>
        </li>
      </ul>
    </template>
  </nav>
</template>

<style scoped>
ul {
  display: inline-block;
}

li {
  display: inline-block;
  margin: 0 10px;
  list-style-type: none;
}

a {
  color: white;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

ul:last-of-type {
  float: right;
}
</style>
