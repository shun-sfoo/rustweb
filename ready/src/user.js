import {
  Datagrid,
  List,
  TextField,
  SimpleForm,
  TextInput,
  Edit,
} from 'react-admin';

export const UserList = () => (
  <List>
    <Datagrid rowClick="edit">
      <TextField source="name" />
    </Datagrid>
  </List>
);

export const UserEdit = () => (
  <Edit>
    <SimpleForm>
      <TextInput source="old_password" label="输入原密码" />
      <TextInput source="new_password" label="输入新密码" />
    </SimpleForm>
  </Edit>
);
