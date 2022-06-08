import { Admin, ListGuesser, Resource } from 'react-admin';
import dataProvider from './dataProvider';

import { FileList } from './files';
import { MyEdit } from './myedit';
import { Login } from './login';
import { FileUpload } from './files';
import authProvider from './authProvider';

import polyglotI18nProvider from 'ra-i18n-polyglot';

import chineseMessages from 'ra-language-chinese';

const i18nProvider = polyglotI18nProvider(() => chineseMessages, 'ch');

const App = () => (
  <Admin
    i18nProvider={i18nProvider}
    authProvider={authProvider}
    loginPage={Login}
    dataProvider={dataProvider}
  >
    <Resource
      name="files"
      list={FileList}
      options={{ label: '流向记录' }}
      create={FileUpload}
    />
    <Resource
      name="clones"
      list={FileList}
      options={{ label: '库存记录' }}
      create={FileUpload}
    />
  </Admin>
);
export default App;
