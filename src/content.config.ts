import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

const library = defineCollection({
  loader: glob({ pattern: '**/*.md', base: './src/content/library' }),
  schema: z.object({
    title: z.string(),
    author: z.string(),
    coverAuthor: z.string().optional(),
    category: z.enum(['databases', 'systems', 'graphics', 'languages', 'physics']),
    cover: z.enum(['verm', 'sand', 'moss', 'blue', 'black', 'lilac', 'cream', 'rose']),
    ornament: z.string().optional(),
    progress: z.number().min(0).max(100).optional(),
    status: z.enum(['unread', 'reading', 'finished']).default('unread'),
    highlights: z
      .array(z.object({ quote: z.string(), note: z.string().optional() }))
      .default([]),
    /** External object URL (Cloudflare R2). Empty until the owner uploads the PDF. */
    fileUrl: z.string().default(''),
    placeholder: z.boolean().default(true),
    short: z.string().optional(),
    pages: z.number().optional(),
    added: z.string().optional(),
    desk: z.string().optional(),
    byline: z.string().optional(),
    pageLabel: z.string().optional(),
    onDesk: z.boolean().default(false),
    spineHeight: z.number().optional(),
    spineWidth: z.number().optional(),
    order: z.number().default(0),
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
