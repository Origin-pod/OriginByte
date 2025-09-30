import React, { useState, useEffect, useMemo } from 'react';
import Tweet from './components/Tweet';
import bookmarksData from './cleaned_twitter_bookmarks_with_urls.json';

function App() {
  const [tweets, setTweets] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');

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

  // Fuzzy search function
  const fuzzySearch = (query, text) => {
    if (!query) return true;
    
    const queryLower = query.toLowerCase();
    const textLower = text.toLowerCase();
    
    // Direct substring match gets highest priority
    if (textLower.includes(queryLower)) return true;
    
    // Fuzzy matching - check if all characters in query appear in order
    let queryIndex = 0;
    for (let i = 0; i < textLower.length && queryIndex < queryLower.length; i++) {
      if (textLower[i] === queryLower[queryIndex]) {
        queryIndex++;
      }
    }
    return queryIndex === queryLower.length;
  };

  // Filter tweets based on search query
  const filteredTweets = useMemo(() => {
    if (!searchQuery.trim()) return tweets;
    
    return tweets.filter(tweet => {
      // Search in tweet text
      if (fuzzySearch(searchQuery, tweet.text || '')) return true;
      
      // Search in user name
      if (fuzzySearch(searchQuery, tweet.user?.name || '')) return true;
      
      // Search in user handle
      if (fuzzySearch(searchQuery, tweet.user?.screen_name || '')) return true;
      
      return false;
    });
  }, [tweets, searchQuery]);

  const handleSearchChange = (e) => {
    setSearchQuery(e.target.value);
  };

  const clearSearch = () => {
    setSearchQuery('');
  };

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
        <div className="search-container">
          <div className="search-input-wrapper">
            <input
              type="text"
              placeholder="Search bookmarks..."
              value={searchQuery}
              onChange={handleSearchChange}
              className="search-input"
            />
            {searchQuery && (
              <button onClick={clearSearch} className="clear-search">
                ×
              </button>
            )}
          </div>
          {searchQuery && (
            <div className="search-results-count">
              {filteredTweets.length} result{filteredTweets.length !== 1 ? 's' : ''}
            </div>
          )}
        </div>
      </div>
      <div className="tweets-container">
        {filteredTweets.length > 0 ? (
          filteredTweets.map((tweet) => (
            <Tweet key={tweet.id} tweet={tweet} />
          ))
        ) : searchQuery ? (
          <div className="no-results">
            No bookmarks found matching "{searchQuery}"
          </div>
        ) : (
          tweets.map((tweet) => (
            <Tweet key={tweet.id} tweet={tweet} />
          ))
        )}
      </div>
    </div>
  );
}

export default App;
