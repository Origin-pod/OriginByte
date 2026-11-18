/** Display metadata for the writing index. Dates and read times on the page come from the posts themselves. */

export interface PostMeta {
  /** Short label used in lists. TODO(owner): confirm these display titles. */
  short: string;
  cat: 'rendering' | 'frameworks' | 'engines' | 'meta';
  /** One-line dek for the index. TODO(owner): replace with a hand-written dek. */
  dek: string;
  dekPlaceholder: true;
  hot?: boolean;
  featured?: boolean;
}

export const writingMeta: Record<string, PostMeta> = {
  '2025-07-02-from-html-to-pixel': {
    short: 'HTML to pixel',
    cat: 'rendering',
    dek: 'What the browser does between receiving bytes and painting the frame you are looking at.',
    dekPlaceholder: true,
    hot: true,
    featured: true,
  },
  '2025-07-03-from-html-to-pixel-without-gpu': {
    short: 'HTML to pixel, without a GPU',
    cat: 'rendering',
    dek: 'The same pipeline when compositing is not handed to the GPU.',
    dekPlaceholder: true,
  },
  '2025-07-06-cpu-parallel-rendering': {
    short: 'CPU parallel rendering',
    cat: 'rendering',
    dek: 'One frame, many cores.',
    dekPlaceholder: true,
  },
  '2025-05-23-ui-framework-paradigms': {
    short: 'UI framework paradigms',
    cat: 'frameworks',
    dek: 'Retained, immediate, reactive.',
    dekPlaceholder: true,
  },
  '2025-05-23-back-of-the-envelope-calculation-for-fe': {
    short: 'Back-of-the-envelope for the frontend',
    cat: 'frameworks',
    dek: 'A performance guide for the cost of UI work.',
    dekPlaceholder: true,
  },
  '2025-07-04-game-engines-101': {
    short: 'Game engines 101',
    cat: 'engines',
    dek: 'Loops, entities and the fixed timestep.',
    dekPlaceholder: true,
  },
  '2025-07-05-react-game-engine-dsl': {
    short: 'A React-like game engine DSL',
    cat: 'engines',
    dek: 'A React-shaped API aimed at game-engine performance.',
    dekPlaceholder: true,
  },
  '2025-10-21-hosting-domain-deploy': {
    short: 'From subdomain to a real domain',
    cat: 'meta',
    dek: 'Moving Origin Byte off the Netlify subdomain and onto a custom domain.',
    dekPlaceholder: true,
  },
};

export function estimateMinutes(markdown: string): number {
  const words = markdown.trim().split(/\s+/).filter(Boolean).length;
  return Math.max(1, Math.round(words / 220));
}
