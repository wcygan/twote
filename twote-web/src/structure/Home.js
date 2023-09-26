import React from 'react';
import Cookies from 'js-cookie';
import {Button} from 'react-bootstrap';
import {AUTH_TOKEN} from "../middleware/AuthInterceptor";

const Home = () => {
    const handleButtonClick = () => {
        if (Cookies.get(AUTH_TOKEN)) {
            window.location.href = "/";
        } else {
            window.location.href = "/login";
        }
    };

    return (
        <Button onClick={handleButtonClick}>
            Home
        </Button>
    );
};

export default Home;
