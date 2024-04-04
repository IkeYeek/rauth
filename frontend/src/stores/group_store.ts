import {defineStore} from "pinia";
import {useAuthStore} from "@/stores/auth_store";
import {NotAuthenticated, NotAuthorized} from "@/errors/auth_errors";
import {useEnvStore} from "@/stores/env_store";
import axios from "axios";
import {ApiError} from "@/errors/api_errors";
type Group = {
    id: number;
    name: string;
}
type NewGroup = {
    name: String;
}
export const useGroupStore = defineStore("group", () => {
    const auth_store = useAuthStore();
    const env_store = useEnvStore();
    const createGroup = async (to_create: NewGroup): Promise<Group> => {
        if (await auth_store.isAuth()) {
            try {
                const {status, data} = axios.post<Group>(`${env_store.app_base}api/groups/`, to_create, {
                    validateStatus: (s) => s < 500,
                    headers: {
                        Authorization: `Bearer: ${auth_store.getToken()}`
                    }
                })
                if (status === 200) {
                    return data;
                }
            } catch (e) {
                console.error(e);
                throw new ApiError();
            }
            throw new NotAuthorized();
        } else {
            throw new NotAuthenticated();
        }
    }

    const getAll = async (): Promise<Array<Group>> => {
        if (await auth_store.isAuth()) {
            try {
                const {status, data} = await axios.get<Array<Group>>(`${env_store.app_base}api/groups`, {
                    validateStatus: (s) => s < 500,
                    headers: {
                        Authorization: `Bearer: ${auth_store.getToken()}`
                    }
                });
                if (status === 200) {
                    return data;
                }
            }
            catch (e) {
                console.error(e);
                throw new ApiError();
            }
            throw new NotAuthorized();
        } else {
            throw new NotAuthenticated();
        }
    }

    const get = async (id: number): Promise<Group> => {
        if (await auth_store.isAuth()) {
            try {
                const {status, data} = await axios.get<Group>(`${env_store.app_base}api/groups/${id}`, {
                    validateStatus: (s) => s < 500,
                    headers: {
                        Authorization: `Bearer: ${auth_store.getToken()}`
                    }
                });
                if (status === 200) {
                    return data;
                }
            } catch (e) {
                console.error(e);
                throw new ApiError();
            }
            throw new NotAuthorized();
        } else {
            throw new NotAuthenticated();
        }
    }
});