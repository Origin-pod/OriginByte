/* ============ Origin Byte · Index — shared behaviour ============ */
(() => {
if (window.__obIndex) return;
window.__obIndex = true;
const d = document, root = d.documentElement, body = d.body;
const RM = matchMedia('(prefers-reduced-motion: reduce)');
const reduced = () => RM.matches;
const page = body.dataset.page || '';
const $ = (s, el = d) => el.querySelector(s), $$ = (s, el = d) => [...el.querySelectorAll(s)];

/* ---------- shared chrome (only if the server didn't render it) ---------- */
const NAV = [['home','Index','/','00'],['work','Work','/work/','01'],['lab','Lab','/hacks/','02'],['notes','Notes','/notes/','03'],['writing','Writing','/blog/','04'],['library','Library','/library/','05']];
const sun = '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"><circle cx="8" cy="8" r="3"/><path d="M8 1.5v1.3M8 13.2v1.3M1.5 8h1.3M13.2 8h1.3M3.4 3.4l.9.9M11.7 11.7l.9.9M3.4 12.6l.9-.9M11.7 4.3l.9-.9"/></svg>';
const moon = '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"><path d="M13.5 9.6A5.6 5.6 0 0 1 6.4 2.5a5.6 5.6 0 1 0 7.1 7.1Z"/></svg>';
const cur = k => k === page ? ' aria-current="page"' : '';
if (!d.querySelector('[data-chrome]')) {
  const chrome = `
<div class="edge t"></div><div class="edge b"></div>
<a class="brand" href="/"><span class="rune" data-rune title="">🜲</span>Origin Byte</a>
<div class="theme" role="group" aria-label="Theme"><span class="knob"></span><button data-set="light" aria-label="Light">${sun}</button><button data-set="dark" aria-label="Dark">${moon}</button></div>
<div class="topr"><span><span class="ist mono">--:--</span> IST</span><button type="button" data-palette>⌘K</button></div>
<nav class="rail" aria-label="Sections">${NAV.map(([k,l,h])=>`<a href="${h}"${cur(k)}>${l}</a>`).join('')}
<div class="g">elsewhere</div><a href="https://github.com/Origin-pod">GitHub <span class="arr">↗</span></a><a href="https://x.com/sheetalssr">X <span class="arr">↗</span></a><a href="https://www.linkedin.com/in/sheetal-singh-218855176">LinkedIn <span class="arr">↗</span></a></nav>
<header class="mbar"><a class="b" href="/"><span class="rune" data-rune>🜲</span>Origin Byte</a><div class="r"><button class="ic" data-toggle-theme aria-label="Toggle theme"><span class="i-l">${moon}</span></button><button class="ic burger" data-menu aria-expanded="false"><i></i>Menu</button></div></header>
<div class="scrim" data-menu-close></div>
<div class="sheet" aria-label="Menu"><nav>${NAV.map(([k,l,h,n],i)=>`<a href="${h}"${cur(k)} style="transition-delay:${80+i*45}ms">${l}<span>${n}</span></a>`).join('')}</nav>
<div class="el"><a href="https://github.com/Origin-pod">GitHub ↗</a><a href="https://x.com/sheetalssr">X ↗</a><a href="https://www.linkedin.com/in/sheetal-singh-218855176">LinkedIn ↗</a><a href="/enter-the-void/">🜲</a></div></div>
<div class="toast" role="status" aria-live="polite"></div>`;
  body.insertAdjacentHTML('afterbegin', chrome);
  if (!body.dataset.nosurprise) body.insertAdjacentHTML('beforeend', `<div class="surprise"><button class="btn mag" data-surprise>Surprise me <span class="kbd">R</span></button></div>`);
}
const ft = $('#ft');
if (ft) ft.outerHTML = `<footer class="ft"><span>© 2026 Sheetal Singh <button class="paw" type="button" data-paw aria-label="A small mark">🐾</button></span><span class="r"><button class="ssr-dev-indicator" data-ssr-lock type="button" hidden>dev mode</button><span class="mono" style="font-size:12px">Bengaluru · <span class="ist">--:--</span> IST</span><a class="ft-rune" data-rune data-ssr-rune href="/enter-the-void/" title="speak, friend">🜲</a></span></footer>`;

/* ---------- clock ---------- */
const tick = () => { const t = new Date().toLocaleTimeString('en-GB',{timeZone:'Asia/Kolkata',hour:'2-digit',minute:'2-digit'}); $$('.ist').forEach(e=>e.textContent=t); };
tick(); setInterval(tick, 20000);

/* ---------- toast ---------- */
let tt; const toast = (html, ms = 2600) => { const el = $('.toast'); if (!el) return; el.innerHTML = html; el.classList.add('show'); clearTimeout(tt); tt = setTimeout(()=>el.classList.remove('show'), ms); };

/* ---------- theme ---------- */
const setTheme = (t, persist = true) => { root.dataset.theme = t; try { localStorage.setItem('starlight-theme', t); } catch (e) {} if (persist) localStorage.setItem('ob-theme', t); $$('.theme button').forEach(b=>b.setAttribute('aria-pressed', b.dataset.set === t)); const ml = $('.mbar .i-l'); if (ml) ml.innerHTML = t === 'dark' ? sun : moon; };
setTheme(root.dataset.theme || 'light', false);
const flip = (t, ev) => {
  if (t === root.dataset.theme) return;
  if (!d.startViewTransition || reduced()) { root.classList.add('theme-fade'); setTheme(t); setTimeout(()=>root.classList.remove('theme-fade'), 400); return; }
  const r = ev && ev.currentTarget ? ev.currentTarget.getBoundingClientRect() : {left:innerWidth/2,top:0,width:0,height:0};
  const x = r.left + r.width/2, y = r.top + r.height/2, R = Math.hypot(Math.max(x, innerWidth-x), Math.max(y, innerHeight-y));
  const vt = d.startViewTransition(() => setTheme(t));
  vt.ready.then(() => root.animate({ clipPath:[`circle(0px at ${x}px ${y}px)`, `circle(${R}px at ${x}px ${y}px)`] }, { duration: 650, easing: 'cubic-bezier(.65,0,.35,1)', pseudoElement: '::view-transition-new(root)' }));
};
$$('.theme button').forEach(b => b.addEventListener('click', e => flip(b.dataset.set, e)));
$$('[data-toggle-theme]').forEach(b => b.addEventListener('click', e => flip(root.dataset.theme === 'dark' ? 'light' : 'dark', e)));
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => { if (!localStorage.getItem('ob-theme')) setTheme(e.matches ? 'dark' : 'light', false); });

/* ---------- mobile menu ---------- */
const menu = open => { body.classList.toggle('menu-open', open); const b = $('[data-menu]'); if (b) b.setAttribute('aria-expanded', String(open)); if (open) { const first = $('.sheet nav a'); if (first) first.focus(); } };
const menuBtn = $('[data-menu]'); if (menuBtn) menuBtn.addEventListener('click', () => menu(!body.classList.contains('menu-open')));
const menuClose = $('[data-menu-close]'); if (menuClose) menuClose.addEventListener('click', () => menu(false));

/* ---------- staggered reveal ---------- */
$$('[data-r]').forEach((el, i) => el.style.setProperty('--i', Math.min(i, 18)));
requestAnimationFrame(() => requestAnimationFrame(() => root.classList.add('ready')));

/* ---------- hover-dim lists with a sliding highlight ---------- */
$$('.list').forEach(list => {
  const hl = d.createElement('span'); hl.className = 'hl'; list.prepend(hl);
  const rows = $$('.row', list);
  rows.forEach(r => r.addEventListener('pointerenter', () => {
    rows.forEach(x => x.classList.toggle('on', x === r)); list.classList.add('dim');
    hl.style.transform = `translateY(${r.offsetTop}px)`; hl.style.height = r.offsetHeight + 'px'; hl.style.opacity = 1;
  }));
  list.addEventListener('pointerleave', () => { list.classList.remove('dim'); rows.forEach(x=>x.classList.remove('on')); hl.style.opacity = 0; });
});

/* ---------- magnetic buttons ---------- */
const fine = matchMedia('(pointer:fine)').matches;
if (fine) $$('.mag').forEach(b => {
  b.addEventListener('pointermove', e => { if (reduced()) return; const r = b.getBoundingClientRect(); b.style.transition = 'transform .15s linear'; b.style.transform = `translate(${(e.clientX - r.left - r.width/2)*.22}px,${(e.clientY - r.top - r.height/2)*.35}px)`; });
  b.addEventListener('pointerleave', () => { b.style.transition = ''; b.style.transform = ''; });
});

/* ---------- segmented controls & chips ---------- */
$$('.seg').forEach(seg => {
  const pill = d.createElement('span'); pill.className = 'pill'; seg.prepend(pill);
  const place = () => { const a = $('button[aria-pressed="true"]', seg); if (!a) return; pill.style.width = a.offsetWidth + 'px'; pill.style.transform = `translateX(${a.offsetLeft}px)`; };
  $$('button', seg).forEach(b => b.addEventListener('click', () => { $$('button', seg).forEach(x => x.setAttribute('aria-pressed', x === b)); place(); seg.dispatchEvent(new CustomEvent('pick', { detail: b.dataset.v })); }));
  place(); addEventListener('resize', place); d.fonts && d.fonts.ready.then(place);
});
$$('.chips').forEach(c => $$('button', c).forEach(b => b.addEventListener('click', () => { $$('button', c).forEach(x => x.setAttribute('aria-pressed', x === b)); c.dispatchEvent(new CustomEvent('pick', { detail: b.dataset.v })); })));

/* ---------- book covers: tilt + pull from shelf ---------- */
$$('.book').forEach(bk => {
  const cv = $('.cover', bk); if (!cv) return;
  if (fine) bk.addEventListener('pointermove', e => { if (reduced()) return; const r = cv.getBoundingClientRect(); const px = (e.clientX - r.left)/r.width, py = (e.clientY - r.top)/r.height;
    cv.style.setProperty('--ry', ((px - .5) * 18) + 'deg'); cv.style.setProperty('--rx', ((.5 - py) * 14) + 'deg'); cv.style.setProperty('--gx', px*100 + '%'); cv.style.setProperty('--gy', py*100 + '%'); });
  bk.addEventListener('pointerleave', () => { cv.style.setProperty('--rx','0deg'); cv.style.setProperty('--ry','0deg'); });
  bk.addEventListener('click', e => { const href = bk.getAttribute('href'); if (!href || href === '#' || reduced()) return; if (bk.dataset.nopull != null) return; e.preventDefault(); bk.classList.add('pull'); setTimeout(() => location.href = href, 380); });
});

/* ---------- the rune: flips, and keeps a secret ---------- */
let clicks = 0, ct, turns = 0;
const sparks = el => { const r = el.getBoundingClientRect(); for (let i = 0; i < 14; i++) { const s = d.createElement('span'); s.className = 'spark'; s.textContent = '🜲'; s.style.left = r.left + r.width/2 - 7 + 'px'; s.style.top = r.top + r.height/2 - 9 + 'px'; body.appendChild(s);
  const a = Math.random()*Math.PI*2, v = 50 + Math.random()*90; s.animate([{transform:'translate(0,0) scale(.4) rotate(0)',opacity:1},{transform:`translate(${Math.cos(a)*v}px,${Math.sin(a)*v + 60}px) scale(1) rotate(${Math.random()*360}deg)`,opacity:0}],{duration:900+Math.random()*500,easing:'cubic-bezier(.22,1,.36,1)'}).onfinish = () => s.remove(); } };
$$('[data-rune]').forEach(r => r.addEventListener('click', e => {
  if (r.hasAttribute('data-ssr-rune')) {
    try { if (sessionStorage.getItem('ssr:devmode') === '1') { e.preventDefault(); sessionStorage.removeItem('ssr:devmode'); location.reload(); return; } } catch (err) {}
  }
  if (r.tagName === 'A' && clicks < 4) e.preventDefault();
  if (r.closest('.brand,.b') ) e.preventDefault();
  turns += 180; r.style.transform = `rotate(${turns}deg)`;
  clicks++; clearTimeout(ct); ct = setTimeout(() => clicks = 0, 1600);
  if (clicks === 3) toast('The rune hums…');
  if (clicks >= 5) { clicks = 0; $$('.rune').forEach(x => x.classList.add('glow')); if (!reduced()) sparks(r); toast('speak, friend, and enter. <a href="/enter-the-void/">open the door →</a>', 4200); setTimeout(() => $$('.rune').forEach(x => x.classList.remove('glow')), 4200); }
}));

/* ---------- surprise me ---------- */
const PLACES = [['DB Simulator','/hacks/#db'],['a note on B-trees vs LSM trees','/notes/databases/b-trees-vs-lsm/'],['HTML to pixel','/blog/2025-07-02-from-html-to-pixel/'],['Database Internals','/library/'],['the Void','/enter-the-void/'],['a page that isn’t there','/this-page-fell-off-the-shelf/']];
const surprise = () => { const [n, h] = PLACES[Math.floor(Math.random()*PLACES.length)]; toast(`Taking you to <b style="color:inherit">${n}</b>…`, 1400); setTimeout(() => location.href = h, reduced() ? 0 : 900); };
$$('[data-surprise]').forEach(b => b.addEventListener('click', surprise));

/* ---------- keyboard ---------- */
addEventListener('keydown', e => {
  if (e.target.closest('input,textarea') ) { if (e.key === 'Escape') e.target.blur(); return; }
  if ((e.metaKey || e.ctrlKey) && (e.key === 'k' || e.key === 'K')) { const s = $('[data-search]'); if (s) { e.preventDefault(); s.focus(); } return; }
  if (e.key === '/') { const s = $('[data-search]'); if (s) { e.preventDefault(); s.focus(); } }
  else if (e.key === 'r' || e.key === 'R') { if (!body.dataset.nosurprise || $('[data-surprise]')) surprise(); }
  else if (e.key === 't' || e.key === 'T') flip(root.dataset.theme === 'dark' ? 'light' : 'dark');
  else if (e.key === 's' || e.key === 'S') window.__shuffle && window.__shuffle();
  else if (e.key === 'Escape') menu(false);
});

/* ---------- home: the card fan ---------- */
const deck = $('.deck');
if (deck) {
  const cards = $$('.card', deck);
  const SLOTS = [{x:-236,y:28,r:-10,z:1},{x:-120,y:9,r:-4.5,z:2},{x:0,y:-6,r:0,z:5},{x:120,y:9,r:4.5,z:4},{x:236,y:28,r:10,z:3}];
  let order = cards.map((_, i) => i), hover = null, px = 0, stacked = true;
  const scale = () => innerWidth < 640 ? .64 : innerWidth < 900 ? .86 : 1;
  const layout = () => {
    const k = scale();
    cards.forEach((c, ci) => {
      const s = order.indexOf(ci), S = SLOTS[s];
      let x = S.x, y = S.y, r = S.r, z = S.z, lift = 0;
      if (stacked) { x = 0; y = 34; r = (s - 2) * 1.6; z = s; }
      else if (hover !== null) {
        x = S.x * 1.07 + px * 14 * (s - 2) / 2; r = S.r + px * 2.2;
        if (hover === ci) { lift = -24; r = S.r * .3; z = 9; }
      }
      c.style.zIndex = z;
      c.style.transform = `translate(${x*k}px,${(y + lift)*k}px) rotate(${r}deg) scale(${k})`;
    });
  };
  cards.forEach((c, ci) => {
    c.style.transitionDelay = (ci * 45) + 'ms';
    c.addEventListener('pointerenter', () => { hover = ci; cards.forEach(x => x.style.transitionDelay = '0ms'); layout(); });
    c.addEventListener('pointermove', e => { const r = c.getBoundingClientRect(); c.style.setProperty('--gx', (e.clientX - r.left)/r.width*100 + '%'); c.style.setProperty('--gy', (e.clientY - r.top)/r.height*100 + '%'); });
  });
  deck.addEventListener('pointermove', e => { if (reduced()) return; const r = deck.getBoundingClientRect(); px = ((e.clientX - r.left)/r.width - .5) * 2; layout(); });
  deck.addEventListener('pointerleave', () => { hover = null; px = 0; layout(); });
  const shuffle = () => {
    if (reduced()) { order.unshift(order.pop()); layout(); return; }
    stacked = true; hover = null; cards.forEach(x => x.style.transitionDelay = '0ms'); layout();
    setTimeout(() => { for (let i = order.length - 1; i > 0; i--) { const j = Math.floor(Math.random()*(i+1)); [order[i], order[j]] = [order[j], order[i]]; }
      stacked = false; cards.forEach((x, ci) => x.style.transitionDelay = (order.indexOf(ci) * 55) + 'ms'); layout(); }, 420);
  };
  window.__shuffle = shuffle;
  $$('[data-shuffle]').forEach(b => b.addEventListener('click', shuffle));
  layout();
  setTimeout(() => { stacked = false; layout(); setTimeout(() => cards.forEach(x => x.style.transitionDelay = '0ms'), 900); }, reduced() ? 0 : 280);
  addEventListener('resize', layout);
}

/* ---------- search filter (home & lists) ---------- */
const sIn = $('[data-search]');
if (sIn) sIn.addEventListener('input', () => {
  const q = sIn.value.trim().toLowerCase(); let any = false;
  $$('[data-s]').forEach(el => { const hit = !q || el.dataset.s.toLowerCase().includes(q) || el.textContent.toLowerCase().includes(q); el.hidden = !hit; any = any || hit; });
  $$('[data-sec]').forEach(sec => sec.hidden = q && !$$('[data-s]', sec).some(x => !x.hidden));
  const em = $('.empty'); if (em) { em.classList.toggle('show', !any); const b = em.querySelector('b'); if (b) b.textContent = sIn.value; }
});

/* ---------- shared-element morph ---------- */
const morph = (fn, scope) => {
  if (!d.startViewTransition || reduced()) return fn();
  const els = $$('[data-cat], .book', scope); els.forEach((el, i) => el.style.viewTransitionName = 'm' + i);
  d.startViewTransition(fn).finished.finally(() => els.forEach(el => el.style.viewTransitionName = ''));
};
window.__morph = morph;

/* ---------- category filters ---------- */
$$('[data-filter]').forEach(ctrl => ctrl.addEventListener('pick', e => {
  const v = e.detail, scope = $(ctrl.dataset.filter);
  if (!scope) return;
  const run = () => $$('[data-cat]', scope).forEach(el => el.hidden = !(v === 'all' || (el.dataset.cat || '').split(' ').includes(v)));
  morph(run, scope);
}));

$$('[data-palette]').forEach(b => b.addEventListener('click', ev => {
  ev.preventDefault();
  const s = $('[data-search]');
  if (s) { s.focus(); const box = s.closest('.search') || s; box.scrollIntoView({ behavior: reduced() ? 'auto' : 'smooth', block: 'center' }); }
  else toast('Press / on a page with search');
}));

const homeSeg = $('#homeseg');
if (homeSeg) homeSeg.addEventListener('pick', e => { const v = e.detail; $$('[data-sec]').forEach(s => { if (s.closest('#homeseg')) return; s.hidden = !(v === 'all' || s.dataset.sec === v); }); });

const lib = $('#lib');
if (lib) $$('#vs button').forEach(b => b.addEventListener('click', () => { const go = () => { $$('#vs button').forEach(x => x.setAttribute('aria-pressed', x === b)); lib.classList.toggle('spines', b.dataset.v === 'spines'); }; morph(go, lib); }));

/* ---------- note: write-ratio slider ---------- */
const w = $('#w');
if (w) {
  const up = () => {
    const x = w.value / 100;
    const bCost = (.9 * x + 1 * (1 - x)), lCost = (.32 * x + 1.7 * (1 - x));
    const m = Math.max(bCost, lCost);
    $('#bb').style.width = (bCost / m * 100) + '%';
    $('#lb').style.width = (lCost / m * 100) + '%';
    $('#bt').textContent = bCost.toFixed(2) + '×';
    $('#lt').textContent = lCost.toFixed(2) + '×';
    $('#wv').textContent = w.value + '% writes';
    $('#vd').textContent = Math.abs(bCost - lCost) < .08 ? 'A coin toss — pick by operations.' : bCost < lCost ? 'The B-tree wins: reads dominate.' : 'The LSM tree wins: writes dominate.';
  };
  w.addEventListener('input', up); w.value = 72; up();
}

/* ---------- post: HTML → pixel stages ---------- */
const stg = $('#stg');
if (stg) {
  const caps = ['<b>Parse.</b> A tokenizer turns the byte stream into tags; a tree builder nests them into the DOM.','<b>Style.</b> Every element gets a computed style: cascade, inheritance and specificity resolved.','<b>Layout.</b> Boxes get positions and sizes. The expensive, recursive bit.','<b>Paint.</b> Drawing commands are recorded into a display list.','<b>Composite.</b> Layers are rasterised and stacked on the GPU — scrolling and transforms happen here.'];
  const bs = [...stg.querySelectorAll('button')], ps = [...$('#stage').querySelectorAll(':scope > div')];
  const go = i => {
    bs.forEach((b, j) => b.setAttribute('aria-pressed', String(j === i)));
    ps.forEach((p, j) => p.classList.toggle('on', j === i));
    const capt = $('#capt'); if (capt) capt.innerHTML = caps[i];
    const count = $('.explore .cap .mono'); if (count) count.textContent = (i + 1) + ' / 5';
    if (i === 4) $$('.layers span').forEach((s, k) => { s.style.transform = `translate(${(k - 1) * 40}px,${(k - 1) * -6}px) rotate(${(k - 1) * -2}deg)`; });
  };
  bs.forEach((b, i) => b.addEventListener('click', () => go(i)));
  const pr = $('#prog');
  if (pr) addEventListener('scroll', () => { const h = root; const max = h.scrollHeight - h.clientHeight; pr.style.width = (max > 0 ? h.scrollTop / max * 100 : 0) + '%'; }, { passive: true });
  const cp = $('#cp');
  if (cp) cp.addEventListener('click', () => {
    const done = () => { cp.textContent = 'Copied ✓'; setTimeout(() => { cp.textContent = 'Copy link'; }, 1600); };
    if (navigator.clipboard) navigator.clipboard.writeText(location.href).then(done, done); else done();
  });
}

/* ---------- book reader: turn pages ---------- */
const reader = $('#reader');
if (reader) {
  const pn = $('#pn'), sp = $('#spread');
  let p = Number(reader.dataset.start || 1);
  const max = Number(reader.dataset.pages || 1);
  const turn = dir => {
    p = Math.max(1, Math.min(max, p + dir));
    if (pn) pn.textContent = String(p);
    if (!sp) return;
    sp.classList.remove('turn'); void sp.offsetWidth; if (!reduced()) sp.classList.add('turn');
  };
  const prev = $('#prev'), next = $('#next');
  if (prev) prev.addEventListener('click', () => turn(-2));
  if (next) next.addEventListener('click', () => turn(2));
  addEventListener('keydown', e => { if (e.target.closest('input,textarea')) return; if (e.key === 'ArrowRight') turn(2); if (e.key === 'ArrowLeft') turn(-2); });
  const cite = $('#cite');
  if (cite) cite.addEventListener('click', () => {
    const text = reader.dataset.cite || document.title;
    const done = () => { cite.textContent = 'Copied ✓'; };
    if (navigator.clipboard) navigator.clipboard.writeText(text).then(done, done); else done();
  });
}

/* ---------- 404: scattered cards tidy into a fan ---------- */
const pile = $('#pile');
if (pile) {
  const cs = [...pile.querySelectorAll('.c')]; let tidy = false;
  const mess = () => cs.forEach(c => { c.style.transform = `translate(${(Math.random() - .5) * 280}px,${(Math.random() - .3) * 120}px) rotate(${(Math.random() - .5) * 70}deg)`; });
  const fan = () => cs.forEach((c, i) => { c.style.transitionDelay = i * 50 + 'ms'; c.style.transform = `translate(${(i - 2) * 62}px,${Math.abs(i - 2) * 10}px) rotate(${(i - 2) * 7}deg)`; });
  mess();
  const toggle = () => { tidy = !tidy; if (tidy) fan(); else { cs.forEach(c => { c.style.transitionDelay = '0ms'; }); mess(); } };
  pile.addEventListener('click', toggle);
  pile.addEventListener('keydown', e => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); toggle(); } });
}

