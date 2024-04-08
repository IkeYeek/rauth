import { defineStore } from "pinia";
import { ApiService } from "@/helpers/api_service";
import type { User } from "@/stores/user_store";

export type NewGroup = {
  name: String;
};
export type Group = NewGroup & {
  id: number;
};
type GroupUpdate = {
  new_name: String;
};
type AddGroupPayload = {
  user_id: number;
};

export const useGroupStore = defineStore("group", () => {
  const create = async (to_create: NewGroup): Promise<Group> => {
    return await ApiService.makeAuthenticatedApiRequest<Group>("post", "api/groups", to_create);
  };

  const getAll = async (): Promise<Array<Group>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<Group>>("get", "api/groups");
  };

  const get = async (id: number): Promise<Group> => {
    return await ApiService.makeAuthenticatedApiRequest<Group>("get", `api/groups/${id}`);
  };

  const update = async (id: number, new_value: GroupUpdate): Promise<Group> => {
    return await ApiService.makeAuthenticatedApiRequest<Group>(
      "patch",
      `api/groups/${id}`,
      new_value,
    );
  };

  const remove = async (id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/groups/${id}`);
  };

  const addUserTo = async (id: number, payload: AddGroupPayload): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>(
      "post",
      `api/groups/${id}/users`,
      payload,
    );
  };

  const deleteUserFrom = async (id: number, user_id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>(
      "delete",
      `api/groups/${id}/users/${user_id}`,
    );
  };

  const listUsersFrom = async (id: number): Promise<Array<User>> => {
    return await ApiService.makeAuthenticatedApiRequest<Array<User>>(
      "get",
      `api/groups/${id}/users`,
    );
  };

  return {
    create,
    getAll,
    get,
    update,
    remove,
    addUserTo,
    deleteUserFrom,
    listUsersFrom,
  };
});
