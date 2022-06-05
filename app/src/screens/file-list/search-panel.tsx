/** @jsxImportSource @emotion/react */
import React from 'react';
import { DatePicker, Form, Input } from 'antd';
import 'moment/locale/zh-cn';
import locale from 'antd/es/date-picker/locale/zh_CN';

export interface User {
  id: string;
  name: string;
  token: string;
}

const { RangePicker } = DatePicker;

interface SearchPanelProps {
  param: {
    name: string;
    uploadTimeBegin: string;
    uploadTimeEnd: string;
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
              uploadTimeBegin: dateString[0],
              uploadTimeEnd: dateString[1],
            });
          }}
        />
      </Form.Item>
    </Form>
  );
};
