/**
 * Shared site facts. The phone number from the résumé is intentionally absent —
 * do not add it here, in pages, or in commit messages.
 */
export const site = {
  name: 'Origin Byte',
  author: 'Sheetal Singh',
  email: 'sheetalsinghssr@gmail.com',
  city: 'Bengaluru',
  github: 'https://github.com/Origin-pod',
  x: 'https://x.com/sheetalssr',
  linkedin: 'https://www.linkedin.com/in/sheetal-singh-218855176',
  /** Canonical in-site URL. Netlify proxies this at public/_redirects. */
  dbSim: '/dbsim/',
  dbSimSource: 'https://github.com/Origin-pod/Origin-DB-Simulator',
  /**
   * TODO(owner): set PUBLIC_RESUME_PDF_URL in the Netlify environment
   * (or replace this) with the object-storage URL of the résumé PDF.
   * The PDF itself is not in git.
   */
  resumePdfUrl: (import.meta.env.PUBLIC_RESUME_PDF_URL ?? '') as string,
  /**
   * TODO(owner): the “now reading” line is still the mockup placeholder.
   */
  nowReading: {
    title: 'Database Internals',
    detail: 'ch. 7',
    placeholder: true,
  },
  /**
   * SHA-256 of the Void passphrase (trimmed, lowercased).
   * TODO(owner): replace this hash to change the word.
   * The check is client-side only and is not real privacy.
   */
  voidHash: 'c7b9bdd40758eaa827b7abc1fc31408901ac99cef4f3891c1fb3598f531bb9e6',
} as const;

export const nav = [
  { key: 'home', label: 'Index', href: '/', num: '00' },
  { key: 'work', label: 'Work', href: '/work/', num: '01' },
  { key: 'lab', label: 'Lab', href: '/hacks/', num: '02' },
  { key: 'notes', label: 'Notes', href: '/notes/', num: '03' },
  { key: 'writing', label: 'Writing', href: '/blog/', num: '04' },
  { key: 'library', label: 'Library', href: '/library/', num: '05' },
] as const;

export function navKey(page: string): string {
  if (page === 'note') return 'notes';
  if (page === 'post') return 'writing';
  if (page === 'book') return 'library';
  return page;
}
