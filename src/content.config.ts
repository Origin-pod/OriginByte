import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

/**
 * Reading tracker. Titles stay placeholders until the real list arrives.
 * PDFs are not committed; `fileUrl` points at external storage when one exists.
 */
const library = defineCollection({
  loader: glob({ pattern: '**/*.md', base: './src/content/library' }),
  schema: z.object({
    title: z.string(),
    author: z.string().optional(),
    status: z.enum(['reading', 'read', 'want-to-read']).optional(),
    fileUrl: z.string().optional(),
    placeholder: z.boolean().default(true),
  }),
});

export const collections = {
  docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }),
  blog: defineCollection({
    loader: glob({ pattern: '**/*.md', base: './src/content/blog' }),
    schema: z.object({
      title: z.string(),
      description: z.string().optional(),
      date: z.coerce.date().optional(),
    }),
  }),
  library,
};
