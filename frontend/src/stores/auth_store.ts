import { defineStore } from "pinia";
import axios from "axios";
import { useEnvStore } from "@/stores/env_store";
import { BadCreditentials } from "@/errors/auth_errors";
import { ApiError } from "@/errors/api_errors";
import { ref } from "vue";

type AuthResponse = {
  jwt: string;
};
export const useAuthStore = defineStore("auth", () => {
  const envStore = useEnvStore();
  const api_base = useEnvStore().app_base;
  const authed = ref(false);
  const getToken = (): string | null => {
    return localStorage.getItem("jwt");
  };

  const setToken = (token: string) => {
    localStorage.setItem("jwt", token);
  };

  const tryAuth = async (login: string, password: string): Promise<void> => {
    try {
      const { status, data } = await axios.post<AuthResponse>(
        `${api_base}auth`,
        {
          login,
          password,
        },
        {
          validateStatus: (s) => s < 500,
        },
      );
      if (status === 200) {
        setToken(data.jwt);
        authed.value = true;
        return;
      }
    } catch (e) {
      console.error(e);
      throw new ApiError();
    }
    throw new BadCreditentials();
  };

  const isAuth = async (): Promise<boolean> => {
    const token = getToken();
    if (token === null) {
      return false;
    }
    try {
      const { status } = await axios.get(`${envStore.app_base}auth`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
        validateStatus: (s) => s < 500,
      });
      authed.value = status === 200;
      return status === 200;
    } catch (e) {
      console.error(e);
      if (e instanceof BadCreditentials) {
        logOut();
      }
      throw new ApiError();
    }
  };

  const logOut = (): void => {
    localStorage.removeItem("jwt");
    authed.value = false;
  };

  return {
    tryAuth,
    isAuth,
    getToken,
    setToken,
    logOut,
    authed,
  };
});
