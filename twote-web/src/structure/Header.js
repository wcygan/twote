import React from 'react';
import {Navbar} from 'react-bootstrap';
import Home from "./Home";
import Logout from "./Logout";

function Header() {
    return (
        <Navbar bg="dark" variant="dark">
            <Home/>
            <Logout/>
        </Navbar>
    );
}

export default Header;
