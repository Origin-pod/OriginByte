# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

### Installation

```
$ yarn
```

### Local Development

```
$ yarn start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

### Build

```
$ yarn build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

### Deployment

Using SSH:

```
$ USE_SSH=true yarn deploy
```

Not using SSH:

```
$ GIT_USER=<Your GitHub username> yarn deploy
```

If you are using GitHub pages for hosting, this command is a convenient way to build the website and push to the `gh-pages` branch.

### Twitter Bookmarks Scraper

This repo includes a Playwright-based script to export your X/Twitter bookmarks to JSON.

#### Usage

1. Install dependencies and Playwright browsers:

```
npm install
npx playwright install --with-deps
```

2. Run the scraper:

```
npm run scrape:twitter:bookmarks
```

On first run, a Chromium window opens. Log in to X/Twitter if prompted, then return to the terminal and press Enter to continue. Your session persists under `.playwright-user/` for subsequent runs.

3. Output

- JSON file written to `data/twitter-bookmarks.json` with fields: `scrapedAt`, `count`, and `items` (array of tweets with `authorName`, `authorHandle`, `time`, `url`, `text`, `images`, `videos`).

Notes:

- The scraper scrolls to load more bookmarks. Adjust limits in `scripts/scrape-twitter-bookmarks.ts` if needed.
- Selectors may change if X/Twitter updates its UI.
