import {ProfileServiceClient} from '../proto/backend/profile_grpc_web_pb.js';
import {GetProfileRequest} from '../proto/backend/profile_pb.js';
import {authOptions} from "../middleware/AuthInterceptor";
import React, {useEffect, useState} from "react";
import {useParams} from 'react-router-dom';

const ProfilePage = () => {
    const [userId, setUserId] = useState('');
    const [firstName, setFirstName] = useState('');
    const [lastName, setLastName] = useState('');
    const [joinedAt, setJoinedAt] = useState('');
    const [bio, setBio] = useState('');

    const { id } = useParams();

    useEffect(() => {
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
            setUserId(response.getUserId());
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