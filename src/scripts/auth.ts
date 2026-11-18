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

