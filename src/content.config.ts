import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';
import { isPrivateDocId } from './data/private';

/** Private shelves are drafts, so production omits their pages, sidebar, and search entries. */
function privateDocsLoader() {
  const base = docsLoader();
  return {
    name: 'originbyte-docs',
    load: async (context: Parameters<NonNullable<ReturnType<typeof docsLoader>['load']>>[0]) => {
      await base.load!(context);
      for (const id of context.store.keys()) {
        if (!isPrivateDocId(String(id))) continue;
        const entry = context.store.get(id);
        if (!entry?.data) continue;
        // Mutate the stored entry. Replacing it drops the file digest, and the loader
        // then treats the next sync as a duplicate write.
        entry.data.draft = true;
      }
    },
  };
}

export const collections = {
  docs: defineCollection({ loader: privateDocsLoader(), schema: docsSchema() }),
  blog: defineCollection({
    loader: glob({ pattern: '**/*.md', base: './src/content/blog' }),
    schema: z.object({
      title: z.string(),
      description: z.string().optional(),
      date: z.coerce.date().optional(),
    }),
  }),
};
