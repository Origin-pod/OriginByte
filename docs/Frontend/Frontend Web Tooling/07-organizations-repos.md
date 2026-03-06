# Active Organizations & Repositories

## Tier 1: Major Corporate-Backed Projects

### Vercel Ecosystem

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [vercel/turborepo](https://github.com/vercel/turborepo) | ~26k | Rust | Turbopack bundler + Turborepo monorepo tool |
| [swc-project/swc](https://github.com/swc-project/swc) | ~31k | Rust | JS/TS compiler (Babel replacement) |

**Key People:**
- **Tobias Koppers** — Creator of webpack, now leads Turbopack at Vercel
- **DongYoon Kang** (kdy1) — Creator of SWC, hired by Vercel

**Funding:** Vercel has raised $563M+ total. SWC and Turbopack development is funded through Vercel's engineering budget.

**GitHub Org:** [github.com/vercel](https://github.com/vercel)

---

### ByteDance / web-infra-dev Ecosystem

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [web-infra-dev/rspack](https://github.com/web-infra-dev/rspack) | ~10k | Rust | webpack-compatible bundler |
| [web-infra-dev/rsbuild](https://github.com/web-infra-dev/rsbuild) | ~2k | TypeScript + Rust | Higher-level build tool on Rspack |
| [web-infra-dev/rspress](https://github.com/web-infra-dev/rspress) | ~1.5k | TypeScript | Static site generator on Rspack |

**Key People:**
- **Zack Jackson** — Module Federation creator, contributes to Rspack
- ByteDance's internal web-infra team

**Funding:** ByteDance internal engineering investment. Rspack is used in production across TikTok, Douyin, Lark, Coze.

**GitHub Org:** [github.com/web-infra-dev](https://github.com/web-infra-dev)

---

### VoidZero Ecosystem

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [oxc-project/oxc](https://github.com/oxc-project/oxc) | ~12k | Rust | Parser, linter, transformer, resolver, minifier |
| [rolldown/rolldown](https://github.com/rolldown/rolldown) | ~9k | Rust | Rollup-compatible bundler for Vite |
| [vitejs/vite](https://github.com/vitejs/vite) | ~70k | TypeScript | Build tool / dev server (moving to Rolldown) |
| [vitest-dev/vitest](https://github.com/vitest-dev/vitest) | ~14k | TypeScript | Test runner (Vite-native) |

**Key People:**
- **Evan You** — Creator of Vue.js and Vite, founder/CEO of VoidZero
- **Boshen Chen** — Lead of Oxc project
- **Underfin** — Core Rolldown contributor

**Funding:**
- $4.6M seed round (Accel, 2024)
- $12.5M Series A (Accel, Peak XV, 2025)
- All projects remain MIT open source forever

**Company:** [VoidZero Inc](https://voidzero.dev)
**GitHub Orgs:** [github.com/oxc-project](https://github.com/oxc-project), [github.com/rolldown](https://github.com/rolldown), [github.com/vitejs](https://github.com/vitejs)

---

## Tier 2: Independent / Community-Driven Projects

### Biome

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [biomejs/biome](https://github.com/biomejs/biome) | ~16k | Rust | Linter + formatter (ESLint + Prettier replacement) |

**Key People:**
- **Emanuele Stoppa** — Core maintainer
- Community-driven team (forked from Rome after Rome Tools Inc collapsed)

**Funding:** Vercel sponsored the type inference engine. Open Collective for community funding.

**GitHub Org:** [github.com/biomejs](https://github.com/biomejs)

---

### Parcel / Lightning CSS

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [parcel-bundler/lightningcss](https://github.com/parcel-bundler/lightningcss) | ~7k | Rust | CSS parser, transformer, bundler, minifier |
| [parcel-bundler/parcel](https://github.com/parcel-bundler/parcel) | ~43k | JS + Rust | Zero-config bundler (uses Rust components) |

**Key People:**
- **Devon Govett** — Creator and primary maintainer of both Parcel and Lightning CSS

**GitHub Org:** [github.com/parcel-bundler](https://github.com/parcel-bundler)

---

### dprint

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [dprint/dprint](https://github.com/dprint/dprint) | ~3k | Rust | Pluggable code formatter (used by Deno) |

**Key People:**
- **David Sherret** — Creator (also contributes to Deno)

**GitHub Org:** [github.com/dprint](https://github.com/dprint)

---

### napi-rs

| Repo | Stars | Language | What It Does |
|------|-------|----------|--------------|
| [napi-rs/napi-rs](https://github.com/napi-rs/napi-rs) | ~6k | Rust | Rust → Node.js native addon framework |

**Key People:**
- **LongYinan (Brooooooklyn)** — Creator (also works on Rspack)

**GitHub Org:** [github.com/napi-rs](https://github.com/napi-rs)

---

## Tier 3: Experimental / Niche Projects

| Repo | Stars | Language | Status | What It Does |
|------|-------|----------|--------|--------------|
| [denoland/deno](https://github.com/denoland/deno) | ~100k | Rust | Production | JS runtime built on Rust + V8 |
| [Schniz/fnm](https://github.com/Schniz/fnm) | ~18k | Rust | Production | Fast Node.js version manager |
| [volta-cli/volta](https://github.com/volta-cli/volta) | ~11k | Rust | Production | JS tool version manager |
| [farm-fe/farm](https://github.com/farm-fe/farm) | ~5k | Rust | Active dev | Rust-based build tool |
| [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) | ~7.5k | Rust | Production | Rust ↔ JS via WASM |
| [rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack) | ~6k | Rust | Production | Build tool for Rust → WASM |
| [kaleidawave/ezno](https://github.com/kaleidawave/ezno) | ~2.5k | Rust | Experimental | TypeScript type checker |
| [dudykr/stc](https://github.com/dudykr/stc) | ~5.5k | Rust | **Abandoned** | Speedy TypeScript Checker |

---

## Dependency Graph: Who Uses What

```
                    ┌─────────────┐
                    │   napi-rs    │  (Rust ↔ Node.js bridge)
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
    ┌─────▼─────┐   ┌─────▼─────┐   ┌──────▼─────┐
    │    SWC     │   │    Oxc     │   │  Lightning  │
    │  (parser,  │   │  (parser,  │   │    CSS      │
    │  transform,│   │  linter,   │   │  (CSS       │
    │  minifier) │   │  resolver, │   │   engine)   │
    └──┬──┬──┬──┘   │  transform)│   └──────┬──────┘
       │  │  │      └──┬──┬─────┘           │
       │  │  │         │  │                 │
       │  │  │         │  │    ┌────────────┘
       │  │  │         │  │    │
  ┌────▼┐ │ ┌▼────┐ ┌─▼──▼┐ ┌─▼───────┐
  │Turbo││ │Rspack│ │Roll-│ │ Parcel  │
  │pack ││ │      │ │down │ │         │
  └──┬──┘│ └──┬───┘ └──┬──┘ └─────────┘
     │   │    │        │
     │   │    │   ┌────▼───┐
     │   │    │   │  Vite   │
     │   │    │   │  8+     │
  ┌──▼───▼┐   │   └────────┘
  │Next.js│   │
  └───────┘   │
              │
         ┌────▼───┐
         │  Deno   │  (uses SWC for TS, dprint for fmt)
         └────────┘
```

---

## Curated Lists & Resources

| Resource | URL |
|----------|-----|
| awesome-js-tooling-in-rust | [github.com/aspect-build/rules_js](https://github.com/aspect-build/rules_js) |
| Are We Web Yet? | [arewewebyet.org](https://www.arewewebyet.org/) |
| Oxc Projects Using Oxc | [oxc.rs/docs/guide/projects](https://oxc.rs/docs/guide/projects) |
| Build Tools Performance Benchmarks | [github.com/rspack-contrib/build-tools-performance](https://github.com/rspack-contrib/build-tools-performance) |
| State of JS 2024: Build Tools | [2024.stateofjs.com/en-US/libraries/build_tools](https://2024.stateofjs.com/en-US/libraries/build_tools/) |
| This Dot Labs: 2025 Guide to JS Build Tools | [thisdot.co/blog/the-2025-guide-to-js-build-tools](https://www.thisdot.co/blog/the-2025-guide-to-js-build-tools) |

---

## Sources

- [VoidZero: Announcing VoidZero Inc](https://voidzero.dev/posts/announcing-voidzero-inc)
- [VoidZero: Announcing Vite+](https://voidzero.dev/posts/announcing-vite-plus)
- [Accel: Seed Investment in VoidZero](https://www.accel.com/noteworthies/our-seed-investment-in-voidzero-evan-yous-bold-vision-for-javascript-tooling)
- [IndianWeb2: VoidZero Raises $12.5M Series A](https://www.indianweb2.com/2025/10/voidzero-raises-125m-series-to-launch.html)
- [The New Stack: Vite Creator Launches Company](https://thenewstack.io/vite-creator-launches-company-to-build-javascript-toolchain/)
- [InfoQ: Rspack 1.0 Released](https://www.infoq.com/news/2024/10/rspack-released/)
- [InfoQ: Oxlint v1.0 Released](https://www.infoq.com/news/2025/08/oxlint-v1-released/)
