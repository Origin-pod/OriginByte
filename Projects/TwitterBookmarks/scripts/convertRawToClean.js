#!/usr/bin/env node
/*
Usage:
  node scripts/convertRawToClean.js <inputRaw.json> <outputClean.json>

Reads a raw Twitter API (GraphQL timeline) JSON and outputs a cleaned schema:
{
  exported_at: ISOString,
  tweets: [
    {
      id, created_at, text, lang,
      user: { id, name, screen_name, verified, avatar_url },
      engagement: { views, bookmarks, likes, retweets, replies, quotes },
      urls: [ ... ],
      media: [ { type: 'photo'|'video', url?, thumbnail?, variants?[] } ],
      bookmarked: true|false,
      possibly_sensitive, source, tweet_url
    }
  ]
}
*/

const fs = require('fs');
const path = require('path');

const [,, inFile, outFile] = process.argv;
if (!inFile || !outFile) {
  console.error('Usage: node scripts/convertRawToClean.js <inputRaw.json> <outputClean.json>');
  process.exit(1);
}

const readJson = (p) => JSON.parse(fs.readFileSync(p, 'utf8'));

// Helpers
const get = (obj, fn, def = undefined) => {
  try { const v = fn(obj); return v === undefined ? def : v; } catch { return def; }
};

const extractTweetFromEntry = (entry) => {
  const result = get(entry, e => e.content.itemContent.tweet_results.result) || {};
  if (!result || typeof result !== 'object') return null;

  const legacy = result.legacy || {};
  const coreUser = get(result, r => r.core.user_results.result) || {};
  const legacyUser = coreUser.legacy || {};

  const id = result.rest_id || legacy.id_str;
  const created_at = legacy.created_at || get(result, r => r.core.created_at);
  const full_text = legacy.full_text || get(result, r => r.note_tweet.note_tweet_results.result.text) || '';
  const lang = legacy.lang || get(result, r => r.lang);

  const name = get(coreUser, u => u.core.name) || legacyUser.name;
  const screen_name = get(coreUser, u => u.core.screen_name) || legacyUser.screen_name;
  const user_id = coreUser.rest_id || legacyUser.id_str;
  const verified = get(coreUser, u => u.is_blue_verified) || false;
  const avatar_url = get(coreUser, u => u.avatar.image_url);

  // Engagement
  const views = Number(get(result, r => r.views.count)) || 0;
  const bookmarks = Number(legacy.bookmark_count) || 0;
  const likes = Number(legacy.favorite_count) || 0;
  const retweets = Number(legacy.retweet_count) || 0;
  const replies = Number(legacy.reply_count) || 0;
  const quotes = Number(legacy.quote_count) || 0;

  // URLs from note_tweet or entities
  const urls = [];
  const entityUrls = get(legacy, l => l.entities.urls) || [];
  for (const u of entityUrls) {
    if (u?.expanded_url) urls.push(u.expanded_url);
  }

  // Media extraction (prefer extended_entities)
  const mediaOut = [];
  const extMedia = get(legacy, l => l.extended_entities.media) || get(legacy, l => l.entities.media) || [];
  for (const m of extMedia) {
    if (!m || typeof m !== 'object') continue;
    const type = m.type;
    if (type === 'photo') {
      mediaOut.push({ type: 'photo', url: m.media_url_https || m.media_url });
    } else if (type === 'video' || type === 'animated_gif') {
      const thumb = m.media_url_https || m.media_url;
      const variants = [];
      const vinfo = m.video_info || {};
      const vvars = vinfo.variants || [];
      for (const v of vvars) {
        if (v.url) variants.push(v.url);
      }
      mediaOut.push({ type: 'video', thumbnail: thumb, variants });
    }
  }

  // Bookmark flag best-effort
  const bookmarked = !!legacy.bookmarked;
  const possibly_sensitive = !!legacy.possibly_sensitive;
  const source = legacy.source || get(result, r => r.source);

  const tweet_url = (screen_name && id)
    ? `https://x.com/${screen_name}/status/${id}`
    : undefined;

  return {
    id,
    created_at,
    text: full_text,
    lang,
    user: { id: user_id, name, screen_name, verified, avatar_url },
    engagement: { views, bookmarks, likes, retweets, replies, quotes },
    urls,
    media: mediaOut,
    bookmarked,
    possibly_sensitive,
    source,
    tweet_url,
  };
};

const collectEntries = (raw) => {
  const instructions = get(raw, r => r.data.bookmark_timeline_v2.timeline.instructions) ||
                       get(raw, r => r.data.home.home_timeline_urt.instructions) ||
                       get(raw, r => r.timeline.instructions) || [];
  const entries = [];
  for (const instr of instructions) {
    const arr = instr?.entries || instr?.addEntries?.entries || [];
    for (const e of arr) entries.push(e);
  }
  return entries;
};

try {
  const raw = readJson(path.resolve(inFile));
  const entries = collectEntries(raw);
  const tweets = [];
  for (const entry of entries) {
    const t = extractTweetFromEntry(entry);
    if (t && t.id && t.text !== undefined) tweets.push(t);
  }

  // If output file exists, append to its tweets (de-duplicate by id)
  let finalTweets = tweets;
  const outAbs = path.resolve(outFile);
  if (fs.existsSync(outAbs)) {
    try {
      const existing = JSON.parse(fs.readFileSync(outAbs, 'utf8'));
      const existingTweets = Array.isArray(existing?.tweets) ? existing.tweets : [];
      const seen = new Set();
      const merged = [];
      // Keep existing order first
      for (const et of existingTweets) {
        if (et && et.id && !seen.has(et.id)) {
          seen.add(et.id);
          merged.push(et);
        }
      }
      // Append new tweets not already present
      for (const nt of tweets) {
        if (nt && nt.id && !seen.has(nt.id)) {
          seen.add(nt.id);
          merged.push(nt);
        }
      }
      finalTweets = merged;
    } catch (_) {
      // If parsing existing fails, fall back to fresh output
      finalTweets = tweets;
    }
  }

  const out = {
    exported_at: new Date().toISOString(),
    count: Array.isArray(finalTweets) ? finalTweets.length : 0,
    tweets: finalTweets,
  };
  fs.writeFileSync(path.resolve(outFile), JSON.stringify(out, null, 2));
  console.log(`Converted ${tweets.length} tweets -> ${outFile}`);
} catch (err) {
  console.error('Conversion failed:', err.message);
  process.exit(1);
}
