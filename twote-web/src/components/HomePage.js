import React, { useEffect, useState } from 'react';
import { FindMostRecentProfilesRequest } from '../proto/backend/profile_pb.js';
import { ProfileServiceClient } from '../proto/backend/profile_grpc_web_pb.js';
import { authOptions } from '../middleware/AuthInterceptor.js';
import MiniProfile from '../content/MiniProfile.js';

function HomePage() {
    const [profiles, setProfiles] = useState([]);

    useEffect(() => {
        sendRequest();
    }, []);

    const sendRequest = () => {
        const client = new ProfileServiceClient("http://localhost:8080", null, authOptions);

        const request = new FindMostRecentProfilesRequest();

        client.findMostRecentProfiles(request, {}, (err, response) => {
            if (err) {
                console.error(err);
            } else {
                setProfiles(response.getProfilesList());
            }
        });
    };

    return (
        <div>
            <p>Profiles:</p>
            {profiles.map((profile, index) => (
                <MiniProfile
                    key={index}
                    profile={profile}
                />
            ))}
        </div>
    );
}

export default HomePage;
