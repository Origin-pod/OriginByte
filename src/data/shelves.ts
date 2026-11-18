/** Shelf copy for the notes index. Blurbs match the approved mockup and are still placeholders. */

export interface ShelfMeta {
  id: string;
  title: string;
  group: 'web' | 'sys' | 'sci' | 'ref';
  blurb: string;
  /** TODO(owner): replace the mockup blurbs, and the per-shelf counts are computed live. */
  blurbPlaceholder: boolean;
  private?: boolean;
  icon: string;
}

const svg = (inner: string) =>
  `<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3">${inner}</svg>`;

export const shelfMeta: Record<string, ShelfMeta> = {
  frontend: {
    id: 'frontend',
    title: 'Frontend',
    group: 'web',
    blurb: 'Rendering models, state, and the architecture of big UIs.',
    blurbPlaceholder: true,
    icon: svg('<rect x="2" y="3" width="12" height="10" rx="1.5"/><path d="M2 6h12"/>'),
  },
  browser: {
    id: 'browser',
    title: 'Browser',
    group: 'web',
    blurb: 'From bytes to pixels: parsing, style, layout, paint, composite.',
    blurbPlaceholder: true,
    icon: svg('<circle cx="8" cy="8" r="6"/><path d="M2 8h12M8 2c2 2 2 10 0 12M8 2c-2 2-2 10 0 12"/>'),
  },
  backend: {
    id: 'backend',
    title: 'Backend',
    group: 'web',
    blurb: 'APIs, queues, caches and the shapes of distributed systems.',
    blurbPlaceholder: true,
    icon: svg('<rect x="2.5" y="2.5" width="11" height="4" rx="1"/><rect x="2.5" y="9.5" width="11" height="4" rx="1"/>'),
  },
  databases: {
    id: 'databases',
    title: 'Databases',
    group: 'sys',
    blurb: 'Storage engines, transactions, recovery — the DB Simulator’s homework.',
    blurbPlaceholder: true,
    icon: svg('<ellipse cx="8" cy="4" rx="5.5" ry="2"/><path d="M2.5 4v8c0 1.1 2.5 2 5.5 2s5.5-.9 5.5-2V4M2.5 8c0 1.1 2.5 2 5.5 2s5.5-.9 5.5-2"/>'),
  },
  'system-programming': {
    id: 'system-programming',
    title: 'Systems programming',
    group: 'sys',
    blurb: 'Rust, memory, and the cost of abstractions.',
    blurbPlaceholder: true,
    icon: svg('<path d="M4 5l3 3-3 3M8.5 11H12"/>'),
  },
  os: {
    id: 'os',
    title: 'Operating systems',
    group: 'sys',
    blurb: 'Processes, memory, scheduling and file systems.',
    blurbPlaceholder: true,
    icon: svg('<rect x="3" y="3" width="10" height="10" rx="1.5"/><path d="M6 1v2M10 1v2M6 13v2M10 13v2M1 6h2M1 10h2M13 6h2M13 10h2"/>'),
  },
  'theoretical-physics': {
    id: 'theoretical-physics',
    title: 'Physics',
    group: 'sci',
    blurb: 'The maths under game engines and simulations.',
    blurbPlaceholder: true,
    private: true,
    icon: svg('<circle cx="8" cy="8" r="1.5"/><ellipse cx="8" cy="8" rx="6.5" ry="2.5"/><ellipse cx="8" cy="8" rx="6.5" ry="2.5" transform="rotate(60 8 8)"/><ellipse cx="8" cy="8" rx="6.5" ry="2.5" transform="rotate(120 8 8)"/>'),
  },
  'quantum-computing': {
    id: 'quantum-computing',
    title: 'Quantum computing',
    group: 'sci',
    blurb: 'Qubits, gates, and the machines that might run them.',
    blurbPlaceholder: true,
    private: true,
    icon: svg('<circle cx="8" cy="8" r="1.5"/><ellipse cx="8" cy="8" rx="6.5" ry="2.5"/>'),
  },
  handbooks: {
    id: 'handbooks',
    title: 'Handbooks',
    group: 'ref',
    blurb: 'Checklists and field guides I reach for at work.',
    blurbPlaceholder: true,
    private: true,
    icon: svg('<path d="M3 2.5h8a2 2 0 0 1 2 2v9H5a2 2 0 0 1-2-2z"/><path d="M3 11.5a2 2 0 0 1 2-2h8"/>'),
  },
  'dev-tooling': {
    id: 'dev-tooling',
    title: 'Dev tooling',
    group: 'web',
    blurb: 'The small tools that sit under the editor.',
    blurbPlaceholder: true,
    icon: svg('<path d="M3 8h10M8 3v10"/><circle cx="8" cy="8" r="6"/>'),
  },
  'conference-talks': {
    id: 'conference-talks',
    title: 'Conference talks',
    group: 'ref',
    blurb: 'Notes from talks worth keeping.',
    blurbPlaceholder: true,
    icon: svg('<rect x="2" y="3" width="12" height="8" rx="1"/><path d="M6 11v2h4v-2"/>'),
  },
  'research-papers': {
    id: 'research-papers',
    title: 'Research papers',
    group: 'ref',
    blurb: 'Papers read slowly, filed by the idea they left behind.',
    blurbPlaceholder: true,
    icon: svg('<path d="M4 2.5h6l3 3V13.5H4z"/><path d="M10 2.5V6h3"/>'),
  },
  'thinking-patterns': {
    id: 'thinking-patterns',
    title: 'Thinking patterns',
    group: 'ref',
    blurb: 'Ways of cutting a problem so it stays cut.',
    blurbPlaceholder: true,
    icon: svg('<path d="M2 8h12M8 2v12"/>'),
  },
  'learning-resources': {
    id: 'learning-resources',
    title: 'Learning resources',
    group: 'ref',
    blurb: 'Courses, book notes, and the to-read pile.',
    blurbPlaceholder: true,
    icon: svg('<path d="M3 3h10v10H3z"/><path d="M6 6h4M6 8.5h4"/>'),
  },
  projects: {
    id: 'projects',
    title: 'Projects',
    group: 'web',
    blurb: 'Idea dumps and the starts of tools.',
    blurbPlaceholder: true,
    icon: svg('<rect x="2" y="2" width="5" height="5"/><rect x="9" y="9" width="5" height="5"/>'),
  },
  'random-explorations': {
    id: 'random-explorations',
    title: 'Random explorations',
    group: 'ref',
    blurb: 'Side paths that have not earned a shelf of their own.',
    blurbPlaceholder: true,
    icon: svg('<circle cx="8" cy="8" r="2"/><path d="M8 2v2M8 12v2M2 8h2M12 8h2"/>'),
  },
};

export const shelfOrder = [
  'frontend',
  'browser',
  'backend',
  'databases',
  'system-programming',
  'os',
  'theoretical-physics',
  'quantum-computing',
  'handbooks',
  'dev-tooling',
  'conference-talks',
  'research-papers',
  'thinking-patterns',
  'learning-resources',
  'projects',
  'random-explorations',
];

/** The eight doors on the home page, in mockup order. */
export const homeShelves = [
  'frontend',
  'browser',
  'backend',
  'databases',
  'system-programming',
  'os',
  'theoretical-physics',
  'handbooks',
];

export const swatches = ['blue', 'verm', 'sand', 'moss', 'black', 'lilac', 'cream', 'rose'] as const;
