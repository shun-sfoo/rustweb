import { fetchUtils } from 'react-admin';
import { stringify } from 'query-string';

const apiUrl = process.env.REACT_APP_API_URL;
const httpClient = fetchUtils.fetchJson;

export default {
  // called when the user attempts to log in
  login: ({ username, password }) => {
    const query = {
      username: username,
      password: password,
    };

    const url = `${apiUrl}/login?${stringify(query)}`;

    return httpClient(url).then(({ json }) => {
      let user = json.user;
      localStorage.setItem('id', user.id);
      localStorage.setItem('name', user.name);
      localStorage.setItem('token', user.token);

      return Promise.resolve();
    });
  },
  // called when the user clicks on the logout button
  logout: () => {
    localStorage.removeItem('name');
    return Promise.resolve();
  },
  // called when the API returns an error
  checkError: ({ status }) => {
    if (status === 401 || status === 403) {
      localStorage.removeItem('name');
      return Promise.reject();
    }
    return Promise.resolve();
  },
  // called when the user navigates to a new location, to check for authentication
  checkAuth: () => {
    return localStorage.getItem('name') ? Promise.resolve() : Promise.reject();
  },
  // called when the user navigates to a new location, to check for permissions / roles
  getPermissions: () => {
    // Required for the authentication to work
    return Promise.resolve();
  },
};
