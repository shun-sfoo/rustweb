import { Admin, EditGuesser, ListGuesser, Resource } from 'react-admin';
import dataProvider from './dataProvider';

import { FileList, FileUpload } from './files';
import { StoreList, StoreUpload } from './store';
import authProvider from './authProvider';
import { UserList, UserEdit } from './user';

import polyglotI18nProvider from 'ra-i18n-polyglot';

import chineseMessages from 'ra-language-chinese';
import FlowIcon from '@mui/icons-material/Book';
import UserIcon from '@mui/icons-material/Group';

const i18nProvider = polyglotI18nProvider(() => chineseMessages, 'ch');

const App = () => {
  return (
    <Admin
      i18nProvider={i18nProvider}
      authProvider={authProvider}
      dataProvider={dataProvider}
    >
      <Resource
        name="files"
        options={{ label: '流向记录' }}
        icon={FlowIcon}
        list={FileList}
        create={FileUpload}
      />
      <Resource
        name="stores"
        options={{ label: '库存记录' }}
        list={StoreList}
        create={StoreUpload}
      />
      <Resource
        name="users"
        options={{ label: '用户信息' }}
        icon={UserIcon}
        list={UserList}
        edit={UserEdit}
      />
    </Admin>
  );
};

export default App;
