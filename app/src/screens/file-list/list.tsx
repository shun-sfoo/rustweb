import React from 'react';
import { Table } from 'antd';
import styled from '@emotion/styled';
import { User } from 'screens/project-list/search-panel';
import dayjs from 'dayjs';

interface File {
  id: string;
  name: string;
  size: number;
  operator: string;
  uploadTime: number;
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
          render(_, record) {
            return (
              <span>
                {users.find((user) => user.id === record.operator)?.name ||
                  '未知'}
              </span>
            );
          },
        },
        {
          title: '文件大小',
          dataIndex: 'size',
        },
        {
          title: '上传时间',
          render(_, record) {
            return (
              <span>
                {record.uploadTime
                  ? dayjs(record.uploadTime * 1000).format(
                      'YYYY-MM-DD HH:mm:ss'
                    )
                  : '无'}
              </span>
            );
          },
        },
      ]}
      dataSource={list}
    />
  );
};
