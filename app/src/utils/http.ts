import qs from 'qs';
import * as auth from 'auth-provider';
import { useAuth } from 'context/auth-context';
const apiUrl = process.env.REACT_APP_API_URL;

interface Config extends RequestInit {
  token?: string;
  data?: Object;
}

export const http = async (
  endpoint: string,
  { data, token, method, headers, ...customConfig }: Config = {}
) => {
  const config = {
    method: 'GET',
    headers: {
      Authorization: token ? `Bearer ${token}` : '',
      'Content-Type': data ? 'application/json' : '',
    },
    // 如果传入post请求会覆盖默认的get
    ...customConfig,
  };

  if (config.method.toUpperCase() === 'GET') {
    endpoint += `?${qs.stringify(data)}`;
  } else {
    config.body = JSON.stringify(data || {});
  }

  return window
    .fetch(`${apiUrl}/${endpoint}`, config)
    .then(async (response) => {
      // unauthorize to logout
      if (response.status === 401) {
        await auth.logout();
        window.location.reload();
        return Promise.reject({ message: `请求${endpoint}失败,请重新登录` });
      }
      const data = await response.json();
      if (response.ok) {
        return data;
      } else {
        return Promise.reject(data);
      }
    });
};

export const useHttp = () => {
  const { user } = useAuth();

  // ... rest操作符号， 是传入的参数可以为 (endpoint, config)  而不是元组形式
  // utility type 的用法： 用范型给它传入一个其他类型，然后utility type对这个类型进行某种操作
  // Partial 允许类型的成员为option Omit 删除类型的某些属性 Omit<Person, 'name' | 'age'>
  return (...[endpoint, config]: Parameters<typeof http>) =>
    http(endpoint, { ...config, token: user?.token });
};

// 联合类型
// sting | number

// type Partial<T> = {
//   [P in keyof T]?: T[P];
// };
