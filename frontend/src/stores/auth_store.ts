import { defineStore } from 'pinia'
import axios from 'axios'
import { useEnvStore } from '@/stores/env_store'
import { BadCreditentials } from '@/errors/auth_errors'
import { ApiError } from '@/errors/api_errors'
type AuthResponse = {
  jwt: string
}
export const useAuthStore = defineStore('auth', () => {
  const api_base = useEnvStore().app_base
  const getToken = (): string | null => {
    return localStorage.getItem('jwt')
  }

  const setToken = (token: string) => {
    localStorage.setItem('jwt', token)
  }

  const tryAuth = async (login: string, password: string): Promise<void> => {
    try {
      const { status, data } = await axios.post<AuthResponse>(
        `${api_base}auth`,
        {
          login,
          password
        },
        {
          validateStatus: (s) => s < 500
        }
      )
      if (status === 200) {
        setToken(data.jwt)
        return
      }
    } catch (e) {
      console.error(e)
      throw new ApiError()
    }
    throw new BadCreditentials()
  }

  const isAuth = async (): Promise<boolean> => {
    const token = getToken()
    if (token === null) return false
    try {
      const { status } = await axios.get(`${api_base}auth/`, {
        headers: {
          Authorization: `Bearer ${getToken()}`
        },
        validateStatus: (s) => s < 500
      })
      return status === 200
    } catch (e) {
      console.error(e)
      throw new ApiError()
    }
  }

  return {
    tryAuth,
    isAuth,
    getToken,
    setToken
  }
})
