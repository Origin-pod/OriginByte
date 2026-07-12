const PRIVATE_SECTIONS: readonly string[] = [
  'notes/theoretical-physics',
  'notes/quantum-computing',
  'notes/handbooks',
];

const STORAGE_KEY = 'ssr:devmode';

export function isUnlocked(): boolean {
  try {
    return sessionStorage.getItem(STORAGE_KEY) === '1';
  } catch {
    return false;
  }
}

export function unlock(): void {
  try {
    sessionStorage.setItem(STORAGE_KEY, '1');
  } catch {
    /* sessionStorage unavailable */
  }
}

export function lock(): void {
  try {
    sessionStorage.removeItem(STORAGE_KEY);
  } catch {
    /* sessionStorage unavailable */
  }
}

export function isPrivatePath(pathname: string): boolean {
  return PRIVATE_SECTIONS.some(
    (slug) => pathname === `/${slug}/` || pathname.startsWith(`/${slug}/`)
  );
}

export function privatePrefix(pathname: string): string | null {
  for (const slug of PRIVATE_SECTIONS) {
    if (pathname === `/${slug}/` || pathname.startsWith(`/${slug}/`)) {
      return `/${slug}`;
    }
  }
  return null;
}

async function sha256(text: string): Promise<string> {
  const buf = new TextEncoder().encode(text);
  const hash = await crypto.subtle.digest('SHA-256', buf);
  return Array.from(new Uint8Array(hash))
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
}

export async function checkPassphrase(
  input: string,
  expectedHash: string
): Promise<boolean> {
  const hash = await sha256(input.trim().toLowerCase());
  return hash === expectedHash;
}
