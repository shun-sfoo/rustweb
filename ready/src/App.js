import { Admin, ListGuesser, Resource } from 'react-admin';
import './App.css';
import dataProvider from './dataProvider';

const App = () => (
  <Admin dataProvider={dataProvider}>
    <Resource name="files" list={ListGuesser} />
  </Admin>
);
export default App;
