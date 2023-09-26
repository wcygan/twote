import React, {useState} from 'react';
import {HelloRequest} from '../proto/hello_pb.js';
import {HelloServiceClient} from '../proto/hello_grpc_web_pb.js';
import { authOptions } from '../middleware/AuthInterceptor.js';

function HelloComponent() {
    const [response, setResponse] = useState('');

    const sendRequest = () => {
        const client = new HelloServiceClient("http://localhost:8080", null, authOptions);

        const request = new HelloRequest();
        request.setGreeting('Hello from React!');

        client.sayHello(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setResponse('Error: ' + err.message);
            } else {
                console.log(response);
                setResponse(response.getReply());
            }
        });
    };

    return (<div>
            <button onClick={sendRequest}>Send gRPC Request</button>
            <p>Response: {response}</p>
        </div>);
}

export default HelloComponent;
