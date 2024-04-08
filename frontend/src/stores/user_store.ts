import { defineStore } from "pinia";
import { ApiService } from "@/helpers/api_service";
import { type Group, useGroupStore } from "@/stores/group_store";

type NewUser = {
  login: string;
  hash: string;
};

export type User = NewUser & {
  id: number;
  groups?: Array<Group>
};

export type UpdateUserPayload = {
  new_login: string | undefined;
  new_hash: string | undefined;
};
export const useUserStore = defineStore("user", () => {
  const groupStore = useGroupStore();
  const create = async (to_create: NewUser): Promise<User> => {
    return await ApiService.makeAuthenticatedApiRequest<User>("post", "api/users", to_create);
  };

  const getAll = async (): Promise<Array<User>> => {
    const users = await ApiService.makeAuthenticatedApiRequest<Array<User>>("get", "api/users");
    return await Promise.all(users.map(async (user) => {
      return {
        ...user,
        groups: await getUserGroups(user.id),
      };
    }));
  };

  const get = async (user_id: number): Promise<User> => {
    const user = await ApiService.makeAuthenticatedApiRequest<User>("get", `api/users/${user_id}`);
    return {
      ...user,
      groups: await getUserGroups(user.id),
    };
  };

  const update = async (user_id: number, payload: UpdateUserPayload, groups: Array<Group>): Promise<User> => {
    const userGroups = await getUserGroups(user_id);
    const newGroups = groups.filter(group => !userGroups.some(g => g.id === group.id));
    const leftGroups = userGroups.filter(group => !groups.some(g => g.id === group.id));
    await Promise.all(newGroups.map(async (newGroup) => await groupStore.addUserTo(newGroup.id, { user_id })));
    await Promise.all(leftGroups.map(async (leftGroup) => await groupStore.deleteUserFrom(leftGroup.id, user_id)));
    return await ApiService.makeAuthenticatedApiRequest<User>(
      "patch",
      `api/users/${user_id}`,
      payload,
    );
  };

  const remove = async (user_id: number): Promise<void> => {
    return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/users/${user_id}`, {});
  };

  const getUserGroups = async (user_id: number): Promise<Array<Group>> => {
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
