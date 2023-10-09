import React from 'react';
import { Navbar } from 'react-bootstrap';
import Home from "./Home";
import Logout from "./Logout";
import ProfileButton from "./ProfileButton";
import { useLocation } from 'react-router-dom';
import './Header.css';  // Import the CSS file

function Header() {
    const location = useLocation();
    const path = location.pathname;
    const showButtons = path !== '/login' && path !== '/create-account';

    return (
        <Navbar bg="dark" variant="dark" className="navbar-custom">
            {showButtons && <>
                <div className="left-buttons">
                    <Home/>
                    <ProfileButton/>
                </div>
                <div className="right-button">
                    <Logout/>
                </div>
            </>}
        </Navbar>
    );
}

export default Header;
