import { Admin, ListGuesser, Resource } from 'react-admin';
import dataProvider from './dataProvider';

import { FileList } from './files';
import { MyEdit } from './myedit';

import polyglotI18nProvider from 'ra-i18n-polyglot';

import chineseMessages from 'ra-language-chinese';

const i18nProvider = polyglotI18nProvider(() => chineseMessages, 'ch');

const App = () => (
  <Admin i18nProvider={i18nProvider} dataProvider={dataProvider}>
    <Resource name="files" list={FileList} MyEdit={FileList} />
  </Admin>
);
export default App;
