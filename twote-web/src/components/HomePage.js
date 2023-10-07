import React, { useEffect, useState } from 'react';
import { GetRandomProfiles } from '../proto/profile_pb.js';
import { ProfileServiceClient } from '../proto/profile_grpc_web_pb.js';
import { authOptions } from '../middleware/AuthInterceptor.js';
import { Button } from "react-bootstrap";

function HomePage() {
    const [response, setResponse] = useState([]);

    useEffect(() => {
        sendRequest();
    }, []);

    const sendRequest = () => {
        const client = new ProfileServiceClient("http://localhost:8080", null, authOptions);

        const request = new GetRandomProfiles();

        client.randomProfiles(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setResponse(['Error: ' + err.message]);
            } else {
                const profilesList = response.getProfilesList().map(profile => {
                    return profile.getFirstName() + ' ' + profile.getLastName();
                });

                setResponse(profilesList);
            }
        });
    };

    return (
        <div>
            <p>Response:</p>
            {response.map((profile, index) => (
                <div key={index}>{profile}</div>
            ))}
        </div>
    );
}

export default HomePage;
