# CSS Tooling: Lightning CSS

## Lightning CSS

**Repo:** [github.com/parcel-bundler/lightningcss](https://github.com/parcel-bundler/lightningcss) | ~7k stars
**Maintainer:** Devon Govett (Parcel team)
**Language:** Rust
**License:** MPL-2.0
**Status:** Production-ready, widely adopted

### What It Is

Lightning CSS is an **extremely fast CSS parser, transformer, bundler, and minifier** written in Rust. It handles the full CSS pipeline: parsing, vendor prefixing, syntax lowering (modern CSS → older browser-compatible CSS), bundling `@import` chains, and minification.

### Architecture & Internals

```
CSS Source
    │
    ▼
┌────────────────────────────────────┐
│  PARSER                            │
│                                    │
│  Built on Mozilla's cssparser      │
│  and selectors crates              │
│  (same code used by Firefox!)      │
│                                    │
│  - Tokenizer → Token stream       │
│  - Parser → Stylesheet AST        │
│  - Handles all modern CSS:         │
│    nesting, container queries,     │
│    cascade layers, custom          │
│    properties, color functions     │
└──────────────┬─────────────────────┘
               │ CSS AST
               ▼
┌────────────────────────────────────┐
│  TRANSFORMER                       │
│                                    │
│  - Vendor prefixing (autoprefixer  │
│    replacement)                    │
│  - Syntax lowering:               │
│    - CSS nesting → flat selectors  │
│    - color-mix() → static colors   │
│    - lab/lch/oklch → rgb          │
│    - logical properties → physical │
│  - Custom transforms via visitor   │
│    API (Rust)                      │
└──────────────┬─────────────────────┘
               │
               ▼
┌────────────────────────────────────┐
│  BUNDLER                           │
│                                    │
│  - Resolves @import chains         │
│  - Concatenates stylesheets        │
│  - Deduplicates rules             │
└──────────────┬─────────────────────┘
               │
               ▼
┌────────────────────────────────────┐
│  MINIFIER                          │
│                                    │
│  - Shorthand property merging      │
│  - Color space optimization        │
│  - Selector merging               │
│  - Dead code removal              │
│  - Whitespace removal             │
│  - Over 100x faster than           │
│    cssnano (JS)                    │
└────────────────────────────────────┘
```

**Mozilla DNA:**

Lightning CSS is built on `cssparser` and `selectors` crates — the same Rust crates that power **Firefox's CSS engine** (Servo/Stylo). This means Lightning CSS has the same parsing correctness and spec compliance as a real browser engine. No JavaScript CSS tool can make this claim.

### Performance

- **Over 100x faster** than comparable JavaScript tools
- Can minify **2.7 million lines of CSS per second** on a single thread
- Makes tools like autoprefixer, postcss-preset-env, and cssnano obsolete for most use cases

### Integration Points

| Tool | How It Uses Lightning CSS |
|------|--------------------------|
| **Parcel** | Default CSS transformer (built-in) |
| **Vite** | Experimental CSS processing (via `css.lightningcss` config) |
| **Tailwind CSS v4** | Uses Lightning CSS for CSS processing |
| **Next.js** | Experimental support |

### Available As

- **Rust crate** (`lightningcss` on crates.io) — for Rust-native integration
- **Node.js package** (`lightningcss` on npm) — via napi-rs native bindings
- **WASM package** (`lightningcss-wasm` on npm) — for Deno and browsers
- **Standalone CLI** — for build scripts

### What It Replaces

| JavaScript Tool | Lightning CSS Equivalent |
|-----------------|-------------------------|
| autoprefixer | Built-in vendor prefixing |
| postcss-preset-env | Built-in syntax lowering |
| cssnano | Built-in minification |
| postcss-import | Built-in @import bundling |
| postcss-nesting | Built-in nesting transform |

### Current Limitations

- **No PostCSS plugin compatibility** — existing PostCSS plugins don't work with Lightning CSS
- **No SCSS/Sass/Less support** — only standard CSS (and CSS-native nesting)
- **CSS-in-JS not handled** — Lightning CSS works on `.css` files, not runtime CSS-in-JS solutions
- Limited ecosystem of custom transforms compared to PostCSS's massive plugin library

---

## The CSS Tooling Gap

CSS tooling in Rust is arguably the **most complete** part of the ecosystem. Lightning CSS handles parsing, transforms, bundling, and minification — the full pipeline. The gaps are:

1. **SCSS/Sass compilation**: No Rust-based Sass compiler exists. `sass` (Dart Sass) and the deprecated `node-sass` (C++ LibSass) are the only options.
2. **CSS-in-JS**: Solutions like styled-components, emotion, and vanilla-extract have no Rust integration point. These are inherently JavaScript runtime constructs.
3. **PostCSS plugin ecosystem**: Thousands of PostCSS plugins exist. Lightning CSS replaces the most common ones but can't run arbitrary PostCSS plugins.
4. **Tailwind CSS**: While Tailwind v4 uses Lightning CSS internally, the Tailwind compiler itself is a separate tool (Rust-based as of v4).

---

## Sources

- [Lightning CSS Official Site](https://lightningcss.dev/)
- [Lightning CSS Getting Started](https://lightningcss.dev/docs.html)
- [Lightning CSS GitHub](https://github.com/parcel-bundler/lightningcss)
- [Lightning CSS on crates.io](https://crates.io/crates/lightningcss)
