import React from 'react';
import {Navbar} from 'react-bootstrap';
import Home from "./Home";
import Logout from "./Logout";
import ProfileButton from "./ProfileButton";
import { useLocation } from 'react-router-dom';

function Header() {
    const location = useLocation();
    const path = location.pathname;
    const showButtons = path !== '/login' && path !== '/create-account';

    return (
        <Navbar bg="dark" variant="dark">
            {showButtons && <>
                <Home/>
                <Logout/>
                <ProfileButton/>
            </>}
        </Navbar>
    );
}

export default Header;
