import React from 'react';
import Media from './Media';

const Tweet = ({ tweet }) => {
  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric'
    });
  };

  const formatNumber = (num) => {
    if (num >= 1000000) {
      return (num / 1000000).toFixed(1) + 'M';
    } else if (num >= 1000) {
      return (num / 1000).toFixed(1) + 'K';
    }
    return num.toString();
  };

  const formatTweetText = (text, urls) => {
    let formattedText = text;
    
    // Replace URLs with clickable links
    urls?.forEach(url => {
      const linkText = url.length > 30 ? url.substring(0, 30) + '...' : url;
      formattedText = formattedText.replace(
        url,
        `<a href="${url}" target="_blank" rel="noopener noreferrer">${linkText}</a>`
      );
    });

    // Replace t.co links that might not be in the urls array
    formattedText = formattedText.replace(
      /https:\/\/t\.co\/\w+/g,
      (match) => `<a href="${match}" target="_blank" rel="noopener noreferrer">${match}</a>`
    );

    return { __html: formattedText };
  };

  const getInitials = (name) => {
    return name
      .split(' ')
      .map(word => word[0])
      .join('')
      .substring(0, 2)
      .toUpperCase();
  };

  return (
    <div className="tweet" onClick={() => window.open(tweet.tweet_url, '_blank')}>
      <div className="tweet-header">
        <div className="avatar">
          {tweet.user?.avatar_url ? (
            <img
              src={tweet.user.avatar_url}
              alt={`${tweet.user.name} avatar`}
              className="avatar-img"
              loading="lazy"
            />
          ) : (
            getInitials(tweet.user.name)
          )}
        </div>
        <div className="tweet-info">
          <div className="user-info">
            <span className="user-name">{tweet.user.name}</span>
            <span className="user-handle">@{tweet.user.screen_name}</span>
            <span className="tweet-date">· {formatDate(tweet.created_at)}</span>
          </div>
        </div>
      </div>
      
      <div className="tweet-content">
        <div 
          className="tweet-text"
          dangerouslySetInnerHTML={formatTweetText(tweet.text, tweet.urls)}
        />
        
        <Media media={tweet.media} />
        
        <div className="engagement">
          <div className="engagement-item">
            <svg className="engagement-icon" viewBox="0 0 24 24">
              <path d="M1.751 10c0-4.42 3.584-8 8.005-8h4.366c4.49 0 8.129 3.64 8.129 8.13 0 2.96-1.607 5.68-4.196 7.11l-8.054 4.46v-3.69h-.067c-4.49.1-8.183-3.51-8.183-8.01z"/>
            </svg>
            <span>{formatNumber(tweet.engagement.replies)}</span>
          </div>
          
          <div className="engagement-item retweets">
            <svg className="engagement-icon" viewBox="0 0 24 24">
              <path d="M4.5 3.88l4.432 4.14-1.364 1.46L5.5 7.55V16c0 1.1.896 2 2 2H13v2H7.5c-2.209 0-4-1.79-4-4V7.55L1.432 9.48.068 8.02 4.5 3.88zM16.5 6H11V4h5.5c2.209 0 4 1.79 4 4v8.45l2.068-1.93 1.364 1.46-4.432 4.14-4.432-4.14 1.364-1.46 2.068 1.93V8c0-1.1-.896-2-2-2z"/>
            </svg>
            <span>{formatNumber(tweet.engagement.retweets)}</span>
          </div>
          
          <div className="engagement-item likes">
            <svg className="engagement-icon" viewBox="0 0 24 24">
              <path d="M16.697 5.5c-1.222-.06-2.679.51-3.89 2.16l-.805 1.09-.806-1.09C9.984 6.01 8.526 5.44 7.304 5.5c-1.243.07-2.349.78-2.91 1.91-.552 1.12-.633 2.78.479 4.82 1.074 1.97 3.257 4.27 7.129 6.61 3.87-2.34 6.052-4.64 7.126-6.61 1.111-2.04 1.03-3.7.477-4.82-.561-1.13-1.666-1.84-2.908-1.91z"/>
            </svg>
            <span>{formatNumber(tweet.engagement.likes)}</span>
          </div>
          
          <div className="engagement-item">
            <svg className="engagement-icon" viewBox="0 0 24 24">
              <path d="M12 2.59l5.7 5.7-1.41 1.42L13 6.41V16h-2V6.41l-3.3 3.3-1.41-1.42L12 2.59zM21 15l-.02 3.51c0 1.38-1.12 2.49-2.5 2.49H5.5C4.11 21 3 19.88 3 18.5V15h2v3.5c0 .28.22.5.5.5h12.98c.28 0 .5-.22.5-.5L19 15h2z"/>
            </svg>
            <span>{formatNumber(tweet.engagement.views)}</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Tweet;
