import {defineStore} from "pinia";
import dotenv from 'dotenv';
import {ref} from "vue";
dotenv.config();
export const useEnvStore = defineStore('env', () => {
    const app_base = ref<string>(process.env.BASE_URL || "http://localhost:8080/");

    return {
        app_base
    }
})