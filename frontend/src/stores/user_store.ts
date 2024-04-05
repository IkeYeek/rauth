import { defineStore } from "pinia";
import { ApiService } from "@/helpers/api_service";
import type { Group } from "@/stores/group_store";
type NewUser = {
  login: string;
  hash: string;
};

export type User = NewUser & {
  id: number;
};

type UpdateUserPayload = {
  new_login: string | undefined;
  new_hash: string | undefined;
};
export const useUserStore = defineStore("user", () => {
  const create = async (to_create: NewUser): Promise<User> => {
    return await ApiService.makeAuthenticatedApiRequest<User>("post", "api/users", to_create);
  };

  const getAll = async (): Promise<Array<User>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<User>>("get", "api/users");
  };

  const get = async (user_id: number): Promise<User> => {
    return await ApiService.makeAuthenticatedApiRequest<User>("get", `api/users/${user_id}`);
  };

  const update = async (user_id: number, payload: UpdateUserPayload): Promise<User> => {
    return await ApiService.makeAuthenticatedApiRequest<User>(
      "patch",
      `api/users/${user_id}`,
      payload,
    );
  };

  const remove = async (user_id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/users/${user_id}`);
  };

  const getUserGroups = async (user_id: number): Promise<Group> => {
    return await ApiService.makeAuthenticatedApiRequest("get", `api/users/${user_id}/groups`);
  };

  return {
    create,
    getAll,
    get,
    update,
    remove,
    getUserGroups,
  };
});
