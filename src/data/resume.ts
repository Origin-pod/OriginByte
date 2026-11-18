/**
 * Résumé content for /work/. Facts are taken from the owner's résumé.
 * The phone number is intentionally not stored anywhere in this repo.
 */

export type Bullet = string | { text: string; children: string[] };

export interface Job {
  id: string;
  when: string[];
  current?: boolean;
  company: string;
  role: string;
  bullets: Bullet[];
  stack: string[];
}

export interface TimelineSeg {
  href: string;
  label: string;
  left: string;
  width: string;
  kind?: 'edu' | 'now';
}

export const timeline: TimelineSeg[] = [
  { href: '#edu', label: 'IIIT Bhopal', left: '6%', width: '43%', kind: 'edu' },
  { href: '#tekion', label: 'Tekion', left: '50%', width: '33%' },
  { href: '#glean', label: 'Glean', left: '84%', width: '8.5%' },
  { href: '#harness', label: 'H', left: '93%', width: '7%', kind: 'now' },
];

export const jobs: Job[] = [
  {
    id: 'harness',
    when: ['Dec 2025', '● present'],
    current: true,
    company: 'Harness',
    role: 'Senior Software Engineer',
    bullets: [
      'Working on products for <b>Application and API Security</b> across the Discovery and Protection modules.',
      'Built features such as <b>Agent Discovery</b>, <b>Bot Policy</b> configuration and monitoring, and <b>Entity Derivation</b>.',
      'Built user interfaces in Angular 22, D3.js, Material UI, GraphQL and Canvas.',
    ],
    stack: ['Angular', 'D3.js', 'Material UI', 'GraphQL', 'Canvas'],
  },
  {
    id: 'glean',
    when: ['Oct 2024', 'Nov 2025'],
    company: 'Glean',
    role: 'Software Engineer',
    bullets: [
      {
        text: 'Designed and implemented end-to-end features for the Agent Builder platform:',
        children: [
          '<b>Featured Agents</b> — curation and showcasing of high-impact agents for better discoverability.',
          '<b>Prompt support in App Builder</b> — flexible prompt management to enhance agent usage.',
          "<b>LLM Model Hub</b> — admins configure hosts and providers for Glean's customer-facing services, trading off quality and cost.",
          '<b>Conversational agent triggers</b> — the UI for triggers that start agent workflows.',
          '<b>Slack integration</b> — agent triggering and permissions inside Slack.',
        ],
      },
      'Built and enhanced interfaces with React, Redux Toolkit, TanStack Query and Ant Design.',
    ],
    stack: ['React', 'Redux Toolkit', 'TanStack Query', 'Ant Design'],
  },
  {
    id: 'tekion',
    when: ['Jan 2022', 'Sep 2024'],
    company: 'Tekion Corp',
    role: 'Software Engineer',
    bullets: [
      'Built UI features for the consumer e-commerce experience for OEMs, with <b>300k+ monthly users</b>.',
      'Built an in-house state-management and data-fetching library with abstracted business logic, cutting feature effort by <b>33%</b> (3 months → 1).',
      'Refactored feature components into shared platform components with MBDS, Context API and custom hooks.',
      'Migrated from a composition-based architecture to a <b>builder pattern</b> to solve scaling issues.',
      'Accessibility, Next config for routing, i18n and web components.',
      'Wrote a bash scaffolding script for components used by a team of <b>40 developers</b>, plus Slack deploy notifications via webhooks.',
      'Built an internal EC2 dashboard with the AWS SDK for a hackathon.',
      'Mentored interns and worked closely with PMs, backend and support.',
    ],
    stack: ['Next.js', 'React', 'Redux Toolkit', 'Storybook', 'Node.js', 'Bash', 'Docker', 'TailwindCSS'],
  },
];

export const skills: { term: string; lead: string; rest?: string }[] = [
  { term: 'languages', lead: 'TypeScript, JavaScript, Rust', rest: 'Bash' },
  {
    term: 'frontend',
    lead: 'React, Next.js, Angular, Redux Toolkit, TanStack Query',
    rest: 'D3.js, Canvas, Web Components, Storybook, Material UI, Ant Design, TailwindCSS',
  },
  { term: 'platform', lead: 'Node.js, GraphQL, WebAssembly', rest: 'Docker, AWS SDK, CI automation' },
  {
    term: 'practice',
    lead: 'Frontend architecture, design systems, accessibility, i18n',
    rest: 'developer tooling, mentoring',
  },
];

export const project = {
  when: 'Feb 2026',
  stack: 'Rust · TypeScript · React · WebAssembly',
  title: 'DB Simulator',
  summary:
    'A visual, modular database builder: compose engine blocks — storage engines (LSM trees, B-trees), concurrency control (MVCC, 2PL), recovery (WAL, ARIES) and indexing — and learn the internals through real-time experiments and trade-off visualisations.',
};

export const education = {
  when: '2018 — 2022',
  school: 'Indian Institute of Information Technology, Bhopal',
  detail: 'B.Tech, Information Technology',
  cgpa: '9.35',
};
