# Origin Byte

A hand-kept index of the things Sheetal Singh builds, notes, and reads. Astro 5, Starlight 0.34, and React 19 islands, deployed on Netlify. The visual system is the approved Index mockup: off-white directory, hairline borders, vermilion `#ff4f12`, Public Sans, Geist Mono, and Newsreader (self-hosted, with the optical-size axis).

## Develop

```bash
npm install
npm run dev
npm run build
```

`netlify.toml` installs Playwright Chromium (without `--with-deps`; that flag needs `su`, which the Netlify image rejects) before the build so `rehype-mermaid` can render diagrams, then publishes `dist`. Node 22.

## Layout

| Path | What it is |
| --- | --- |
| `/` | Home. Card fan, letter, search. |
| `/work/` | Résumé, from `src/data/resume.ts`. |
| `/hacks/` | The Lab, from `src/data/apps.ts`. |
| `/notes/` | Notes index. Individual notes stay on Starlight at `/notes/...`. |
| `/blog/` | The eight essays. URLs are unchanged. |
| `/library/` | Reading list. Title, optional author, and a status. |
| `/enter-the-void/` | Passphrase gate for private note shelves. |
| `src/styles/site.css` | Design tokens and shared chrome. |
| `src/styles/pages.css` | Page-specific layout. |
| `src/styles/fonts.css` | Self-hosted `@font-face` rules. No Google Fonts. |
| `src/scripts/site.js` | Theme, fan, rune, lists, reader, void, keyboard. |
| `src/components/chrome/` | The one nav and footer. |

Light and dark follow the system, and a manual choice is saved as `ob-theme`. An inline script in the document head applies it before paint. Switching uses a circular View Transitions reveal, with a fade when that API is missing. `prefers-reduced-motion` turns the motion off.

Keyboard: `S` shuffles the home fan, `T` toggles the theme, `R` is surprise, `/` focuses search. The mockup’s `P` placeholder outline is not in production.

## Add a note

Put a Markdown or MDX file under `src/content/docs/notes/<shelf>/`. Starlight picks it up from the sidebar in `astro.config.mjs`. The notes index groups by the directory name and reads titles from `src/data/shelves.ts` (`shelfMeta`). Add a shelf there, and to `shelfOrder`, when you create a new directory. Private shelves (`theoretical-physics`, `quantum-computing`, `handbooks`) stay hidden until the Void passphrase is entered.

The sample note `/notes/databases/b-trees-vs-lsm/` includes the write-ratio slider. The B-tree key-drop animation lives on the Lab feature, which is the figure the mockup animates.

## Add an app

Edit `src/data/apps.ts`. `group: 'here'` is for apps served by this site. Everything else is `ou` (Origin Universe) or `sq` (side quest) and should link to its own repository. Blurbs copied from the mockup are marked `blurbPlaceholder: true`.

The DB Simulator has one canonical URL: `/dbsim/`. `public/_redirects` proxies that path to `https://origin-pod.github.io/Origin-DB-Simulator/`. The source repository stays `https://github.com/Origin-pod/Origin-DB-Simulator`.

## Add a book

Add `src/content/library/<slug>.md` with:

```yaml
title: ""
author: ""              # optional
status: want-to-read    # reading | read | want-to-read, optional
fileUrl: ""             # optional HTTPS URL of the file. Leave unset until it exists.
placeholder: true       # set false once this is a real entry
```

The page groups and filters by `status` only. A `fileUrl` makes that row a link. There is no progress, no highlights, and no per-book page.

### PDFs live in Cloudflare R2, not in git

Book files are not committed. `fileUrl` should be an object URL (or a signed URL minted by a Worker) on a private R2 bucket. The planned shape:

1. Create an R2 bucket, for example `originbyte-library`, with public access off.
2. Upload one PDF per book. Keep the object key stable, such as `books/database-internals.pdf`.
3. Put a Worker or Netlify function in front of the bucket if the objects must stay private, and point `fileUrl` at that URL.
4. Set `placeholder: false` once the link works.

Every book currently in `src/content/library` is `placeholder: true` with no `fileUrl`.

The PDF that used to live at `docs/Random Explorations/UBI/Tell me with web research what.pdf` was removed with the unused Docusaurus `docs/` tree. It was not referenced by the Astro build. It remains recoverable from git history and was not uploaded anywhere.

## Résumé PDF

`PUBLIC_RESUME_PDF_URL` (a public env var, read in `src/data/site.ts`) is the download target of the PDF button on `/work/`. It is empty until set. Do not commit the résumé PDF, and do not add the phone number from it — the email is the only contact published.

## The Void

`/enter-the-void/` compares a SHA-256 of the typed word with the hash in `src/data/site.ts` and, on a match, sets `sessionStorage['ssr:devmode']`. That reveals the private note shelves and the footer rune. This is a client-side courtesy, not access control: the notes are in the built HTML, and anyone can read the hash. The plaintext passphrase is not in the repo.
