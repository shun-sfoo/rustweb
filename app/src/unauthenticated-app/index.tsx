import styled from '@emotion/styled';
import { Button, Card, Divider } from 'antd';
import React from 'react';
import { LoginScreen } from './login';
import { RegisterScreen } from './register';

export const UnauthenticatedApp = () => {
  const [isRegister, setIsRegister] = React.useState(false);

  return (
    <Container>
      <Header />
      <ShadowCard>
        <Title> {isRegister ? '请注册' : '请登录'} </Title>
        {isRegister ? <RegisterScreen /> : <LoginScreen />}
        <Divider />
        <Button type={'link'} onClick={() => setIsRegister(!isRegister)}>
          {isRegister ? '已经有账号了？直接登录' : '没有账号？注册新账号'}
        </Button>
      </ShadowCard>
    </Container>
  );
};

export const LongButton = styled(Button)`
  width: 100%;
`;

const Title = styled.h2`
  margin-bottom: 2.4rem;
  color: rgb(94, 108, 132);
`;

const Header = styled.header`
  padding: 5rem 0;
  background-size: 8rem;
  width: 100%;
`;

const ShadowCard = styled(Card)`
  width: 40rem;
  min-height: 56rem;
  padding: 3.2rem 4rem;
  border-radius: 0.3rem;
  box-sizing: border-box;
  box-shadow: rgba(0, 0, 0, 0.1) 0 0 10px;
  text-align: center;
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  min-height: 100vh;
`;