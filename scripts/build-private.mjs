/**
 * Render the private shelves into an edge-function module, not into dist/.
 * The public build marks these pages as drafts, so they are not in the static site.
 */
import { readdir, readFile, writeFile, mkdir } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { marked } from 'marked';

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');
const NOTES = join(ROOT, 'src/content/docs/notes');
const OUT = join(ROOT, 'netlify/edge-functions/private-pages.js');
const SHELVES = ['theoretical-physics', 'quantum-computing', 'handbooks'];

marked.use({ gfm: true });

function frontmatter(text) {
  const match = text.match(/^---\n([\s\S]*?)\n---\n?/);
  const body = match ? text.slice(match[0].length) : text;
  const title = match?.[1].match(/^title:\s*["']?(.+?)["']?\s*$/m)?.[1] ?? 'Note';
  return { title, body };
}

function page(title, inner) {
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex, nofollow">
<title>${title}</title>
<style>
  :root { color-scheme: light dark; }
  body { margin: 0; font: 16px/1.6 "Iowan Old Style", Georgia, serif; background: #fbfbfa; color: #1c1c1a; }
  @media (prefers-color-scheme: dark) { body { background: #0f0f0e; color: #f3f2ee; } a { color: #ff8a63; } }
  main { max-width: 680px; margin: 0 auto; padding: 48px 20px 80px; }
  a { color: #d63c00; }
  h1 { font-weight: 400; letter-spacing: -0.02em; line-height: 1.15; }
  pre { overflow: auto; padding: 12px 14px; background: #151515; color: #eee; border-radius: 10px; }
  code { font-family: ui-monospace, monospace; font-size: 0.9em; }
  .nav { font: 13px/1.4 ui-sans-serif, system-ui, sans-serif; margin-bottom: 28px; }
  ul { padding-left: 1.1em; }
</style>
</head>
<body>
<main>
${inner}
</main>
</body>
</html>`;
}

function escape(value) {
  return value.replace(/[&<>"]/g, (ch) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[ch]);
}

const pages = {};
const byShelf = new Map();

for (const shelf of SHELVES) {
  const files = [];
  const walk = async (folder) => {
    for (const entry of await readdir(folder, { withFileTypes: true })) {
      const full = join(folder, entry.name);
      if (entry.isDirectory()) await walk(full);
      else if (/\.mdx?$/.test(entry.name)) files.push(full);
    }
  };
  await walk(join(NOTES, shelf));
  const notes = [];
  for (const file of files.sort()) {
    const raw = await readFile(file, 'utf8');
    const { title, body } = frontmatter(raw);
    const rel = relative(NOTES, file).replace(/\.mdx?$/, '').replace(/\\/g, '/');
    const href = `/notes/${rel.endsWith('/index') || rel === 'index' ? rel.replace(/\/?index$/, '') : rel}/`.replace(/\/{2,}/g, '/');
    const html = page(
      escape(title),
      `<p class="nav"><a href="/api/void">Sealed shelves</a></p><h1>${escape(title)}</h1>${marked.parse(body)}`,
    );
    pages[href] = html;
    notes.push({ href, title });
  }
  byShelf.set(shelf, notes);
}

const indexItems = SHELVES.map((shelf) => {
  const notes = byShelf.get(shelf) ?? [];
  const links = notes.map((note) => `<li><a href="${note.href}">${escape(note.title)}</a></li>`).join('');
  return `<h2>${escape(shelf.replace(/-/g, ' '))}</h2><ul>${links}</ul>`;
}).join('');

pages['/api/void'] = page('Sealed', `<p class="nav"><a href="/">← index</a></p><h1>Sealed shelves</h1>${indexItems}`);

await mkdir(dirname(OUT), { recursive: true });
await writeFile(OUT, `export const pages = ${JSON.stringify(pages)};\n`);
console.log(`private pages: ${Object.keys(pages).length}`);
