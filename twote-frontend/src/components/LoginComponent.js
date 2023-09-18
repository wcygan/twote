import React, { useState } from 'react';
import { LoginRequest } from '../proto/login_pb.js';
import { LoginServiceClient } from '../proto/login_grpc_web_pb.js';

function LoginComponent() {
    const [response, setResponse] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    const sendRequest = () => {
        const client = new LoginServiceClient("http://localhost:8080");

        const request = new LoginRequest();
        request.setUsername(username);
        request.setPassword(password);

        client.login(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setResponse('Error: ' + err.message);
            } else {
                console.log(response);
                setResponse(response.getReply());
            }
        });
    };

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
            <button onClick={sendRequest}>Send gRPC Request</button>
            <p>Response: {response}</p>
        </div>
    );
}

export default LoginComponent;