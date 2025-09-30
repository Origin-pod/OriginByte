import React from 'react';

const Media = ({ media }) => {
  if (!media || media.length === 0) return null;

  return (
    <div className={`media-container ${media.length === 1 ? 'single-item' : ''}`}>
      {media.map((item, index) => {
        if (item.type === 'photo') {
          return (
            <img
              key={index}
              src={item.url}
              alt="Tweet media"
              className="media-image"
              loading="lazy"
            />
          );
        } else if (item.type === 'video') {
          // Use the highest quality video variant available
          const videoUrl = item.variants?.find(variant => 
            variant.includes('.mp4') && variant.includes('1280x720')
          ) || item.variants?.find(variant => 
            variant.includes('.mp4')
          ) || item.variants?.[0];

          return (
            <video
              key={index}
              className="media-video"
              controls
              poster={item.thumbnail}
              preload="metadata"
            >
              <source src={videoUrl} type="video/mp4" />
              Your browser does not support the video tag.
            </video>
          );
        }
        return null;
      })}
    </div>
  );
};

export default Media;
