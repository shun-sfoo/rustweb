import * as React from 'react';
import { useState } from 'react';
import { useMediaQuery } from '@mui/material';
import PropTypes from 'prop-types';
import { useLocation } from 'react-router-dom';
import b2 from './b2.jpg';
import { ReactComponent as Logo } from './neoLogo.svg';

import {
  Avatar,
  Button,
  Card,
  CardActions,
  CircularProgress,
} from '@mui/material';
import LockIcon from '@mui/icons-material/Lock';
import {
  Form,
  required,
  TextInput,
  useTranslate,
  useLogin,
  useNotify,
} from 'react-admin';

import Box from '@mui/material/Box';

const Login = () => {
  const [loading, setLoading] = useState(false);
  const translate = useTranslate();

  const notify = useNotify();
  const login = useLogin();
  const location = useLocation();

  const handleSubmit = (auth) => {
    setLoading(true);
    login(auth, location.state ? location.state.nextPathname : '/').catch(
      (error: Error) => {
        setLoading(false);
        notify(
          typeof error === 'string'
            ? error
            : typeof error === 'undefined' || !error.message
            ? 'ra.auth.sign_in_error'
            : error.message,
          {
            type: 'warning',
            messageArgs: {
              _:
                typeof error === 'string'
                  ? error
                  : error && error.message
                  ? error.message
                  : undefined,
            },
          }
        );
      }
    );
  };

  const isSmall = useMediaQuery((theme) => theme.breakpoints.down('sm'));

  return (
    <Form onSubmit={handleSubmit} noValidate>
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          minHeight: '100vh',
          alignItems: 'center',
          justifyContent: 'flex-start',
          background: `url(${b2})`,
          backgroundRepeat: 'no-repeat',
          backgroundSize: 'cover',
        }}
      >
        {isSmall ? (
          <Card sx={{ minWidth: 300, margin: '1rem' }}>
            <Box
              sx={{
                margin: '1em',
                display: 'flex',
                justifyContent: 'center',
              }}
            >
              <Avatar sx={{ bgcolor: 'secondary.main' }}>
                <LockIcon />
              </Avatar>
            </Box>
            <Box
              sx={{
                marginTop: '1em',
                display: 'flex',
                justifyContent: 'center',
                color: (theme) => theme.palette.grey[500],
              }}
            >
              请登录青海灵峰药业流向系统
            </Box>
            <Box sx={{ padding: '0 1em 1em 1em' }}>
              <Box sx={{ marginTop: '1em' }}>
                <TextInput
                  autoFocus
                  source="username"
                  label={translate('ra.auth.username')}
                  disabled={loading}
                  validate={required()}
                  fullWidth
                />
              </Box>
              <Box sx={{ marginTop: '1em' }}>
                <TextInput
                  source="password"
                  label={translate('ra.auth.password')}
                  type="password"
                  disabled={loading}
                  validate={required()}
                  fullWidth
                />
              </Box>
            </Box>
            <CardActions sx={{ padding: '0 1em 1em 1em' }}>
              <Button
                variant="contained"
                type="submit"
                color="primary"
                disabled={loading}
                fullWidth
              >
                {loading && <CircularProgress size={25} thickness={2} />}
                {translate('ra.auth.sign_in')}
              </Button>
            </CardActions>
          </Card>
        ) : (
          <Card sx={{ minWidth: 300, marginTop: '10%', marginLeft: '50%' }}>
            <Box
              sx={{
                margin: '1em',
                display: 'flex',
                justifyContent: 'center',
              }}
            >
              <Avatar sx={{ bgcolor: 'secondary.main' }}>
                <LockIcon />
              </Avatar>
            </Box>
            <Box
              sx={{
                marginTop: '1em',
                display: 'flex',
                justifyContent: 'center',
                color: (theme) => theme.palette.grey[500],
              }}
            >
              请登录青海灵峰药业流向系统
            </Box>
            <Box sx={{ padding: '0 1em 1em 1em' }}>
              <Box sx={{ marginTop: '1em' }}>
                <TextInput
                  autoFocus
                  source="username"
                  label={translate('ra.auth.username')}
                  disabled={loading}
                  validate={required()}
                  fullWidth
                />
              </Box>
              <Box sx={{ marginTop: '1em' }}>
                <TextInput
                  source="password"
                  label={translate('ra.auth.password')}
                  type="password"
                  disabled={loading}
                  validate={required()}
                  fullWidth
                />
              </Box>
            </Box>
            <CardActions sx={{ padding: '0 1em 1em 1em' }}>
              <Button
                variant="contained"
                type="submit"
                color="primary"
                disabled={loading}
                fullWidth
              >
                {loading && <CircularProgress size={25} thickness={2} />}
                {translate('ra.auth.sign_in')}
              </Button>
            </CardActions>
          </Card>
        )}
      </Box>
    </Form>
  );
};

Login.propTypes = {
  authProvider: PropTypes.func,
  previousRoute: PropTypes.string,
};

interface FormValues {
  username?: string;
  password?: string;
}

export default Login;
