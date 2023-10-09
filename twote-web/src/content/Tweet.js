import React from 'react';
import {date_from_seconds, timeSince} from "../common/DateUtil";
import './Tweet.css';
import {userProfile} from "../common/UrlUtil";
import {Link} from "react-router-dom";

function Tweet({ tweet }) {
    console.log(tweet);
    return (
        <div className="tweet">
            <div className="tweet-header">
                <Link
                    to={userProfile(tweet.getUserId())}
                    className="user-name">{tweet.getFirstName()} {tweet.getLastName()}
                </Link>
                <span className="tweet-time">{timeSince(date_from_seconds(tweet.getCreatedAt().getSeconds()))}</span>
            </div>
            <div className="tweet-body">
                <p>{tweet.getMessage()}</p>
            </div>
        </div>
    );
}

export default Tweet;
