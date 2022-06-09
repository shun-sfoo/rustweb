import { Admin, EditGuesser, ListGuesser, Resource } from 'react-admin';
import dataProvider from './dataProvider';

import { FileList, FileUpload } from './files';
import { StoreList, StoreUpload } from './store';
import authProvider from './authProvider';
import { UserList, UserEdit } from './user';

import polyglotI18nProvider from 'ra-i18n-polyglot';

import chineseMessages from 'ra-language-chinese';

const i18nProvider = polyglotI18nProvider(() => chineseMessages, 'ch');

const App = () => {
  console.log(localStorage);

  const id = localStorage.getItem('id');
  if (id === '1') {
    return (
      <Admin
        i18nProvider={i18nProvider}
        authProvider={authProvider}
        dataProvider={dataProvider}
      >
        <Resource name="files" list={FileList} create={FileUpload} />
        <Resource name="stores" list={StoreList} create={StoreUpload} />
        <Resource name="users" list={UserList} edit={UserEdit} />
      </Admin>
    );
  } else {
    return (
      <Admin
        i18nProvider={i18nProvider}
        authProvider={authProvider}
        dataProvider={dataProvider}
      >
        <Resource name="files" list={FileList} />
        <Resource name="stores" list={StoreList} />
        <Resource name="users" list={UserList} edit={UserEdit} />
      </Admin>
    );
  }
};

export default App;
