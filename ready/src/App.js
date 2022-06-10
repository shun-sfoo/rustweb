import { Admin, EditGuesser, ListGuesser, Resource, Login } from 'react-admin';
import dataProvider from './dataProvider';

import { FileList, FileUpload } from './files';
import { StoreList, StoreUpload } from './store';
import authProvider from './authProvider';
import { UserList, UserEdit } from './user';
import { Login as cLogin } from './login';

import polyglotI18nProvider from 'ra-i18n-polyglot';
import back from './background.jpg';

import chineseMessages from 'ra-language-chinese';
import FlowIcon from '@mui/icons-material/Book';
import UserIcon from '@mui/icons-material/Group';

import SignInSide from './slide';

const name1 = '流向记录';
const name2 = '库存记录';

const i18nProvider = polyglotI18nProvider(() => chineseMessages, 'ch');

const MyLoginPage = () => <Login backgroundImage={back} />;

const App = () => {
  return (
    <Admin
      loginPage={SignInSide}
      i18nProvider={i18nProvider}
      authProvider={authProvider}
      dataProvider={dataProvider}
    >
      <Resource
        name="files"
        options={{ label: 'files' }}
        icon={FlowIcon}
        list={FileList}
        create={FileUpload}
      />
      <Resource
        name="stores"
        options={{ label: 'stores' }}
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
