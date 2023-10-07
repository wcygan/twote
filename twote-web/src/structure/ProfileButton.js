import React from 'react';
import {Button} from 'react-bootstrap';

const Profile = () => {
    const handleButtonClick = () => {
        window.location.href = "/profile";
    };

    return (
        <Button onClick={handleButtonClick}>
            Profile
        </Button>
    );
};

export default Profile;
