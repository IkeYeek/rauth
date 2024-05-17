import { defineStore } from "pinia";
import axios from "axios";
import { useEnvStore } from "@/stores/env_store";
import { BadCreditentials } from "@/errors/auth_errors";
import { ApiError } from "@/errors/api_errors";
import { ref } from "vue";

export const useAuthStore = defineStore("auth", () => {
  const envStore = useEnvStore();
  const authed = ref(false);
  const isSuper = ref(false);


  const isAuth = async (): Promise<boolean> => {
    try {
      const { status } = await axios.get(`${envStore.app_base}auth`, {
        /*headers: {
          Authorization: `Bearer ${token}`,
        },*/
        withCredentials: true,
        validateStatus: (s) => s < 500,
      });
      authed.value = status === 200;
      await checkIsSuper();
      return status === 200;
    } catch (e) {
      console.error(e);
      if (e instanceof BadCreditentials) {
        logOut();
      }
      throw new ApiError();
    }
  };

  const checkIsSuper = async (): Promise<boolean> => {
    try {
      const { status } = await axios.get(`${envStore.app_base}auth/super`, {
        withCredentials: true,
        validateStatus: (s) => s < 500,
      });
      return status === 200;
    } catch (e) {
      console.log(e);
      return false;
    }
  };

  const logOut = (): void => {
    window.open(`${envStore.app_base}auth/logout`, "_blank");
    window.location.reload();
  };

  return {
    isAuth,
    isSuper,
    logOut,
    authed,
  };
});