/* ---------- the void: glow, and the existing passphrase check ---------- */
const door = $('#door');
const pw = $('#pw');
if (door && pw) {
  const glow = $('#glow'), rings = $('#rings');
  addEventListener('pointermove', e => {
    if (reduced()) return;
    const x = (e.clientX / innerWidth - .5), y = (e.clientY / innerHeight - .5);
    if (glow) glow.style.transform = `translate(${x * 120}px,${y * 120}px)`;
    if (rings) rings.style.transform = `translate(${x * -20}px,${y * -20}px)`;
  });
  let spin = 0;
  const rn = $('#rn');
  if (rn) rn.addEventListener('click', e => { spin += 180; e.currentTarget.style.transform = `rotate(${spin}deg)`; });
}

$$('[data-empty-pdf]').forEach(b => b.addEventListener('click', e => { e.preventDefault(); toast('The résumé PDF isn’t linked yet.'); }));

/* ---------- darkroom: type meow, or the small paw ---------- */
const darkroom = $('#darkroom');
const openDarkroom = () => { if (!darkroom) return; darkroom.hidden = false; };
const closeDarkroom = () => { if (darkroom) darkroom.hidden = true; };
$$('[data-paw]').forEach(b => b.addEventListener('click', openDarkroom));
$$('[data-darkroom-close]').forEach(b => b.addEventListener('click', closeDarkroom));
let heard = '';
addEventListener('keydown', e => {
  if (e.key === 'Escape') { closeDarkroom(); return; }
  const typing = e.target && (e.target.closest('input, textarea, [contenteditable="true"]'));
  if (typing || e.metaKey || e.ctrlKey || e.altKey || e.key.length !== 1) return;
  heard = (heard + e.key.toLowerCase()).slice(-4);
  if (heard === 'meow') { heard = ''; openDarkroom(); }
});
if (darkroom && !reduced()) {
  $$('.pol', darkroom).forEach(card => {
    let drag = null;
    card.addEventListener('pointerdown', ev => {
      if (ev.button !== 0) return;
      card.setPointerCapture(ev.pointerId);
      drag = { x: ev.clientX, y: ev.clientY, ox: 0, oy: 0 };
    });
    card.addEventListener('pointermove', ev => {
      const r = card.getBoundingClientRect();
      const px = ((drag ? drag.x : ev.clientX) - r.left) / r.width - .5;
      if (drag && ev.buttons) {
        drag.ox = ev.clientX - drag.x;
        drag.oy = ev.clientY - drag.y;
      }
      const ox = drag ? drag.ox : 0;
      const oy = drag ? drag.oy : 0;
      card.style.transform = `translate(${ox}px, ${oy}px) rotate(${px * 8 - 2}deg)`;
    });
    const release = () => { drag = null; card.style.transform = ''; };
    card.addEventListener('pointerup', release);
    card.addEventListener('pointercancel', release);
  });
}
})();
