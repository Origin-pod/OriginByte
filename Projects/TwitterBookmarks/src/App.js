import React, { useState, useEffect } from 'react';
import Tweet from './components/Tweet';
import bookmarksData from './cleaned_twitter_bookmarks_with_urls.json';

function App() {
  const [tweets, setTweets] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    try {
      // Load the tweets from the imported JSON
      if (bookmarksData && bookmarksData.tweets) {
        setTweets(bookmarksData.tweets);
        setLoading(false);
      } else {
        setError('No tweets found in the data');
        setLoading(false);
      }
    } catch (err) {
      setError('Error loading tweets: ' + err.message);
      setLoading(false);
    }
  }, []);

  if (loading) {
    return (
      <div className="app">
        <div className="header">
          <h1>Bookmarks</h1>
        </div>
        <div className="loading">Loading your bookmarks...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="app">
        <div className="header">
          <h1>Bookmarks</h1>
        </div>
        <div className="error">{error}</div>
      </div>
    );
  }

  return (
    <div className="app">
      <div className="header">
        <h1>Bookmarks</h1>
      </div>
      <div className="tweets-container">
        {tweets.map((tweet) => (
          <Tweet key={tweet.id} tweet={tweet} />
        ))}
      </div>
    </div>
  );
}

export default App;
