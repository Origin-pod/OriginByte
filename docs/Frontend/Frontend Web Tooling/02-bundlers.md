# Bundlers: Turbopack, Rspack, Rolldown, Farm

## Turbopack

**Repo:** [github.com/vercel/turborepo](https://github.com/vercel/turborepo) (monorepo containing Turbopack) | ~26k stars
**Maintainer:** Vercel (Tobias Koppers — webpack creator, DongYoon Kang — SWC creator)
**Language:** Rust
**License:** MIT
**Status:** Production-ready for Next.js dev; production builds GA in Next.js 15+

### What It Is

Turbopack is an **incremental bundler** optimized for JavaScript and TypeScript, written in Rust. It is designed as the successor to webpack — built by the same person (Tobias Koppers) but from scratch in Rust.

### Architecture & Internals

```
┌─────────────────────────────────────────────────┐
│              TURBO ENGINE                         │
│  (Incremental Computation Framework)              │
│                                                   │
│  - Function-level caching and invalidation        │
│  - Dependency graph of computations               │
│  - When an input changes, only recompute          │
│    the minimal set of affected functions           │
│  - Persistent cache across restarts               │
└──────────────────────┬──────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────┐
│              MODULE GRAPH                         │
│                                                   │
│  - Resolves imports/exports across all files      │
│  - Each module is a node; imports are edges       │
│  - Resolution uses enhanced-resolve compatible    │
│    resolver                                       │
│  - Supports: ESM, CJS, CSS, JSON, assets          │
└──────────────────────┬──────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────┐
│              TRANSFORMATION LAYER                 │
│                                                   │
│  - Uses SWC for JS/TS parsing and transforms      │
│  - PostCSS and Lightning CSS for CSS              │
│  - Each transform is an incremental function      │
│    tracked by the Turbo Engine                    │
└──────────────────────┬──────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────┐
│              CHUNKING & EMIT                      │
│                                                   │
│  - Splits module graph into chunks                │
│  - Code splitting, tree shaking                   │
│  - Emits bundled JS/CSS                           │
│  - Source map generation                          │
└─────────────────────────────────────────────────┘
```

**The Turbo Engine — the key innovation:**

The Turbo Engine is an **incremental computation framework** inspired by Salsa (used in rust-analyzer). The core idea:

1. Every operation (parsing a file, resolving an import, transforming a module) is modeled as a **function with tracked inputs**
2. Results are **cached** with their dependency graph
3. When any input changes (file save, config change), the engine computes the **minimal set of functions to re-execute**
4. This granularity goes down to **individual functions**, not just files

This is why Turbopack's HMR is fast even in massive codebases — changing one file only recomputes the exact functions affected, not the entire bundle.

### Ecosystem Position

- **Tightly coupled to Next.js** — Turbopack is the default bundler in `next dev` and now supports production builds
- **Not a standalone general-purpose bundler** (yet) — you don't `npm install turbopack` for a Vite-like experience
- Uses SWC for all JavaScript/TypeScript compilation
- Vercel's long-term play: replace webpack entirely in the Next.js ecosystem

### Current Limitations

- Not usable outside Next.js ecosystem easily
- No public plugin API comparable to webpack's
- webpack migration requires moving to Next.js

---

## Rspack

**Repo:** [github.com/web-infra-dev/rspack](https://github.com/web-infra-dev/rspack) | ~10k stars
**Maintainer:** ByteDance (web-infra-dev team)
**Language:** Rust
**License:** MIT
**Status:** Production-ready (v1.0 shipped August 2024)

### What It Is

Rspack is a **webpack-compatible bundler written in Rust**. Its strategic differentiator: you can **drop Rspack into an existing webpack project** with minimal config changes and get 5-23x faster builds.

### Architecture & Internals

```
┌───────────────────────────────────────────────┐
│               RSPACK CORE (Rust)               │
│                                                │
│  ┌──────────────┐  ┌──────────────────────┐   │
│  │ Module Graph  │  │ Chunk Graph          │   │
│  │ Builder       │  │ (code splitting,     │   │
│  │               │  │  tree shaking)       │   │
│  └──────┬───────┘  └──────────┬───────────┘   │
│         │                      │               │
│  ┌──────▼───────┐  ┌──────────▼───────────┐   │
│  │ SWC-based    │  │ Concurrent            │   │
│  │ Loader       │  │ Compilation           │   │
│  │ (builtin:swc-│  │ (multi-core           │   │
│  │  loader)     │  │  parallelism)         │   │
│  └──────────────┘  └─────────────────────┘    │
│                                                │
│  ┌────────────────────────────────────────┐    │
│  │ Incremental Compilation Engine         │    │
│  │ (inspired by Parcel's approach)        │    │
│  │ - Module-level cache invalidation      │    │
│  │ - Optimized HMR                        │    │
│  └────────────────────────────────────────┘    │
└──────────────────────┬────────────────────────┘
                       │
                       │ NAPI-RS Bridge
                       │
┌──────────────────────▼────────────────────────┐
│          NODE.JS COMPATIBILITY LAYER           │
│                                                │
│  - webpack-compatible config schema            │
│  - Loader API (babel-loader, less-loader,      │
│    sass-loader, vue-loader, etc.)              │
│  - Plugin API (subset of webpack plugins)      │
│  - Compatible with 40+ of top 50 webpack       │
│    plugins                                     │
└────────────────────────────────────────────────┘
```

**Key architectural details:**

- **Concurrent architecture** inspired by esbuild — takes full advantage of multi-core CPUs via Rust's threading model
- **SWC integration**: Uses SWC for code parsing, transformation, and minification via `builtin:swc-loader`
- **NAPI-RS bridge**: The Rust core communicates with Node.js via napi-rs, allowing JavaScript loaders and plugins to work
- **Incremental compilation**: Module-level cache invalidation for fast HMR (inspired by Parcel's design)
- **webpack compatibility layer**: Implements webpack's configuration schema, loader interface, and a large subset of the plugin API

### Performance

- **23x faster** build times compared to webpack (documented benchmark)
- **5-10x faster** HMR updates
- Parallel module processing across all CPU cores

### Production Usage at ByteDance

| App | Scale |
|-----|-------|
| TikTok | Web frontend |
| Douyin | Web frontend |
| Lark | Enterprise collaboration |
| Coze | AI platform |

### Ecosystem: Rsbuild

ByteDance also maintains **Rsbuild** — a higher-level build tool built on Rspack (similar to how Create React App was built on webpack). Rsbuild provides:
- Zero-config setup for React, Vue, Svelte
- Built-in support for TypeScript, CSS Modules, PostCSS
- Dev server with fast HMR
- Production optimization out of the box

---

## Rolldown

**Repo:** [github.com/rolldown/rolldown](https://github.com/rolldown/rolldown) | ~9k stars
**Maintainer:** VoidZero Inc (Evan You's team)
**Language:** Rust
**License:** MIT
**Status:** Active development; powering Vite 8 beta

### What It Is

Rolldown is a **Rust port of Rollup**, designed to become Vite's bundler. It replaces the previous Vite architecture that used esbuild for dev transforms and Rollup for production builds — a dual-engine approach that caused behavioral inconsistencies.

### Architecture & Internals

```
┌─────────────────────────────────────────────┐
│              ROLLDOWN CORE                    │
│                                               │
│  ┌─────────────────────────────────────────┐ │
│  │         OXC PARSER + RESOLVER           │ │
│  │  - Parses JS/TS/JSX/TSX via Oxc        │ │
│  │  - Resolves modules via oxc_resolver    │ │
│  │  - Arena-allocated AST                  │ │
│  └──────────────────┬──────────────────────┘ │
│                     │                         │
│  ┌──────────────────▼──────────────────────┐ │
│  │         MODULE GRAPH                     │ │
│  │  - Parallel module loading              │ │
│  │  - Tree shaking (scope analysis)        │ │
│  │  - Side-effect detection                │ │
│  └──────────────────┬──────────────────────┘ │
│                     │                         │
│  ┌──────────────────▼──────────────────────┐ │
│  │         CHUNK GENERATION                 │ │
│  │  - Code splitting strategies            │ │
│  │  - Rollup-compatible output formats     │ │
│  │  - ESM, CJS, IIFE output               │ │
│  └──────────────────┬──────────────────────┘ │
│                     │                         │
│  ┌──────────────────▼──────────────────────┐ │
│  │         OXC TRANSFORMS + MINIFIER       │ │
│  │  - JSX, TS transforms via Oxc          │ │
│  │  - Minification via Oxc minifier       │ │
│  └─────────────────────────────────────────┘ │
└──────────────────────┬──────────────────────┘
                       │
                       │ Rollup-compatible Plugin API
                       │
┌──────────────────────▼──────────────────────┐
│          PLUGIN COMPATIBILITY                 │
│                                               │
│  - Rollup plugin API compatibility           │
│  - Existing Vite/Rollup plugins can work     │
│  - Plugin hooks: resolveId, load, transform  │
└───────────────────────────────────────────────┘
```

**Key architectural details:**

- **Built on Oxc**: Uses Oxc's parser, resolver, transformer, and minifier — same AST, no re-parsing
- **Rollup API compatibility**: Aims to be a drop-in for Rollup in the Vite ecosystem
- **Parallel module loading**: Modules are loaded and parsed concurrently
- **Unified dev/prod**: Solves Vite's historical inconsistency between esbuild (dev) and Rollup (prod)

### The Vite Integration Story

```
Vite 1-7 (old):   esbuild (dev transforms) + Rollup (prod bundling)
                   ↑ Two different engines = behavioral mismatches

Vite 8+ (new):    Rolldown (both dev and prod)
                   ↑ Single engine = consistent behavior
```

### Current Status

- Vite 8 beta released with Rolldown as the bundler
- Vite+ (commercial product) public preview targeted for early 2026
- All open source components remain MIT forever

---

## Farm

**Repo:** [github.com/farm-fe/farm](https://github.com/farm-fe/farm) | ~5k stars
**Language:** Rust + TypeScript
**License:** MIT
**Status:** Active development, smaller community

### What It Is

Farm is a **Rust-based web build tool** that aims to be extremely fast while supporting the Vite plugin ecosystem. It takes a different approach from the others by focusing on being a complete build tool (not just a bundler).

### Key Features

- Vite-compatible plugin system
- Built-in support for HTML, CSS, JS/TS, static assets
- Partial bundling strategy (between "no-bundle" and "full bundle")
- Dev server with HMR
- Lazy compilation for dev

### Ecosystem Position

Farm is the **smallest player** in this space. While technically capable, it lacks the corporate backing and ecosystem integration of Turbopack (Vercel), Rspack (ByteDance), or Rolldown (VoidZero). It serves as an interesting technical experiment in Rust-based build tooling but faces an uphill adoption battle.

---

## Bundler Comparison Matrix

| Dimension | Turbopack | Rspack | Rolldown | Farm |
|-----------|-----------|--------|----------|------|
| **Backing** | Vercel | ByteDance | VoidZero | Community |
| **Compat target** | New architecture | webpack | Rollup/Vite | Vite plugins |
| **Parser** | SWC | SWC | Oxc | SWC |
| **Primary framework** | Next.js | Any (webpack users) | Vite | Any |
| **Plugin API** | Internal | webpack-compatible | Rollup-compatible | Vite-compatible |
| **Prod-ready** | Yes (in Next.js) | Yes (v1.0) | Beta (Vite 8) | Alpha-ish |
| **Stars** | ~26k (monorepo) | ~10k | ~9k | ~5k |
| **Key innovation** | Incremental computation engine | webpack drop-in | Unified Vite dev/prod | Partial bundling |
| **HMR approach** | Function-level recompute | Module-level cache | Module-level | Lazy compilation |

---

## Sources

- [Rspack Introduction](https://rspack.rs/guide/start/introduction)
- [InfoQ: Rspack 1.0 Released, 23x Faster than Webpack](https://www.infoq.com/news/2024/10/rspack-released/)
- [Vercel: Turbopack](https://vercel.com/blog/turbopack)
- [Next.js: Turbopack API Reference](https://nextjs.org/docs/app/api-reference/turbopack)
- [VoidZero: Announcing Vite 8 Beta](https://voidzero.dev/posts/announcing-vite-8-beta)
- [VoidZero: What's New Oct 2025](https://voidzero.dev/posts/whats-new-oct-2025)
- [DeepWiki: Rspack Architecture](https://deepwiki.com/web-infra-dev/rspack)
- [AppSignal: Introduction to Rspack](https://blog.appsignal.com/2025/04/16/an-introduction-to-javascript-bundler-rspack.html)
