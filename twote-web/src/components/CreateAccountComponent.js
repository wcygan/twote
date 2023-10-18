import React, { useState } from 'react';
import { CreateAccountRequest } from '../proto/backend/account_pb.js';
import { AccountServiceClient } from '../proto/backend/account_grpc_web_pb.js';
import {Button} from "react-bootstrap";
import { useNavigate } from 'react-router-dom';

function CreateAccountComponent() {
    const navigate = useNavigate();
    const [response, setResponse] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [firstName, setFirstName] = useState('');
    const [lastName, setLastName] = useState('');

    const sendRequest = () => {
        const client = new AccountServiceClient("http://localhost:8080");

        const request = new CreateAccountRequest();
        request.setUsername(username);
        request.setPassword(password);
        request.setFirstName(firstName);
        request.setLastName(lastName);

        client.createAccount(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setResponse('Error: ' + err.message);
            } else {
                navigate('/login');
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
            <div>
                <label>First Name:</label>
                <input
                    type="text"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                />
            </div>
            <div>
                <label>Last Name:</label>
                <input
                    type="text"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                />
            </div>
            <Button onClick={sendRequest}>Create Account</Button>
            <p>Response: {response}</p>
        </div>
    );
}

export default CreateAccountComponent;
