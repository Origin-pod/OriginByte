import { isUnlocked, lock, privatePrefix, isPrivatePath } from './auth';

const RUNE_TARGET = '/enter-the-void/';

function hidePrivateSidebar(): void {
  const links = document.querySelectorAll<HTMLAnchorElement>('a[href]');
  const seen = new Set<HTMLElement>();
  links.forEach((a) => {
    const slug = privatePrefix(new URL(a.href, location.origin).pathname);
    if (!slug) return;
    const group = a.closest('details, li, .sidebar-group, .sidebar-entry');
    if (group && !seen.has(group as HTMLElement)) {
      seen.add(group as HTMLElement);
      (group as HTMLElement).classList.add('ssr-gated');
    }
  });
}

function injectNoIndex(): void {
  if (document.querySelector('meta[name="robots"][content*="noindex"]')) return;
  const meta = document.createElement('meta');
  meta.name = 'robots';
  meta.content = 'noindex, nofollow';
  document.head.appendChild(meta);
}

function showLockScreen(): void {
  if (document.querySelector('.ssr-lock-screen')) return;
  const target =
    document.querySelector('article .sl-markdown-content') ||
    document.querySelector('main') ||
    document.body;
  if (!target) return;

  const article = target.closest('article') || target;
  const wrap = document.createElement('div');
  wrap.className = 'ssr-lock-screen';
  wrap.innerHTML = `
    <div class="ssr-lock-screen__inner">
      <div class="ssr-lock-screen__rune">🜲</div>
      <h1 class="ssr-lock-screen__title">sealed</h1>
      <p class="ssr-lock-screen__sub">this section is only visible in dev mode.</p>
      <a class="ssr-lock-screen__cta" href="${RUNE_TARGET}">enter the void &rarr;</a>
    </div>
  `;
  article.prepend(wrap);
}

function hidePrivateContent(): void {
  const content = document.querySelector('article .sl-markdown-content');
  if (content) {
    content.classList.add('ssr-content-hidden');
  }
  const header = document.querySelector('article header');
  if (header) {
    header.classList.add('ssr-content-hidden');
  }
  const footer = document.querySelector('article footer');
  if (footer) {
    footer.classList.add('ssr-content-hidden');
  }
  const pager = document.querySelector('.sl-pagination');
  if (pager) {
    pager.classList.add('ssr-content-hidden');
  }
}

function applyLockState(): void {
  const unlocked = isUnlocked();
  const privatePath = isPrivatePath(location.pathname);

  if (privatePath && !unlocked) {
    document.documentElement.classList.add('ssr-locked');
    showLockScreen();
    hidePrivateContent();
    injectNoIndex();
  } else if (unlocked) {
    document.documentElement.classList.remove('ssr-locked');
    document.documentElement.classList.add('ssr-unlocked');
  } else {
    hidePrivateSidebar();
  }
}

function showDevIndicator(): void {
  if (!isUnlocked()) return;
  const indicator = document.querySelector<HTMLElement>('[data-ssr-lock]');
  if (indicator) {
    indicator.removeAttribute('hidden');
  }
}

function bindRune(): void {
  const rune = document.querySelector<HTMLElement>('[data-ssr-rune]');
  if (!rune) return;
  const handler = (e: Event) => {
    e.preventDefault();
    e.stopPropagation();
    const wasUnlocked = isUnlocked();
    console.log('[ssr:rune] clicked, unlocked=', wasUnlocked);
    rune.animate(
      [{ transform: 'scale(1)' }, { transform: 'scale(1.4)' }, { transform: 'scale(1)' }],
      { duration: 200 }
    );
    if (wasUnlocked) {
      lock();
      console.log('[ssr:rune] locked, reloading…');
      window.location.replace(window.location.href);
    } else {
      window.location.href = RUNE_TARGET;
    }
  };
  rune.addEventListener('click', handler);
  rune.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || e.key === ' ') handler(e);
  });
}

function bindDevIndicator(): void {
  const btn = document.querySelector<HTMLElement>('[data-ssr-lock]');
  if (!btn) return;
  btn.addEventListener('click', (e) => {
    e.preventDefault();
    e.stopPropagation();
    lock();
    window.location.replace(window.location.href);
  });
}

function run(): void {
  applyLockState();
  showDevIndicator();
  bindRune();
  bindDevIndicator();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', run);
} else {
  run();
}
