import React from 'react';
import {Button} from 'react-bootstrap';

const Home = () => {
    const handleButtonClick = () => {
        window.location.href = "/";
    };

    return (
        <Button onClick={handleButtonClick}>
            Home
        </Button>
    );
};

export default Home;
