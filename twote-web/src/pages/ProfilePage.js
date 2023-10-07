import { ProfileServiceClient } from '../proto/profile_grpc_web_pb.js';
import { GetProfileRequest } from '../proto/profile_pb.js';
import {authOptions} from "../middleware/AuthInterceptor";
import {useEffect} from "react";

const ProfilePage = () => {
    useEffect(() => {
        console.log("tom is a bot")
        // const client = new ProfileServiceClient("http://localhost:8080", null, authOptions);
        //
        // const request = new GetProfileRequest();
        //
        // client.get(request, {}, (err, response) => {
        //     if (err) {
        //         console.error(err);
        //         return;
        //     }
        //     console.log(response.toObject());
        // });
    }, []);

    return <div>Your content here</div>;
};

export default ProfilePage;