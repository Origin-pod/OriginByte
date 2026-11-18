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
