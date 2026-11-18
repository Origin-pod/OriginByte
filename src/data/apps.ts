import { site } from './site';

export type AppGroup = 'here' | 'ou' | 'sq';

export interface LabApp {
  title: string;
  /** TODO(owner): these one-line blurbs are the mockup’s placeholder copy. */
  blurb: string;
  blurbPlaceholder: true;
  href: string;
  external?: boolean;
  group: AppGroup;
  swatch: 'verm' | 'sand' | 'moss' | 'blue' | 'black' | 'lilac' | 'cream' | 'rose';
  icon: string;
}

const I = {
  brain: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><circle cx="5" cy="6" r="2.2"/><circle cx="14" cy="5" r="2.2"/><circle cx="10" cy="14" r="2.6"/><path d="M7 6.5l5-1M6 8l3 4M13 7l-2 5"/></svg>',
  ide: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><rect x="2.5" y="3.5" width="15" height="13" rx="2"/><path d="M6 9l2.5 2L6 13M10.5 13H14"/></svg>',
  lines: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M4 5h12M4 9h8M4 13h10M4 17h5"/></svg>',
  shield: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M10 2.5l6 3v4c0 4-2.6 6.6-6 8-3.4-1.4-6-4-6-8v-4z"/><path d="M7.5 10l1.8 1.8L13 8"/></svg>',
  art: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M3 15c3-8 6 2 9-5s4-4 5-6"/><circle cx="5" cy="5" r="1.5"/></svg>',
  nodes: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><rect x="3" y="3" width="5" height="5" rx="1"/><rect x="12" y="3" width="5" height="5" rx="1"/><rect x="7.5" y="12" width="5" height="5" rx="1"/><path d="M5.5 8v2h9V8M10 10v2"/></svg>',
  chart: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><rect x="2.5" y="2.5" width="15" height="15" rx="2"/><path d="M5 13c2.5-6 5 1 10-6"/></svg>',
  wave: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M3 12c2 3 5 3 7 0s5-3 7 0"/><circle cx="14" cy="6" r="1.2" fill="currentColor"/></svg>',
  bloom: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><circle cx="10" cy="10" r="2"/><circle cx="10" cy="5" r="2.4"/><circle cx="14.6" cy="8.6" r="2.4"/><circle cx="12.8" cy="14" r="2.4"/><circle cx="7.2" cy="14" r="2.4"/><circle cx="5.4" cy="8.6" r="2.4"/></svg>',
  plus: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M3 10h14M10 6v8"/><circle cx="10" cy="10" r="7"/></svg>',
  dash: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4" stroke-dasharray="2 2"><rect x="3" y="3" width="14" height="14" rx="3"/></svg>',
  bookmark: '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.4"><path d="M6 3h8v14l-4-3-4 3z"/></svg>',
};

/** Apps that run on this site, then apps that live in other repos. */
export const apps: LabApp[] = [
  {
    title: 'DB Simulator',
    blurb: 'A visual, modular database builder. Snap engine blocks together and watch the trade-offs.',
    blurbPlaceholder: true,
    href: site.dbSim,
    group: 'here',
    swatch: 'moss',
    icon: I.nodes,
  },
  {
    title: 'OriginBrain',
    blurb: 'A second brain of linked notes.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginBrain',
    external: true,
    group: 'ou',
    swatch: 'lilac',
    icon: I.brain,
  },
  {
    title: 'OriginCodeIDE',
    blurb: 'A small IDE in a browser tab.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginCodeIDE',
    external: true,
    group: 'ou',
    swatch: 'black',
    icon: I.ide,
  },
  {
    title: 'OriginSublime',
    blurb: 'An editor, rebuilt for fun.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginSublime',
    external: true,
    group: 'ou',
    swatch: 'verm',
    icon: I.lines,
  },
  {
    title: 'OriginSteward',
    blurb: 'Keeps the other apps tidy.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginSteward',
    external: true,
    group: 'ou',
    swatch: 'sand',
    icon: I.shield,
  },
  {
    title: 'OriginArt',
    blurb: 'Generative sketches.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginArt',
    external: true,
    group: 'ou',
    swatch: 'rose',
    icon: I.art,
  },
  {
    title: 'OriginSystems',
    blurb: 'Systems experiments, assembled.',
    blurbPlaceholder: true,
    href: 'https://github.com/Origin-pod/OriginSystems',
    external: true,
    group: 'ou',
    swatch: 'blue',
    icon: I.nodes,
  },
  {
    title: 'CanvasGPT',
    blurb: 'Sketch on a canvas, ask a model.',
    blurbPlaceholder: true,
    href: 'https://github.com/happycoder0011/CanvasGPT',
    external: true,
    group: 'sq',
    swatch: 'cream',
    icon: I.chart,
  },
  {
    title: 'Parseltongue',
    blurb: 'A tiny language that hisses.',
    blurbPlaceholder: true,
    href: 'https://github.com/that-in-rust/parseltongue',
    external: true,
    group: 'sq',
    swatch: 'moss',
    icon: I.wave,
  },
  {
    title: 'Blr-CherryBlossom',
    blurb: 'Bengaluru in pink season.',
    blurbPlaceholder: true,
    href: 'https://github.com/happycoder0011/Blr-CherryBlossom',
    external: true,
    group: 'sq',
    swatch: 'rose',
    icon: I.bloom,
  },
  {
    title: 'RSVP-Reader',
    blurb: 'Read one word at a time, fast.',
    blurbPlaceholder: true,
    href: 'https://github.com/happycoder0011/RSVP-Reader',
    external: true,
    group: 'sq',
    swatch: 'black',
    icon: I.plus,
  },
  {
    title: 'Vanish',
    blurb: 'Messages that disappear.',
    blurbPlaceholder: true,
    href: 'https://github.com/happycoder0011/Vanish',
    external: true,
    group: 'sq',
    swatch: 'lilac',
    icon: I.dash,
  },
  {
    title: 'Twitter Bookmarks 2.0',
    blurb: 'Bookmarks you can actually find.',
    blurbPlaceholder: true,
    href: 'https://github.com/happycoder0011/Twitter-Bookmark-2.0',
    external: true,
    group: 'sq',
    swatch: 'blue',
    icon: I.bookmark,
  },
];

export const appGroups: { id: AppGroup; title: string; hint: string }[] = [
  { id: 'here', title: 'Runs on this site', hint: '/dbsim' },
  { id: 'ou', title: 'Origin Universe', hint: 'own repos' },
  { id: 'sq', title: 'Side quests', hint: 'own repos' },
];
