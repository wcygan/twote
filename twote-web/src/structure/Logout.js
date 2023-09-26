import React from 'react';
import Cookies from 'js-cookie';
import {Button} from 'react-bootstrap';
import {AUTH_TOKEN} from "../middleware/AuthInterceptor";

const Logout = () => {
    const handleButtonClick = () => {
        Cookies.remove(AUTH_TOKEN);
        window.location.href = "/login";
    };

    return (
        <Button onClick={handleButtonClick}>
            Logout
        </Button>
    );
};

export default Logout;
