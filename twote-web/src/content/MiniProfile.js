import React from 'react';
import { Link } from 'react-router-dom';
import {userProfile} from "../common/UrlUtil";
import './MiniProfile.css';

const MiniProfile = ({ profile }) => {
    return (
        <Link to={userProfile(profile.getUserId())} style={{ textDecoration: 'none' }}>
            <div className="mini-profile">
                {profile.getFirstName()} {profile.getLastName()}
            </div>
        </Link>
    );
};

export default MiniProfile;
