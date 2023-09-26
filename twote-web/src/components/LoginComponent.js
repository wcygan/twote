import React, { useState } from 'react';
import Button from 'react-bootstrap/Button';
import { useNavigate } from 'react-router-dom';
import { LoginRequest } from '../proto/account_pb.js';
import { AccountServiceClient } from '../proto/account_grpc_web_pb.js';
import { AUTH_TOKEN} from '../middleware/AuthInterceptor.js';
import Cookies from 'js-cookie';

function LoginComponent() {
    const navigate = useNavigate();
    const [response, setResponse] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    const sendRequest = () => {
        const client = new AccountServiceClient("http://localhost:8080");

        const request = new LoginRequest();
        request.setUsername(username);
        request.setPassword(password);

        client.login(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setResponse('Error: ' + err.message);
            } else {
                console.log(response);
                Cookies.set(AUTH_TOKEN, response.getToken());
                navigate('/');
            }
        });
    };

    const navigateToCreateAccount = () => {
        navigate('/create-account');
    }

    return (
        <div>
            <div>
                <label>Username:</label>
                <input
                    type="text"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                />
            </div>
            <div>
                <label>Password:</label>
                <input
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
            </div>
            <Button onClick={sendRequest}>Login</Button>
            <Button onClick={navigateToCreateAccount}>Need an Account?</Button>
            <p>Response: {response}</p>
        </div>
    );
}

export default LoginComponent;
