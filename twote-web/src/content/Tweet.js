import React from 'react';
import {date_from_seconds} from "../common/DateUtil";

function Tweet({ tweet }) {
    return (
        <div className="tweet">
            <div className="tweet-header">
                <span className="user-name">{tweet.getFirstName()} {tweet.getLastName()}</span>
                <span className="user-id">@{tweet.getUserId()}</span>
            </div>
            <div className="tweet-body">
                <p>{tweet.getMessage()}</p>
            </div>
            <div className="tweet-footer">
                <span className="tweet-time">{date_from_seconds(tweet.getCreatedAt().getSeconds())}</span>
                {tweet.getParentTweetId().length === 0 && <span className="reply-to">Replying to {tweet.getParentTweetId()}</span>}
            </div>
        </div>
    );
}

export default Tweet;
