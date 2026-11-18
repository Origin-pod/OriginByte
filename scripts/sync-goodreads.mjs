/**
 * Refresh src/data/goodreads.json from Goodreads.
 * 1. Public shelf RSS: /review/list_rss/<id>?shelf=...
 * 2. If that feed is empty, the public grid widget for the same shelves.
 * 3. If both fail, leave the committed snapshot untouched.
 */
import { readFile, writeFile } from 'node:fs/promises';
import { setTimeout as sleep } from 'node:timers/promises';

const USER_ID = '166403141';
const DEST = new URL('../src/data/goodreads.json', import.meta.url);
const UA = 'OriginByte-build';
const SHELVES = [
  ['currently-reading', 'reading'],
  ['read', 'read'],
  ['to-read', 'want-to-read'],
];

const decode = (value) =>
  value
    .replace(/<!\[CDATA\[([\s\S]*?)\]\]>/g, '$1')
    .replace(/&#39;|&apos;/g, "'")
    .replace(/&#34;|&quot;/g, '"')
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&#(\d+);/g, (_, n) => String.fromCharCode(Number(n)))
    .trim();

async function get(url) {
  const res = await fetch(url, { headers: { 'user-agent': UA }, redirect: 'follow' });
  const text = await res.text();
  return { ok: res.ok, status: res.status, type: res.headers.get('content-type') || '', text };
}

function rssItems(xml) {
  const books = [];
  for (const block of xml.matchAll(/<item\b[^>]*>([\s\S]*?)<\/item>/g)) {
    const field = (tag) => {
      const match = block[1].match(new RegExp(`<${tag}[^>]*>([\\s\\S]*?)</${tag}>`));
      return match ? decode(match[1]) : '';
    };
    const title = field('title');
    if (!title) continue;
    books.push({
      title,
      author: field('author_name') || field('author') || undefined,
    });
  }
  return books;
}

async function fromRss() {
  const books = [];
  for (const [shelf, status] of SHELVES) {
    for (let page = 1; page <= 20; page++) {
      const url = `https://www.goodreads.com/review/list_rss/${USER_ID}?shelf=${shelf}&page=${page}&per_page=100`;
      const { ok, status: code, type, text } = await get(url);
      if (!ok || !type.includes('xml') || !text.includes('<item')) {
        if (page === 1) throw new Error(`RSS ${shelf} returned ${code}`);
        break;
      }
      const batch = rssItems(text).map((book) => ({ ...book, status }));
      books.push(...batch);
      if (batch.length < 100) break;
    }
  }
  return books;
}

function widgetBooks(raw) {
  const match = raw.match(/widget_code = '([\s\S]*)';\s*$/);
  const html = (match ? match[1] : raw)
    .replace(/\\n/g, '\n')
    .replace(/\\'/g, "'")
    .replace(/\\"/g, '"')
    .replace(/\\\//g, '/');
  const books = [];
  for (const block of html.matchAll(/<div class="gr_grid_book_container">([\s\S]*?)<\/div>/g)) {
    const title = block[1].match(/\balt="([^"]+)"/)?.[1];
    const id = block[1].match(/\/(\d+)\._S[XY]/)?.[1];
    if (!title) continue;
    books.push({ title: decode(title), bookId: id });
  }
  return books;
}

async function authorFor(bookId, title) {
  if (!bookId) return undefined;
  try {
    const { ok, text } = await get(`https://www.goodreads.com/book/show/${bookId}`);
    if (!ok) return undefined;
    const ld = text.match(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/);
    if (ld) {
      const data = JSON.parse(ld[1]);
      const author = data.author;
      const first = (value) => {
        const name = typeof value === 'string' ? value : value?.name;
        return name ? name.split(',')[0].replace(/\s+/g, ' ').trim() : undefined;
      };
      if (typeof author === 'string' || author?.name) return first(author);
      if (Array.isArray(author)) return first(author[0]);
    }
    const tag = text.match(/<title[^>]*>(.*?)<\/title>/i)?.[1] ?? '';
    const by = tag.match(/ by (.+?) \|/);
    return by ? decode(by[1]) : undefined;
  } catch {
    return undefined;
  }
}

async function fromWidget(previous) {
  const known = new Map(previous.map((book) => [book.title, book.author]));
  const books = [];
  for (const [shelf, status] of SHELVES) {
    const url = `https://www.goodreads.com/review/grid_widget/${USER_ID}?cover_size=medium&num_books=200&order=a&shelf=${shelf}&sort=title&widget_id=1`;
    const { ok, text } = await get(url);
    if (!ok || !text.includes('widget_code')) throw new Error(`widget ${shelf} failed`);
    for (const book of widgetBooks(text)) {
      let author = known.get(book.title);
      if (!author) {
        author = await authorFor(book.bookId, book.title);
        await sleep(120);
      }
      books.push({ title: book.title, author: author || undefined, status });
    }
  }
  return books;
}

function dedupe(books) {
  const seen = new Set();
  return books.filter((book) => {
    const key = `${book.status}\0${book.title}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

const previous = JSON.parse(await readFile(DEST, 'utf8').catch(() => '[]'));
let next = [];
let via = '';
try {
  next = await fromRss();
  via = 'rss';
} catch (error) {
  console.warn(`Goodreads RSS unavailable (${error.message}).`);
}
if (!next.length) {
  try {
    next = await fromWidget(Array.isArray(previous) ? previous : []);
    via = 'widget';
  } catch (error) {
    console.warn(`Goodreads widget unavailable (${error.message}).`);
  }
}
if (!next.length) {
  console.warn('Keeping the committed Goodreads snapshot.');
  process.exit(0);
}
next = dedupe(next).map(({ title, author, status }) => ({
  title,
  ...(author ? { author } : {}),
  status,
}));
await writeFile(DEST, JSON.stringify(next, null, 2) + '\n');
const counts = Object.fromEntries(SHELVES.map(([, status]) => [status, next.filter((book) => book.status === status).length]));
console.log(`Wrote ${next.length} books via ${via}`, counts);
