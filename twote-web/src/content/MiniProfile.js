import React from 'react';
import { Link } from 'react-router-dom';
import {userProfile} from "../common/UrlUtil";

const MiniProfile = ({ id, firstName, lastName }) => {
    return (
        <div className="mini-profile">
            <Link to={userProfile(id)}>{firstName} {lastName}</Link>
        </div>
    );
};

export default MiniProfile;
