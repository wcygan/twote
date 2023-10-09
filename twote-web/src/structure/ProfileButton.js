import React from 'react';
import {Button} from 'react-bootstrap';
import {MY_USER_ID} from "../middleware/AuthInterceptor";
import Cookies from 'js-cookie';
import {userProfile} from "../common/UrlUtil";

const Profile = () => {
    const handleButtonClick = () => {
        window.location.href = userProfile(Cookies.get(MY_USER_ID))
    };

    return (
        <Button onClick={handleButtonClick}>
            Profile
        </Button>
    );
};

export default Profile;
