import {
  Datagrid,
  List,
  TextField,
  SimpleForm,
  TextInput,
  Edit,
  Toolbar,
  SaveButton,
  PasswordInput,
} from 'react-admin';

export const UserList = () => (
  <List actions={false}>
    <Datagrid bulkActionButtons={false} rowClick="edit">
      <TextField source="name" label="用户名" />
    </Datagrid>
  </List>
);
const MyToolbar = (props) => (
  <Toolbar {...props}>
    <SaveButton type="submit" />
  </Toolbar>
);

export const UserEdit = () => {
  return (
    <Edit mutationMode="pessimistic">
      <SimpleForm toolbar={<MyToolbar />}>
        <PasswordInput source="old_password" label="输入原密码" />
        <PasswordInput source="new_password" label="输入新密码" />
      </SimpleForm>
    </Edit>
  );
};
