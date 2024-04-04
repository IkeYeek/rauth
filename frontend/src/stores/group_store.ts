import {defineStore} from "pinia";
import {useAuthStore} from "@/stores/auth_store";
import {useEnvStore} from "@/stores/env_store";
import {ApiService} from "@/helpers/api_service";
type Group = {
    id: number;
    name: string;
}
type NewGroup = {
    name: String;
}
type GroupUpdate = {
    new_name: String;
}
type AddGroupPayload = {
    user_id: number,
}
type User = {
    id: number;
    login: string;
    hash: string;
}
export const useGroupStore = defineStore("group", () => {
    const auth_store = useAuthStore();
    const env_store = useEnvStore();
    const createGroup = async (to_create: NewGroup): Promise<Group> => {
        return await ApiService.makeAuthenticatedApiRequest<Group>("post", "api/groups", to_create);
    }

    const getAll = async (): Promise<Array<Group>> => {
        return await ApiService.makeAuthenticatedApiRequest<Array<Group>>("get", "api/groups", undefined);
    }

    const get = async (id: number): Promise<Group> => {
        return await ApiService.makeAuthenticatedApiRequest<Group>("get", `api/groups/${id}`, undefined);
    }

    const update = async (id: number, new_value: GroupUpdate): Promise<Group> => {
        return await ApiService.makeAuthenticatedApiRequest<Group>("patch", `api/groups/${id}`, new_value);
    }

    const remove = async (id: number): Promise<void> => {
        return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/groups/${id}`, undefined);
    }

    const add_user_to = async (id: number, payload: AddGroupPayload): Promise<void> => {
        return await ApiService.makeAuthenticatedApiRequest<void>("post", `api/groups/${id}/users`, payload);
    }

    const delete_user_from = async (id: number, payload: AddGroupPayload): Promise<void> => {
        return await ApiService.makeAuthenticatedApiRequest<void>("delete", `api/groups/${id}/users`, payload);
    }

    const list_users_from = async (id: number): Promise<Array<User>> => {
        return await ApiService.makeAuthenticatedApiRequest<Array<User>>("get", `api/groups/${id}/users`, undefined);
    }
    
    return {
        createGroup,
        getAll,
        get,
        update,
        remove,
        add_user_to,
        delete_user_from,
        list_users_from,
    }
});