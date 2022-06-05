import React from 'react';
import styled from '@emotion/styled';
import { Row } from 'components/lib';
import { useAuth } from 'context/auth-context';
import { ReactComponent as SoftwareLogo } from 'assets/tail.svg';
import { Button, Dropdown, Menu, Upload, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { UploadFile } from 'antd/lib/upload/interface';
import { FileListScreen } from 'screens/file-list';

export const AuthenticatedApp = () => {
  return (
    <Container>
      <PageHeader />
      <Main>
        <FileListScreen />
      </Main>
    </Container>
  );
};

const apiUrl = process.env.REACT_APP_API_URL;

const props: any = {
  maxCount: 1,
  action: `${apiUrl}/neo_upload_json`,
  beforeUpload: (file: UploadFile) => {
    const isJson = file.type === 'application/json';
    if (!isJson) {
      message.error(`${file.name} is not a json file`);
    }
    return isJson || Upload.LIST_IGNORE;
  },

  onchange(info: any) {
    if (info.response.ok) {
      message.success(info.reponse.data);
    }
  },
};

const PageHeader = () => {
  const { logout, user } = useAuth();
  return (
    <Header between={true}>
      <SoftwareLogo width={'18rem'} color={'rgb(38, 132, 255)'} />
      <HeaderLeft gap={true}>
        <Upload {...props}>
          <Button icon={<UploadOutlined />}>Click to Upload</Button>
        </Upload>
      </HeaderLeft>
      <HeaderRight>
        <Dropdown
          overlay={
            <Menu>
              <Menu.Item key={'logout'}>
                <Button type={'link'} onClick={logout}>
                  登出
                </Button>
              </Menu.Item>
            </Menu>
          }
        >
          <Button type={'link'} onClick={(e) => e.preventDefault()}>
            你好，{user?.name}
          </Button>
        </Dropdown>
      </HeaderRight>
    </Header>
  );
};

const Container = styled.div`
  display: grid;
  grid-template-rows: 6rem 1fr;
  height: 100vh;
`;

const Header = styled(Row)`
  padding: 3.2rem;
  box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.1);
  z-index: 1;
`;

const HeaderLeft = styled(Row)``;

const HeaderRight = styled.div``;

const Main = styled.main``;
