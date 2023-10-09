import React from 'react';
import {Link} from 'react-router-dom';
import {userProfile} from "../common/UrlUtil";

const Profile = ({ profile }) => {
    console.log(profile);
    return (
        <div className="profile">
            <Link to={userProfile(profile.getUserId())}>{profile.getFirstName()} {profile.getLastName()}</Link>
        </div>
    );
};

export default Profile;
