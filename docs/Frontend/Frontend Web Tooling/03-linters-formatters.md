# Linters & Formatters: Biome, Oxlint, dprint

## Biome

**Repo:** [github.com/biomejs/biome](https://github.com/biomejs/biome) | ~16k stars
**Maintainer:** Biome core team (community-driven, forked from Rome)
**Sponsor:** Vercel (funded Biome's type inference engine)
**Language:** Rust
**License:** MIT
**Status:** Production-ready; v2 shipped 2025 with type-aware linting

### Origin Story

Biome rose from the ashes of **Rome** — an ambitious project by Sebastian McKenzie (Babel creator) that tried to build an all-in-one JavaScript toolchain. Rome's corporate entity (Rome Tools Inc) collapsed in 2023, but the community forked the codebase and established Biome as an independent open-source project.

### What It Does

Biome is a **unified linter + formatter** for JavaScript, TypeScript, JSX, JSON, CSS, and GraphQL. One tool replaces both ESLint and Prettier.

### Architecture & Internals

```
┌─────────────────────────────────────────────────────────┐
│                    BIOME CORE                            │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │              BIOME PARSER                           │  │
│  │  - Custom parser (NOT SWC, NOT Oxc)                │  │
│  │  - Lossless CST (Concrete Syntax Tree)             │  │
│  │  - Preserves ALL tokens including whitespace,       │  │
│  │    comments, trivia                                 │  │
│  │  - Error-resilient: can parse broken code           │  │
│  │  - Supports: JS, TS, JSX, TSX, JSON, CSS, GraphQL  │  │
│  └──────────────────────┬─────────────────────────────┘  │
│                         │ CST (Concrete Syntax Tree)      │
│                         │                                 │
│  ┌──────────────────────▼─────────────────────────────┐  │
│  │           TYPE INFERENCE ENGINE                      │  │
│  │  (Funded by Vercel, shipped in Biome v2)            │  │
│  │                                                      │  │
│  │  - Infers types WITHOUT running tsc                 │  │
│  │  - First tool to ship type-aware lint rules          │  │
│  │    that don't depend on the TypeScript compiler     │  │
│  │  - Enables rules like noFloatingPromises,            │  │
│  │    noMismatchedArguments                             │  │
│  └──────────────────────┬─────────────────────────────┘  │
│                         │                                 │
│         ┌───────────────┼───────────────┐                 │
│         ▼               ▼               ▼                 │
│  ┌────────────┐  ┌────────────┐  ┌────────────────┐      │
│  │  LINTER    │  │  FORMATTER │  │  IMPORT         │      │
│  │            │  │            │  │  ORGANIZER      │      │
│  │  340+ rules│  │  Prettier- │  │                 │      │
│  │  from:     │  │  compatible│  │  Sort & group   │      │
│  │  - ESLint  │  │  output    │  │  imports        │      │
│  │  - ts-     │  │            │  │                 │      │
│  │    eslint  │  │  Uses CST  │  │                 │      │
│  │  - custom  │  │  for exact │  │                 │      │
│  │            │  │  formatting│  │                 │      │
│  └────────────┘  └────────────┘  └────────────────┘      │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              LSP SERVER                              │  │
│  │  - Real-time diagnostics in editors                 │  │
│  │  - Format on save                                   │  │
│  │  - Code actions / quick fixes                       │  │
│  └─────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────┘
```

**Why a CST instead of an AST?**

Most compilers use an Abstract Syntax Tree (AST), which discards formatting information (whitespace, comments, semicolons). Biome uses a **Concrete Syntax Tree (CST)** — sometimes called a "lossless" or "full-fidelity" syntax tree — that preserves **every single token** from the source, including whitespace and comments.

This is critical for a formatter: to re-emit code with correct formatting, you need to know exactly what the original looked like. An AST-based formatter has to make assumptions; a CST-based formatter has perfect information.

The tradeoff: CSTs use more memory and are more complex to traverse. Biome compensates with Rust's performance.

**Error-resilient parsing:**

Unlike most parsers that bail on syntax errors, Biome's parser continues past errors, producing a partial CST. This is essential for editor integration (LSP) — users need diagnostics even while their code is mid-edit and syntactically broken.

### Performance: Biome vs ESLint + Prettier

| Metric | Biome | ESLint + Prettier |
|--------|-------|-------------------|
| **Speed** | 10-25x faster | Baseline |
| **Config files** | 1 (`biome.json`) | 4+ (`.eslintrc`, `.prettierrc`, etc.) |
| **npm packages** | 1 binary | 127+ transitive dependencies |
| **Type-aware rules** | Built-in inference (no tsc) | Requires `@typescript-eslint/parser` + tsc |
| **CSS support** | Built-in | Requires stylelint (separate tool) |

### Biome v2 (2025) Highlights

- **Type-aware lint rules** without depending on tsc — the standout feature
- Expanded CSS linting and formatting
- GraphQL support
- Improved migration tooling from ESLint configs

### 2026 Roadmap

Biome's [2026 roadmap](https://biomejs.dev/blog/roadmap-2026/) includes:
- Plugin/extension system (a major missing piece)
- SCSS/Sass support
- HTML linting and formatting
- Markdown support
- Expanded type inference coverage

### Current Limitations

- **No plugin system yet** — you cannot write custom lint rules (this is the biggest gap)
- CSS support is newer and less comprehensive than stylelint
- Some ESLint rules don't have Biome equivalents yet
- The type inference engine doesn't cover the full TypeScript type system

---

## Oxlint (Part of Oxc)

**Repo:** [github.com/oxc-project/oxc](https://github.com/oxc-project/oxc) (within Oxc monorepo)
**Maintainer:** VoidZero team
**Language:** Rust
**License:** MIT
**Status:** v1.0 stable released (August 2025)

### What It Is

Oxlint is the **linter component of Oxc**, designed to run alongside ESLint (not as a full replacement — yet). It handles the performance-critical lint rules in Rust while ESLint handles rules that require plugins or custom logic.

### Architecture

Oxlint leverages the shared Oxc architecture:
- Parses via Oxc's arena-allocated parser (zero extra heap allocs)
- Runs semantic analysis for scope/symbol resolution
- Applies lint rules as visitors over the enriched AST
- Multi-threaded: files processed in parallel across all CPU cores
- Results collected via `mpsc::channel`

### 340+ Built-in Rules

Rules ported from:
- ESLint core
- `@typescript-eslint`
- `eslint-plugin-react`
- `eslint-plugin-react-hooks`
- `eslint-plugin-jsx-a11y`
- `eslint-plugin-import`
- `eslint-plugin-unicorn`
- And more

### Oxlint vs Biome: Philosophical Difference

| | Oxlint | Biome |
|---|--------|-------|
| **Scope** | Linter only | Linter + Formatter + Import Organizer |
| **Strategy** | Complement ESLint (disable slow rules in ESLint, run them in Oxlint) | Replace ESLint + Prettier entirely |
| **Parser** | Oxc (AST, arena-allocated) | Biome (CST, lossless) |
| **Type awareness** | Coming (late 2025/early 2026) | Shipped in v2 (2025) |
| **Custom rules** | Not yet | Not yet |
| **Backing** | VoidZero | Community + Vercel |

---

## dprint

**Repo:** [github.com/dprint/dprint](https://github.com/dprint/dprint) | ~3k stars
**Maintainer:** David Sherret (also a Deno contributor)
**Language:** Rust
**License:** MIT
**Status:** Production-ready, stable

### What It Is

dprint is a **pluggable code formatting platform** written in Rust. Unlike Biome which bundles everything, dprint is a **host** that loads language-specific formatters as **WASM plugins**.

### Architecture

```
┌────────────────────────────────┐
│         dprint CLI             │
│  (Rust binary, manages plugins)│
└──────────────┬─────────────────┘
               │ Loads WASM plugins
    ┌──────────┼──────────┬────────────┐
    ▼          ▼          ▼            ▼
┌────────┐┌────────┐┌────────┐┌────────────┐
│TS/JS   ││ JSON   ││ Markdown│ │ Dockerfile │
│Plugin  ││Plugin  ││ Plugin ││ Plugin     │
│(uses   ││       ││        ││            │
│SWC     ││       ││        ││            │
│parser) ││       ││        ││            │
└────────┘└────────┘└────────┘└────────────┘
   WASM       WASM      WASM       WASM
```

**Key design:**

- **Plugin isolation via WASM**: Each formatter runs in a WASM sandbox. Plugins can't crash the host. Languages can add formatting without modifying dprint core.
- **TypeScript plugin** uses SWC parser internally (compiled to WASM)
- Fast: Rust host + WASM plugins still outperform pure JS formatters
- **Used by Deno** as its built-in formatter (`deno fmt` uses dprint)

### dprint vs Prettier vs Biome

| | dprint | Prettier | Biome |
|---|--------|----------|-------|
| **Language** | Rust + WASM plugins | JavaScript | Rust |
| **Plugin model** | WASM-based, extensible | JS plugins, many built-in | Monolithic (no plugins yet) |
| **Speed** | Fast (Rust + WASM) | Slow (pure JS) | Fastest (pure Rust) |
| **Adoption** | Deno default formatter | Industry standard | Growing rapidly |

---

## Sources

- [Biome GitHub](https://github.com/biomejs/biome)
- [Biome 2026 Roadmap](https://biomejs.dev/blog/roadmap-2026/)
- [Biome Linter Documentation](https://biomejs.dev/linter/)
- [Biome vs ESLint + Prettier (Better Stack)](https://betterstack.com/community/guides/scaling-nodejs/biome-eslint/)
- [InfoQ: Oxlint v1.0 Stable Released](https://www.infoq.com/news/2025/08/oxlint-v1-released/)
- [dprint GitHub](https://github.com/dprint/dprint)
- [Joshua Goldberg: Rust-Based JavaScript Linters](https://www.joshuakgoldberg.com/blog/rust-based-javascript-linters-fast-but-no-typed-linting-right-now/)
