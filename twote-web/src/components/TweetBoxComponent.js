import React, {useState} from 'react';
import {CreateTweetRequest} from "../proto/backend/tweet_pb.js";
import {TweetServiceClient} from "../proto/backend/tweet_grpc_web_pb.js";
import {Button} from "react-bootstrap";
import {useNavigate} from 'react-router-dom';
import Cookies from 'js-cookie';
import {authOptions, MY_USER_ID} from "../middleware/AuthInterceptor";

function TweetBoxComponent() {
    const navigate = useNavigate();
    const [content, setContent] = useState('');

    const sendRequest = () => {
        const client = new TweetServiceClient("http://localhost:8080", null, authOptions);

        const request = new CreateTweetRequest();
        request.setMessage(content);
        request.setUserId(Cookies.get(MY_USER_ID));

        client.create(request, {}, (err, response) => {
            if (err) {
                console.error(err);
            } else {
                // Since there is a new tweet, we need to reload the page to see it.
                // We could also add the tweet to the list of tweets without reloading the page.
                window.location.reload();
            }
        });
    }

    return (
        <div className="tweet-box">
            <textarea
                className="tweet-box-textarea"
                placeholder="What's happening?"
                value={content}
                onChange={(e) => setContent(e.target.value)}
            />
            <Button
                className="tweet-box-button"
                variant="primary"
                onClick={sendRequest}
            >
                Tweet
            </Button>
        </div>
    );
}

export default TweetBoxComponent;