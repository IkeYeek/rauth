import { useAuthStore } from "@/stores/auth_store";
import { BadCreditentials, NotFound } from "@/errors/auth_errors";
import { ApiError, ApiUsage, UnprocessableEntity } from "@/errors/api_errors";
import axios from "axios";
import type { AxiosResponse } from "axios";
import { useEnvStore } from "@/stores/env_store";

export const ApiService = {
  /**
   * Generate an authenticated API request based on the method, URI, and payload.
   *
   * @param {"get" | "post" | "patch" | "delete"} method - The HTTP method for the request.
   * @param {string} uri - The URI to make the request to.
   * @param {object} payload - The payload to send with the request.
   * @return {Promise<T>} The response data from the API request.
   */
  makeAuthenticatedApiRequest: async <T>(
    method: "get" | "post" | "patch" | "delete",
    uri: string,
    payload?: object,
  ): Promise<T> => {
    const authStore = useAuthStore();
    const envStore = useEnvStore();
    const method_binding = {
      post: axios.post,
      patch: axios.patch,
      delete: axios.delete,
    };
    if (await authStore.authed) {
      let bad_usage = false;
      let res: AxiosResponse<T> | undefined = undefined;
      try {
        switch (method) {
          case "get":
            res = await axios.get(envStore.app_base + uri, {
              withCredentials: true,
              validateStatus: (s) => s < 500,
              /*headers: {
                Authorization: `Bearer ${authStore.getToken()}`,
              },*/
            });
            break;
          case "delete":
            res = await axios.delete(envStore.app_base + uri, {
              withCredentials: true,
              validateStatus: (s) => s < 500,
              /*headers: {
                Authorization: `Bearer ${authStore.getToken()}`,
              },*/
            });
            break;
          case "post":
          case "patch":
            res = await method_binding[method as "post" | "patch"](
              envStore.app_base + uri,
              payload,
              {
                withCredentials: true,
                validateStatus: (s) => s < 500,
                /*headers: {
                  Authorization: `Bearer ${authStore.getToken()}`,
                },*/
              },
            );
            break;
          default:
            bad_usage = true;
            break;
        }
      } catch (e) {
        console.error(e);
        throw new ApiError();
      }
      if (bad_usage || res === undefined) {
        throw new ApiUsage();
      }
      if (res.status === 200) {
        /*if (res.headers["x-refresh-token"] !== undefined) {
          const new_token = res.headers["x-refresh-token"] as string;
          authStore.setToken(new_token);
        }*/
        return res.data;
      }
      switch (res.status) {
        case 403:
          throw new BadCreditentials();
        case 404:
          throw new NotFound(res.data as string);
        case 422:
          throw new UnprocessableEntity(res.data as string);
        default:
          throw new ApiError();
      }
    } else {
      throw new BadCreditentials();
    }
  },
};
