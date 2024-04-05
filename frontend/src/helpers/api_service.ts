import { useAuthStore } from '@/stores/auth_store'
import { BadCreditentials } from '@/errors/auth_errors'
import { ApiError, ApiUsage } from '@/errors/api_errors'
import axios from 'axios'
import type { AxiosResponse } from 'axios'
import { useEnvStore } from '@/stores/env_store'

export const ApiService = {
  makeAuthenticatedApiRequest: async <T>(
    method: 'get' | 'post' | 'patch' | 'delete',
    uri: string,
    payload?: object
  ): Promise<T> => {
    const authStore = useAuthStore()
    const envStore = useEnvStore()
    const method_binding = {
      post: axios.post,
      patch: axios.patch,
      delete: axios.delete
    }
    if (await authStore.isAuth()) {
      let bad_usage = false
      let res: AxiosResponse<T> | undefined = undefined
      try {
        switch (method) {
          case 'get':
            res = await axios.get(envStore.app_base + uri, {
              validateStatus: (s) => s < 500,
              headers: {
                Authorization: `Bearer ${authStore.getToken()}`
              }
            })
            break
          case 'post':
          case 'patch':
          case 'delete':
            res = await method_binding[method as 'post' | 'patch' | 'delete'](
              envStore.app_base + uri,
              payload,
              {
                validateStatus: (s) => s < 500,
                headers: {
                  Authorization: `Bearer ${authStore.getToken()}`
                }
              }
            )
            break
          default:
            bad_usage = true
            break
        }
      } catch (e) {
        console.error(e)
        throw new ApiError()
      }
      if (bad_usage || res === undefined) {
        throw new ApiUsage()
      }
      if (res.status === 200) {
        if (res.headers['X-Refresh-Token'] !== undefined) {
          const new_token = res.headers['X-Refresh-Token']
          authStore.setToken(new_token)
        }
        return res.data
      }
      throw new BadCreditentials() // shouldn't happen except if the user got revoked between isAuth request and this one.
    } else {
      throw new BadCreditentials()
    }
  }
}
