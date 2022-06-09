import { fetchUtils } from 'react-admin';
import { stringify } from 'query-string';

const apiUrl = process.env.REACT_APP_API_URL;
const httpClient = fetchUtils.fetchJson;

export default {
  getList: (resource, params) => {
    const { page, perPage } = params.pagination;
    const { field, order } = params.sort;
    const query = {
      sort: JSON.stringify([field, order]),
      range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
      filter: JSON.stringify(params.filter),
    };
    const url = `${apiUrl}/${resource}?${stringify(query)}`;

    let options = {};

    options.user = {
      authenticated: true,
      // use the token from local storage
      token: localStorage.getItem('token'),
    };

    return httpClient(url, options).then(({ headers, json }) => ({
      data: json,
      total: parseInt(headers.get('Content-Range').split('/').pop(), 10),
    }));
  },

  delete: (resource, params) =>
    httpClient(`${apiUrl}/${resource}/${params.id}`, {
      method: 'DELETE',
    }).then(({ json }) => ({ data: json })),

  deleteMany: (resource, params) => {
    const query = {
      filter: JSON.stringify({ id: params.ids }),
    };
    return httpClient(`${apiUrl}/${resource}?${stringify(query)}`, {
      method: 'DELETE',
    }).then(({ json }) => ({ data: json }));
  },

  getMany: (resource, params) => {
    const query = {
      filter: JSON.stringify({ id: params.ids }),
    };
    const url = `${apiUrl}/${resource}?${stringify(query)}`;
    return httpClient(url).then(({ json }) => ({ data: json }));
  },

  create: (resource, params) => {
    let formData = new FormData();
    formData.append('file', params.data.files.rawFile);

    return httpClient(`${apiUrl}/${resource}`, {
      method: 'POST',
      body: formData,
    }).then(({ json }) => ({
      data: { ...params.data, id: json.id },
    }));
  },

  update: (resource, params) => {
    let data = {
      old_password: params.data.old_password.toString(),
      new_password: params.data.new_password.toString(),
    };
    return httpClient(`${apiUrl}/${resource}/${params.id}`, {
      method: 'PUT',
      body: JSON.stringify(params.data),
    })
      .then(({ json }) => {
        {
          return { data: json };
        }
      })
      .catch((error) => {
        return new Promise(function (resolve, reject) {
          reject({ message: '原密码错误，请确认输入的密码' });
        });
      });
  },

  getOne: (resource, params) =>
    httpClient(`${apiUrl}/${resource}/${params.id}`).then(({ json }) => ({
      data: json,
    })),

  getPermissions: () => {
    let options = {};
    options.user = {
      authenticated: true,
      // use the token from local storage
      token: localStorage.getItem('token'),
    };

    const url = `${apiUrl}/permissions`;
    return httpClient(url, options).then(({ json }) => {
      return {
        data: json,
      };
    });
  },
};
