import dayjs from 'dayjs';

import {
  Button,
  Datagrid,
  DateInput,
  List,
  TextField,
  TextInput,
  TopToolbar,
  FileInput,
  FileField,
  ImageField,
  Create,
  SimpleForm,
  useRecordContext,
  CreateButton,
  BulkDeleteWithConfirmButton,
} from 'react-admin';

import IconFileUpload from '@mui/icons-material/UploadFile';
import LaunchIcon from '@mui/icons-material/Launch';
import { Link } from '@mui/material';
import { Fragment } from 'react';

const ListActions = () => (
  <TopToolbar>
    <CreateButton label="上传" icon={<IconFileUpload />} />
  </TopToolbar>
);

const postFilters = [
  <TextInput source="name" label="Search" alwaysOn />,
  <DateInput source="upload_begin" label="上传开始 " alwaysOn />,
  <DateInput source="upload_end" label="上传结束" alwaysOn />,
];

const FileBulkActionButtons = () => (
  <Fragment>
    <BulkDeleteWithConfirmButton
      confirmTitle="确认删除条目"
      confirmContent="是否确认删除？"
    />
  </Fragment>
);

const downloadFile = (record) => {
  console.log(record);
};

export const StoreList = () => (
  <List filters={postFilters} actions={<ListActions />}>
    <Datagrid bulkActionButtons={<FileBulkActionButtons />}>
      <FileField source="location" title="name" label="文件名称" />
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

const MyUrlField = ({ source }) => {
  const record = useRecordContext();
  return record ? (
    <Link
      href={'http://localhost:8080/' + record[source]}
      sx={{ textDecoration: 'none' }}
    >
      {record[source]}
      <LaunchIcon sx={{ width: '0.5em', height: '0.5em', paddingLeft: 2 }} />
    </Link>
  ) : null;
};

export const StoreUpload = () => (
  <Create>
    <SimpleForm>
      <FileInput
        source="files"
        label="选择文件"
        placeholder={<p>点击或者拖拽上传</p>}
      >
        <ImageField source="src" title="title" />
      </FileInput>
    </SimpleForm>
  </Create>
);
