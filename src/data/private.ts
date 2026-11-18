/** Shelves that stay off the public site. The Void password is a Netlify env var, not this file. */
export const PRIVATE_SHELVES = ['theoretical-physics', 'quantum-computing', 'handbooks'] as const;

export function isPrivateShelf(id: string): boolean {
  return (PRIVATE_SHELVES as readonly string[]).includes(id);
}

export function isPrivateDocId(id: string): boolean {
  const clean = id.replace(/\.mdx?$/, '');
  return PRIVATE_SHELVES.some((shelf) => clean === `notes/${shelf}` || clean.startsWith(`notes/${shelf}/`));
}
