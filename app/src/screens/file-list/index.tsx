import styled from '@emotion/styled';
import React from 'react';
import { useEffect, useState } from 'react';
import { cleanObject, useDebounce, useMount } from 'utils';
import { useHttp } from 'utils/http';
import { List } from './list';
import { SearchPanel } from './search-panel';

export const FileListScreen = () => {
  const [param, setParam] = useState({
    name: '',
    uploadTimeBegin: '',
    uploadTimeEnd: '',
  });

  const [list, setList] = useState([]);

  const [users, setUsers] = useState([]);

  const debounceParam = useDebounce(param, 200);

  const client = useHttp();
  // 发送请求，依赖于param，param发生改变访问获得json
  useEffect(() => {
    // client('project', cleanObject(debounceParam))
    client('file_list', { data: cleanObject(debounceParam) }).then(setList);
  }, [debounceParam]);

  useMount(() => {
    client('users').then(setUsers);
  });

  return (
    <Container>
      <h1>文件列表</h1>
      <SearchPanel param={param} setParam={setParam}></SearchPanel>
      <List users={users} list={list} />
    </Container>
  );
};

const Container = styled.div`
  padding: 3.2rem;
`;
