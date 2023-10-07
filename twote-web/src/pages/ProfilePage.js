import { ProfileServiceClient } from '../proto/profile_grpc_web_pb.js';
import { GetProfileRequest } from '../proto/profile_pb.js';
import {authOptions, MY_USER_ID} from "../middleware/AuthInterceptor";
import React, {useEffect, useState} from "react";
import Cookies from 'js-cookie';

const ProfilePage = () => {
    const [firstName, setFirstName] = useState('');
    const [lastName, setLastName] = useState('');
    const [joinedAt, setJoinedAt] = useState('');
    const [bio, setBio] = useState('');

    useEffect(() => {
        const id = Cookies.get(MY_USER_ID);
        const client =
            new ProfileServiceClient("http://localhost:8080", null, authOptions);

        const request = new GetProfileRequest();
        request.setUserId(id);

        client.get(request, {}, (err, response) => {
            if (err) {
                console.error(err);
                return;
            }

            const date = new Date(response.getJoinedAt().getSeconds() * 1000);
            setFirstName(response.getFirstName());
            setLastName(response.getLastName());
            setBio(response.getBiography());
            setJoinedAt(date.toDateString());
        });
    }, []);

    return <div>
        Your content here
        <p>{firstName} {lastName}</p>
        <p>Joined at: {joinedAt}</p>
        <p>Bio: {bio}</p>
    </div>;
};

export default ProfilePage;