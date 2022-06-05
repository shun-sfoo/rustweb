import React from 'react';
import { Table } from 'antd';
import styled from '@emotion/styled';
import { User } from 'screens/project-list/search-panel';

interface File {
  id: string;
  name: string;
  size: string;
  operator: string;
  uploadTime: string;
}

interface ListProps {
  list: File[];
  users: User[];
}

export const List = ({ users, list }: ListProps) => {
  return (
    <Table
      pagination={false}
      columns={[
        { title: 'id', dataIndex: 'id' },
        {
          title: '名称',
          dataIndex: 'name',
        },
        {
          title: '上传者',
          render(value, file) {
            return (
              <span>
                {users.find((user) => user.id === file.operator)?.name ||
                  '未知'}
              </span>
            );
          },
        },
        {
          title: '文件大小',
          dataIndex: 'size',
        },
      ]}
      dataSource={list}
    />
  );
};
