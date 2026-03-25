import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';

export default defineConfig({
  site: 'https://originbyte.netlify.app',
  integrations: [
    starlight({
      title: 'SSR',
      logo: {
        src: './src/assets/profile.jpg',
      },
      favicon: '/favicon.ico',
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/happycoder0011' },
        { icon: 'x.com', label: 'X', href: 'https://x.com/sheetalssr' },
      ],
      customCss: ['./src/styles/custom.css'],
      sidebar: [
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
        { label: 'Theoretical Physics', autogenerate: { directory: 'notes/theoretical-physics' } },
        { label: 'Quantum Computing', autogenerate: { directory: 'notes/quantum-computing' } },
        { label: 'In-Depth Handbooks', autogenerate: { directory: 'notes/handbooks' } },
      ],
    }),
    react(),
  ],
});
