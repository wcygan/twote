import React from 'react';
import {Navbar} from 'react-bootstrap';
import Home from "./Home";
import Logout from "./Logout";
import ProfileButton from "./ProfileButton";

function Header() {
    return (
        <Navbar bg="dark" variant="dark">
            <Home/>
            <Logout/>
            <ProfileButton/>
        </Navbar>
    );
}

export default Header;
