import React, { useEffect, useState } from 'react';
import { HomePageServiceClient } from '../proto/frontend/home_page_grpc_web_pb.js';
import google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb.js';
import { authOptions } from '../middleware/AuthInterceptor.js';
import Tweet from "../content/Tweet";
import MiniProfile from "../content/MiniProfile";
import './HomePage.css';

function HomePage() {
    const [tweets, setTweets] = useState([]);
    const [profiles, setProfiles] = useState([]);

    useEffect(() => {
        sendRequest();
    }, []);

    const sendRequest = () => {
        const client =
            new HomePageServiceClient("http://localhost:8080", null, authOptions);

        client.getHomePage(new google_protobuf_empty_pb.Empty(), {}, (err, response) => {
            if (err) {
                console.error(err);
            } else {
                console.log(response);
                setTweets(response.getTweetsList());
                setProfiles(response.getProfilesList());
            }
        });
    };

    return (
        <div className="homepage">
            <div className="tweets-column">
                {tweets.map((tweet, index) => (
                    <Tweet
                        key={index}
                        tweet={tweet}
                    />
                ))}
            </div>
            <div className="profiles-column">
                {profiles.map((profile, index) => (
                    <MiniProfile
                        key={index}
                        profile={profile}
                    />
                ))}
            </div>
        </div>
    );
}

export default HomePage;

