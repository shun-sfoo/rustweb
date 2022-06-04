import styled from '@emotion/styled';
import React from 'react';
import { useEffect, useState } from 'react';
import { useMount } from 'utils';
import { useHttp } from 'utils/http';
import { List } from './list';

export const PayloadListScreen = () => {
  const [list, setList] = useState([]);

  const client = useHttp();

  useMount(() => {
    client('payloads').then(setList);
  });

  return (
    <Container>
      <h1>负载列表</h1>
      <List list={list} />
    </Container>
  );
};

const Container = styled.div`
  padding: 3.2rem;
`;
