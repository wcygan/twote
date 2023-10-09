import {ProfilePageServiceClient} from '../proto/frontend/profile_page_grpc_web_pb.js';
import {GetProfilePageRequest} from '../proto/frontend/profile_page_pb.js';
import {authOptions} from "../middleware/AuthInterceptor";
import React, {useEffect, useState} from "react";
import {useParams} from 'react-router-dom';
import Profile from "../content/Profile";
import Tweet from "../content/Tweet";

const ProfilePage = () => {
    const { id } = useParams();

    const [profile, setProfile] = useState(null);
    const [tweets, setTweets] = useState([]);

    useEffect(() => {
        const client =
            new ProfilePageServiceClient("http://localhost:8080", null, authOptions);

        const request = new GetProfilePageRequest();
        request.setUserId(id);

        client.getProfilePage(request, {}, (err, response) => {
            if (err) {
                console.error(err);
            } else {
                console.log(response);
                setProfile(response.getProfile());
                setTweets(response.getTweetsList());
            }
        });
    }, []);

    return <div>
        <Profile
            profile={profile}
        />
        {tweets.map((tweet, index) => (
            <Tweet
                key={index}
                tweet={tweet}
            />
        ))}
    </div>;
};

export default ProfilePage;