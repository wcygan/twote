import React from 'react';
import { Link } from 'react-router-dom';
import {userProfile} from "../common/UrlUtil";

const MiniProfile = ({ profile }) => {
    return (
        <div className="mini-profile">
            <Link to={userProfile(profile.getUserId())}>{profile.getFirstName()} {profile.getLastName()}</Link>
        </div>
    );
};

export default MiniProfile;
