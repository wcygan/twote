use schemas::profile::Profile;
use schemas::tweet::Tweet;

pub fn tweet_details(tweet: Tweet, profile: &Profile) -> schemas::frontend::Tweet {
    schemas::frontend::Tweet {
        tweet_id: tweet.tweet_id,
        user_id: tweet.user_id,
        first_name: profile.first_name.clone(),
        last_name: profile.last_name.clone(),
        message: tweet.message,
        created_at: tweet.created_at,
        parent_tweet_id: tweet.parent_tweet_id,
    }
}
