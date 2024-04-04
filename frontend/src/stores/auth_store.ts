import {defineStore} from "pinia";
import axios from "axios";
import {useEnvStore} from "@/stores/env_store";
import {hash as bcyrpt_hash} from "bcrypt"
import {BadCreditentials} from "@/errors/auth_errors";
import {ApiError} from "@/errors/api_errors";
type AuthResponse = {
    jwt: string
}
export const useAuthStore = defineStore('auth', () => {
    const api_base = useEnvStore().app_base;
    const getToken = (): string | null => {
        return localStorage.getItem("jwt");
    }

    const setToken = (token: string) => {
        localStorage.setItem("jwt", token);
    }

    const tryAuth = async (login: string, password: string): Promise<void> => {
        try {
            const hash = await bcyrpt_hash(password, 12);
            const {status, data} = await axios.post<AuthResponse>(`${api_base}auth`, {
                login,
                hash
            }, {
                validateStatus: (s) => s < 500,
            });
            if (status === 200) {
                setToken(data.jwt);
                return;
            }
        } catch (e) {
            console.error(e);
            throw new ApiError();
        }
        throw new BadCreditentials();
    }

    const isAuth = async (): Promise<boolean> => {
        const token = getToken();
        if (token === null) return false;
        const {status} = await axios.get(`${api_base}auth/`, {
            headers: {
                Authorization: `Bearer: ${getToken()}`
            },
            validateStatus: (s) => s < 500,
        });
        return status === 200;
    }

    return {
        tryAuth,
        isAuth,
        getToken,
    }
})