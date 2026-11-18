import { getCollection } from 'astro:content';
import type { APIRoute } from 'astro';

function esc(value: string) {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

export const GET: APIRoute = async ({ site }) => {
  const origin = (site ?? new URL('https://originbyte.netlify.app')).toString().replace(/\/$/, '');
  const posts = (await getCollection('blog')).sort((a, b) => {
    const da = a.data.date ? +new Date(a.data.date) : 0;
    const db = b.data.date ? +new Date(b.data.date) : 0;
    return db - da;
  });
  const items = posts
    .map((post) => {
      const url = `${origin}/blog/${post.id}/`;
      const date = post.data.date ? new Date(post.data.date).toUTCString() : '';
      return `<item>
        <title>${esc(post.data.title)}</title>
        <link>${url}</link>
        <guid>${url}</guid>
        ${date ? `<pubDate>${date}</pubDate>` : ''}
        <description>${esc(post.data.description ?? post.data.title)}</description>
      </item>`;
    })
    .join('\n');
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Origin Byte</title>
    <link>${origin}/blog/</link>
    <description>Long reads from Origin Byte.</description>
    ${items}
  </channel>
</rss>`;
  return new Response(xml, {
    headers: { 'Content-Type': 'application/rss+xml; charset=utf-8' },
  });
};
