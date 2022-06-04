import React from 'react';
import { Table } from 'antd';
import styled from '@emotion/styled';

interface Payload {
  id: string;
  appid: string;
  name: string;
  board: string;
  sign: string;
  is_delta: string;
  metadata_signature: string;
  metadata_size: string;
  sha256_hex: string;
  size: string;
  target_version: string;
  version: string;
}

interface ListProps {
  list: Payload[];
}

export const List = ({ list }: ListProps) => {
  return (
    <Table
      pagination={false}
      columns={[
        { title: 'appid', dataIndex: 'appid' },
        {
          title: '名称',
          dataIndex: 'name',
        },
        {
          title: '主板信号',
          dataIndex: 'board',
        },
        {
          title: '标识',
          dataIndex: 'sign',
        },
        {
          title: '增量更新',
          render(value, payload) {
            return <span> {payload.is_delta ? '是' : '否'}</span>;
          },
        },
        {
          title: '元数据签名',
          dataIndex: 'metadata_signature',
        },
        {
          title: '元数据大小',
          dataIndex: 'metadata_size',
        },
        {
          title: 'sha256_hex',
          dataIndex: 'sha256_hex',
        },
        {
          title: '大小',
          dataIndex: 'size',
        },
        {
          title: '目标版本',
          dataIndex: 'target_version',
        },
        {
          title: '版本',
          dataIndex: 'version',
        },
      ]}
      dataSource={list}
    />
  );
};
