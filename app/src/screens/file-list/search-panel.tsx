/** @jsxImportSource @emotion/react */
import React, { useState } from 'react';
import { Button, DatePicker, Form, Input, message, Upload } from 'antd';
import {
  UploadOutlined,
  DownloadOutlined,
  DeleteOutlined,
} from '@ant-design/icons';
import 'moment/locale/zh-cn';
import locale from 'antd/es/date-picker/locale/zh_CN';
import { getToken } from 'auth-provider';
import dayjs from 'dayjs';

const apiUrl = process.env.REACT_APP_API_URL;

const token = getToken();

const props: any = {
  action: `${apiUrl}/upload`,
  headers: { Authorization: token ? `Bearer ${token}` : '' },
  onchange(info: any) {
    if (info.response.ok) {
      message.success(info.reponse.data);
    }
  },
};

export interface User {
  id: string;
  name: string;
  token: string;
}

const { RangePicker } = DatePicker;

interface SearchPanelProps {
  param: {
    name: string;
    uploadTimeBegin: number;
    uploadTimeEnd: number;
  };
  setParam: (param: SearchPanelProps['param']) => void;
}

export const SearchPanel = ({ param, setParam }: SearchPanelProps) => {
  return (
    <Form css={{ marginBottom: '2rem' }} layout="inline">
      <Form.Item>
        <Input
          placeholder="文件名"
          type="text"
          value={param.name}
          onChange={(evt) =>
            setParam({
              ...param,
              name: evt.target.value,
            })
          }
        />
      </Form.Item>
      <Form.Item>
        <RangePicker
          locale={locale}
          onChange={(_, dateString) => {
            console.log('dataString', dateString);
            setParam({
              ...param,
              uploadTimeBegin: dayjs(dateString[0] + ' 00:00:00').unix(),
              uploadTimeEnd: dayjs(dateString[1] + ' 24:00:00').unix(),
            });
          }}
        />
      </Form.Item>
      <Form.Item>
        <Upload {...props}>
          <Button icon={<UploadOutlined />}>文件上传</Button>
        </Upload>
      </Form.Item>
      <Form.Item>
        <Button icon={<DownloadOutlined />}> 下载</Button>
      </Form.Item>
      <Form.Item>
        <Button icon={<DeleteOutlined />}> 删除</Button>
      </Form.Item>
    </Form>
  );
};
