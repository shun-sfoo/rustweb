import dayjs from 'dayjs';
import {
  Button,
  Datagrid,
  DateInput,
  List,
  TextField,
  TextInput,
  TopToolbar,
  useRecordContext,
} from 'react-admin';

import IconFileUpload from '@mui/icons-material/UploadFile';

const ListActions = () => (
  <TopToolbar>
    <Button
      onClick={() => {
        alert('Your custom action');
      }}
      label="文件上传"
    >
      <IconFileUpload />
    </Button>
  </TopToolbar>
);

const postFilters = [
  <TextInput source="name" label="Search" alwaysOn />,
  <DateInput source="upload_begin" label="上传开始 " alwaysOn />,
  <DateInput source="upload_end" label="上传结束" alwaysOn />,
];

export const FileList = () => (
  <List filters={postFilters} actions={<ListActions />}>
    <Datagrid rowClick="edit">
      <TextField source="id" />
      <TextField source="name" label="文件名称" />
      <MyDateField source="uploadTime" label="上传时间" />
      <TextField source="operator" label="操作者" />
    </Datagrid>
  </List>
);

const MyDateField = ({ source }) => {
  const record = useRecordContext();
  return record
    ? dayjs(record[source] * 1000).format('YYYY-MM-DD HH:mm:ss')
    : null;
};
