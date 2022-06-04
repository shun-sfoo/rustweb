import React from 'react';
import { useEffect, useState } from 'react';
import * as qs from 'qs';
import { cleanObject, useDebounce, useMount } from 'utils';
import { SearchPanel } from './search-panel';
import { List } from './list';
import { useHttp } from 'utils/http';
import styled from '@emotion/styled';

export const ProjectListScreen = () => {
  const [param, setParam] = useState({
    name: '',
    personId: '',
  });

  const [list, setList] = useState([]);

  const [users, setUsers] = useState([]);

  const debounceParam = useDebounce(param, 200);

  const client = useHttp();
  // 发送请求，依赖于param，param发生改变访问获得json
  useEffect(() => {
    // client('project', cleanObject(debounceParam))
    client('projects', { data: cleanObject(debounceParam) }).then(setList);
  }, [debounceParam]);

  useMount(() => {
    client('users').then(setUsers);
  });

  return (
    <Container>
      <SearchPanel
        users={users}
        param={param}
        setParam={setParam}
      ></SearchPanel>
      <List users={users} list={list} />
    </Container>
  );
};

const Container = styled.div`
  padding: 3.2rem;
`;
