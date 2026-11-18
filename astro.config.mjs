import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import rehypeMermaid from 'rehype-mermaid';

const themeBoot = `(function(){var t=null;try{t=localStorage.getItem('ob-theme')}catch(e){}if(!t)t=matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light';var r=document.documentElement;r.dataset.theme=t;r.classList.add('js');try{localStorage.setItem('starlight-theme',t)}catch(e){}try{if(sessionStorage.getItem('ssr:devmode')==='1')r.classList.add('ssr-unlocked');else r.classList.add('ssr-locked')}catch(e){r.classList.add('ssr-locked')}})();`;

const publicSidebar = [
  { label: 'Frontend', autogenerate: { directory: 'notes/frontend' } },
  { label: 'Backend', autogenerate: { directory: 'notes/backend' } },
  { label: 'System Programming', autogenerate: { directory: 'notes/system-programming' } },
  { label: 'Browser', autogenerate: { directory: 'notes/browser' } },
  { label: 'Databases', autogenerate: { directory: 'notes/databases' } },
  { label: 'OS', autogenerate: { directory: 'notes/os' } },
  { label: 'Random Explorations', autogenerate: { directory: 'notes/random-explorations' } },
  { label: 'Thinking Patterns', autogenerate: { directory: 'notes/thinking-patterns' } },
  {
    label: 'Learning Resources',
    items: [
      { label: 'Book Notes', autogenerate: { directory: 'notes/learning-resources/book-notes' } },
      { label: 'To-Read', autogenerate: { directory: 'notes/learning-resources/to-read' } },
      { label: 'Courses', autogenerate: { directory: 'notes/learning-resources/courses' } },
    ],
  },
  { label: 'Conference Talks', autogenerate: { directory: 'notes/conference-talks' } },
  { label: 'Research Papers', autogenerate: { directory: 'notes/research-papers' } },
  {
    label: 'Projects & Tools',
    items: [
      { label: 'Project Ideas', autogenerate: { directory: 'notes/projects', collapsed: true } },
      { label: 'LuminaJS', autogenerate: { directory: 'notes/projects/luminajs' } },
      { label: 'Prompt Library', autogenerate: { directory: 'notes/projects/prompt-library' } },
      { label: 'HandsFree', autogenerate: { directory: 'notes/projects/handsfree' } },
      { label: 'Hacks', autogenerate: { directory: 'notes/projects/hacks' } },
    ],
  },
  { label: 'Dev Tooling', autogenerate: { directory: 'notes/dev-tooling' } },
];

const privateSidebar = [
  { label: 'Theoretical Physics', autogenerate: { directory: 'notes/theoretical-physics' } },
  { label: 'Quantum Computing', autogenerate: { directory: 'notes/quantum-computing' } },
  { label: 'In-Depth Handbooks', autogenerate: { directory: 'notes/handbooks' } },
];

const sidebar = [...publicSidebar, ...privateSidebar];

export default defineConfig({
  site: 'https://originbyte.netlify.app',
  trailingSlash: 'always',
  markdown: {
    rehypePlugins: [
      [rehypeMermaid, { strategy: 'img-svg', dark: true }],
    ],
  },
  integrations: [
    starlight({
      title: 'Origin Byte',
      favicon: '/favicon.svg',
      disable404Route: true,
      credits: false,
      lastUpdated: false,
      editLink: {
        baseUrl: 'https://github.com/Origin-pod/OriginByte/edit/master/',
      },
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/Origin-pod' },
        { icon: 'x.com', label: 'X', href: 'https://x.com/sheetalssr' },
        { icon: 'linkedin', label: 'LinkedIn', href: 'https://www.linkedin.com/in/sheetal-singh-218855176' },
      ],
      customCss: [
        './src/styles/fonts.css',
        './src/styles/site.css',
        './src/styles/pages.css',
        './src/styles/gate.css',
        './src/styles/starlight.css',
      ],
      head: [
        { tag: 'script', content: themeBoot },
        { tag: 'meta', attrs: { name: 'color-scheme', content: 'light dark' } },
      ],
      components: {
        Head: './src/components/starlight/Head.astro',
        ThemeProvider: './src/components/starlight/ThemeProvider.astro',
        PageFrame: './src/components/starlight/PageFrame.astro',
        TwoColumnContent: './src/components/starlight/TwoColumnContent.astro',
        PageTitle: './src/components/starlight/PageTitle.astro',
        Footer: './src/components/starlight/Footer.astro',
        SkipLink: './src/components/starlight/SkipLink.astro',
      },
      sidebar,
    }),
    react(),
  ],
});
