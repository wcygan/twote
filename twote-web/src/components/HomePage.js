import React, { useEffect, useState } from 'react';
import { GetRandomProfiles } from '../proto/profile_pb.js';
import { ProfileServiceClient } from '../proto/profile_grpc_web_pb.js';
import { authOptions } from '../middleware/AuthInterceptor.js';
import MiniProfile from '../structure/MiniProfile.js';

function HomePage() {
    const [profiles, setProfiles] = useState([]);

    useEffect(() => {
        sendRequest();
    }, []);

    const sendRequest = () => {
        const client = new ProfileServiceClient("http://localhost:8080", null, authOptions);

        const request = new GetRandomProfiles();

        client.randomProfiles(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                setProfiles([{ firstName: 'Error', lastName: err.message }]);
            } else {
                const profilesList = response.getProfilesList().map(profile => ({
                    id: profile.getUserId(), // Extract user ID
                    firstName: profile.getFirstName(),
                    lastName: profile.getLastName(),
                }));

                setProfiles(profilesList);
            }
        });
    };

    return (
        <div>
            <p>Profiles:</p>
            {profiles.map((profile, index) => (
                <MiniProfile
                    key={index}
                    id={profile.id}
                    firstName={profile.firstName}
                    lastName={profile.lastName}
                />
            ))}
        </div>
    );
}

export default HomePage;
