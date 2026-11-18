import { pages } from './private-pages.js';

const SHELVES = ['theoretical-physics', 'quantum-computing', 'handbooks'];

function notFound() {
  return new Response('Not found', {
    status: 404,
    headers: { 'content-type': 'text/plain; charset=utf-8', 'cache-control': 'no-store' },
  });
}

function html(body) {
  return new Response(body, {
    status: 200,
    headers: {
      'content-type': 'text/html; charset=utf-8',
      'cache-control': 'private, no-store',
      'x-robots-tag': 'noindex, nofollow',
    },
  });
}

function isPrivatePath(pathname) {
  return SHELVES.some((shelf) => pathname === `/notes/${shelf}` || pathname.startsWith(`/notes/${shelf}/`));
}

function safeEqual(a, b) {
  const enc = new TextEncoder();
  const left = enc.encode(a);
  const right = enc.encode(b);
  const len = Math.max(left.length, right.length);
  let diff = left.length ^ right.length;
  for (let i = 0; i < len; i++) diff |= (left[i] || 0) ^ (right[i] || 0);
  return diff === 0;
}

async function sign(value, secret) {
  const enc = new TextEncoder();
  const key = await crypto.subtle.importKey(
    'raw',
    enc.encode(secret),
    { name: 'HMAC', hash: 'SHA-256' },
    false,
    ['sign'],
  );
  const sig = await crypto.subtle.sign('HMAC', key, enc.encode(value));
  return [...new Uint8Array(sig)].map((byte) => byte.toString(16).padStart(2, '0')).join('');
}

async function authed(cookie, secret) {
  if (!secret || !cookie) return false;
  const [exp, sig] = cookie.split('.');
  if (!exp || !sig || Number(exp) < Date.now()) return false;
  const expected = await sign(exp, secret);
  return safeEqual(sig, expected);
}

export default async (request, context) => {
  const url = new URL(request.url);
  const secret = Netlify.env.get('VOID_PASSWORD') || '';
  const cookie = context.cookies.get('ob-void');
  const open = await authed(cookie, secret);

  if (url.pathname === '/api/void') {
    if (request.method === 'POST') {
      const form = await request.formData();
      const word = String(form.get('word') ?? '');
      if (!secret || !safeEqual(word, secret)) {
        return new Response('That word does not open it.', {
          status: 401,
          headers: { 'content-type': 'text/plain; charset=utf-8', 'cache-control': 'no-store' },
        });
      }
      const exp = String(Date.now() + 14 * 24 * 60 * 60 * 1000);
      context.cookies.set({
        name: 'ob-void',
        value: `${exp}.${await sign(exp, secret)}`,
        path: '/',
        httpOnly: true,
        secure: true,
        sameSite: 'Lax',
        maxAge: 14 * 24 * 60 * 60,
      });
      return new Response(null, { status: 303, headers: { location: '/api/void', 'cache-control': 'no-store' } });
    }
    if (!open) return notFound();
    return html(pages['/api/void'] || '');
  }

  if (!isPrivatePath(url.pathname)) return context.next();
  if (!open) return notFound();
  const key = url.pathname.endsWith('/') ? url.pathname : `${url.pathname}/`;
  const body = pages[key];
  return body ? html(body) : notFound();
};

export const config = {
  path: [
    '/api/void',
    '/notes/theoretical-physics',
    '/notes/theoretical-physics/*',
    '/notes/quantum-computing',
    '/notes/quantum-computing/*',
    '/notes/handbooks',
    '/notes/handbooks/*',
  ],
};
