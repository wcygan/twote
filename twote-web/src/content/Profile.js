import React from 'react';
import {Link} from 'react-router-dom';
import {userProfile} from "../common/UrlUtil";
import './Profile.css';

const Profile = ({ profile }) => {
    console.log(profile);
    return (
        <Link to={userProfile(profile.getUserId())} className="profile-link">
            <div className="profile">
                {profile.getFirstName()} {profile.getLastName()}
            </div>
        </Link>
    );
};

export default Profile;
