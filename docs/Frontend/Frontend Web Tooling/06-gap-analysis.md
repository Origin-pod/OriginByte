# Gap Analysis: What's Missing, What's Hard, What's Broken

## The Current Coverage Map

```
FULLY COVERED BY RUST (production-ready replacements exist)
═══════════════════════════════════════════════════════════
✅ JS/TS Parsing          (SWC, Oxc — faster than any JS parser)
✅ JS/TS Transpilation     (SWC — Babel replacement)
✅ JS/TS Bundling          (Rspack, Turbopack, Rolldown)
✅ JS/TS Minification      (SWC minifier, Oxc minifier)
✅ JS/TS Linting           (Biome, Oxlint — 10-100x faster than ESLint)
✅ JS/TS Formatting         (Biome formatter, dprint)
✅ CSS Parsing             (Lightning CSS — uses Firefox's parser)
✅ CSS Transforms          (Lightning CSS — autoprefixer + postcss-preset-env)
✅ CSS Minification        (Lightning CSS — 100x faster than cssnano)
✅ CSS Bundling            (Lightning CSS — @import resolution)
✅ Module Resolution       (oxc_resolver — 30x faster than enhanced-resolve)
✅ Node.js Version Mgmt    (fnm, Volta)
✅ Rust ↔ JS Bridge        (napi-rs, wasm-bindgen)

PARTIALLY COVERED (Rust tools exist but with significant gaps)
═══════════════════════════════════════════════════════════
⚠️  Type-Aware Linting     Biome v2 has inference engine, but doesn't cover
                           full TS type system. Oxlint type-aware: coming soon.
⚠️  Plugin Systems         Rspack has webpack plugin compat; Rolldown has Rollup
                           compat. But custom Rust plugins require Rust knowledge.
⚠️  Test Running           Deno has built-in test runner. Vitest (JS) dominates.
                           No standalone Rust-based JS test runner.
⚠️  Package Management     orogene (Rust npm client) exists but low adoption.
                           pnpm/npm/yarn still dominate.
⚠️  Dev Servers            Bundlers include dev servers, but not as standalone tools.

NOT COVERED (no production Rust replacement)
═══════════════════════════════════════════════════════════
❌ TypeScript Type Checking (full semantic analysis)
❌ SCSS/Sass/Less Compilation
❌ CSS-in-JS (styled-components, emotion, etc.)
❌ Framework Compilers (Svelte compiler, Angular compiler)
❌ PostCSS Plugin Ecosystem
❌ Custom ESLint Rule Authoring (in Rust)
❌ E2E Testing Frameworks (Playwright, Cypress — not a Rust problem)
❌ Hot Module Replacement Protocol (standardized)
❌ Source Map Composition (across tool chains)
```

---

## Deep Dive: The Hard Problems

### 1. TypeScript Type Checking — The Holy Grail Nobody Has Cracked

**Why it matters:** TypeScript's type checker (`tsc`) is the single slowest tool in most JS developer workflows. A Rust-based type checker would be the most impactful tool possible.

**Why it's incredibly hard:**

TypeScript's type system is **Turing-complete**. It supports:
- Conditional types (`T extends U ? X : Y`)
- Mapped types (`{ [K in keyof T]: ... }`)
- Template literal types (`` `${A}-${B}` ``)
- Recursive type aliases
- Declaration merging
- Module augmentation
- Overload resolution with complex inference

The TypeScript compiler is **~1.5 million lines of code** — not because Microsoft engineers are verbose, but because the type system is genuinely that complex. The type checker interacts with itself recursively (types depend on other types, which depend on computed types, which depend on conditional types...).

**Failed/stalled attempts:**

| Project | Status | What Happened |
|---------|--------|---------------|
| **stc** (Speedy TypeScript Checker) | **Abandoned** | Created by DongYoon Kang (SWC author). Officially abandoned. The complexity of faithfully reimplementing tsc's behavior proved overwhelming for a small team. |
| **ezno** | **Active but far from ready** | Solo project by Ben Sheridan. Takes a different approach — not trying to replicate tsc exactly, but building a type checker with different tradeoffs. Cannot check real-world TS projects yet. |

**The fundamental difficulty:**

```
tsc type checking is NOT just "check if types match"

It involves:
1. Parse all files in the project + all type declarations
2. Build a program-wide symbol table
3. Resolve all type aliases, interfaces, generics
4. Evaluate conditional types (can require arbitrary computation)
5. Perform overload resolution with complex scoring
6. Handle declaration merging across files
7. Check assignability with structural typing
8. Infer generic type parameters
9. Narrow types through control flow analysis
10. Handle module systems (ESM, CJS, declaration files)

Steps 3-9 are deeply intertwined and recursive.
Rust's ownership model makes this particularly challenging —
type checking requires pervasive shared mutable state
(the type environment), which fights Rust's borrow checker.
```

**Microsoft's own approach:** In 2025, Microsoft announced they are porting tsc to Go (not Rust) — project codenamed "Corsa". This sidesteps the shared mutable state problem since Go has a garbage collector, making it easier to port the existing tsc architecture.

**Current workaround:** Tools like Biome implement a **partial type inference engine** — good enough for lint rules but far from a full type checker.

### 2. The Plugin Problem — Extensibility vs Performance

**The tension:**

JavaScript tools like ESLint, PostCSS, and webpack are slow but **infinitely extensible** — anyone can write a plugin in JavaScript. Rust tools are fast but creating plugins requires:

1. Writing Rust (steep learning curve for JS developers)
2. Compiling to a native binary or WASM
3. Understanding Rust's ownership/borrowing model
4. Dealing with cross-compilation for different platforms

**Current approaches to plugins:**

| Approach | Used By | Tradeoff |
|----------|---------|----------|
| **WASM plugins** | SWC, dprint | Universal, sandboxed, ~2x slower than native |
| **JavaScript plugins via NAPI bridge** | Rspack, Rolldown | Compatible with existing ecosystem, adds JS↔Rust overhead |
| **No plugins (monolithic)** | Biome | Fast, but you can't customize |
| **Query languages (GritQL)** | Some experiments | New syntax to learn, limited power |
| **Rust-only plugins** | Oxc (planned) | Maximum performance, tiny contributor pool |

**The Rspack/Rolldown hybrid approach:**

Both Rspack and Rolldown allow JavaScript plugins (webpack plugins and Rollup plugins respectively) through their Node.js bridge layer. This gives ecosystem compatibility at the cost of performance — every plugin invocation crosses the Rust↔JS boundary via napi-rs.

```
Fast path (pure Rust):
    Rust Code → Rust Plugin → Rust Code
    No overhead, maximum speed

Slow path (JS plugin compat):
    Rust Code → napi-rs → Node.js → JS Plugin → Node.js → napi-rs → Rust Code
    Each crossing adds serialization/deserialization overhead
```

**Marvin Hagemeister's insight** (from the Deno team): The future is running JavaScript plugins **inside** Rust tools — Deno 2.2 shipped the ability for Rust tools to host a JS runtime for plugin evaluation. This gives you Rust speed for the core pipeline and JS flexibility for plugins, all in one process.

### 3. Ecosystem Fragmentation — Three Parsers, No Shared AST

**The problem:** The Rust JS tooling ecosystem has **three separate JavaScript parsers**, each with their own AST format:

| Parser | Used By | AST Format |
|--------|---------|------------|
| SWC Parser (`swc_ecma_parser`) | SWC, Turbopack, Rspack, Parcel, Deno | `swc_ecma_ast` |
| Oxc Parser | Oxc, Oxlint, Rolldown, Vite 8+ | Oxc AST (arena-allocated) |
| Biome Parser | Biome | Biome CST (lossless concrete syntax tree) |

**Why this is a problem:**

- Tools from different ecosystems **cannot share work**. If Rspack (SWC parser) and Rolldown (Oxc parser) both need to parse the same file, they parse it independently.
- Plugin authors must learn **different AST formats** for different tools.
- No equivalent of Babel's AST (which became a de facto standard).
- Optimization opportunities are missed — a shared parse could feed multiple tools.

**Why unification is unlikely:**

Each parser made fundamentally different design decisions:
- **SWC**: Standard heap-allocated Rust AST. Battle-tested, stable API.
- **Oxc**: Arena-allocated AST. Faster, but the arena lifecycle constrains how you use the AST. You can't hold AST nodes beyond the arena's lifetime.
- **Biome**: Concrete Syntax Tree (lossless). Preserves all whitespace/comments. Essential for formatting but more memory-intensive and complex to traverse.

These aren't arbitrary choices — they reflect the **core requirements** of each tool. A formatter needs lossless representation. A fast parser needs arena allocation. A battle-tested ecosystem needs API stability.

### 4. The "Last Mile" Problem

Rust tools handle 80-95% of use cases excellently but struggle with edge cases:

**Source map composition:**
When you chain Rust tools (SWC → Rspack → minifier), each tool produces source maps. Composing these maps correctly is complex and bugs here mean debugger breakpoints are wrong.

**CSS-in-JS:**
Tools like styled-components and emotion generate CSS at runtime from JavaScript template literals. No Rust tool can optimize this because the CSS doesn't exist until the JavaScript runs.

**Framework-specific compilers:**
- Svelte has its own compiler (JS-based) that transforms `.svelte` files
- Angular has a compiler (TS-based) for templates and dependency injection
- Vue's SFC compiler is being Rust-ified through VoidZero, but it's a massive effort

These framework compilers contain hundreds of thousands of lines of framework-specific logic that would need to be rewritten from scratch.

### 5. Memory Model Mismatch

**The conceptual problem:**

JavaScript's garbage collector lets you freely create and share references to AST nodes. A typical compiler pass in JavaScript might:
1. Walk the AST
2. Collect references to interesting nodes in a Set
3. Modify some nodes in-place
4. Create new nodes that reference existing ones

In Rust, this pattern fights the ownership system. You can't hold mutable references to multiple AST nodes simultaneously. You can't easily build indexes of nodes without `Rc<RefCell<>>` or unsafe code.

**How each parser handles this:**

- **SWC**: Uses `Arc` for shared ownership, `swc_common::GLOBALS` for global state. Some `unsafe` blocks in hot paths.
- **Oxc**: Arena allocation means all nodes live in the same arena. The arena owns everything. But you can't move nodes between arenas or hold them after the arena drops.
- **Biome**: CST uses a `GreenNode`/`SyntaxNode` tree inspired by rust-analyzer's `rowan` crate. Interior mutability is carefully managed.

---

## Adoption Barriers

### Contributor Pipeline

The JavaScript ecosystem has millions of developers. The Rust ecosystem has hundreds of thousands. The intersection — people who can write Rust AND understand JS tooling internals — is tiny.

This means:
- Bug reports outnumber people who can fix bugs
- Custom rules/plugins require Rust knowledge
- Framework-specific integrations need specialists in both worlds
- Documentation must bridge two worlds

### Migration Paths

| From | To | Migration Difficulty |
|------|-----|---------------------|
| Babel → SWC | Low | SWC handles most Babel transforms; config migration tools exist |
| webpack → Rspack | Low | Drop-in compatible; 40+ of top 50 plugins work |
| ESLint → Biome | Medium | Different rule names/config; migration CLI available |
| ESLint → Oxlint | Low | Run alongside ESLint, gradually move rules |
| Prettier → Biome | Low | Mostly Prettier-compatible output |
| Rollup/Vite → Rolldown | Low (eventually) | Designed as drop-in for Vite 8+ |
| PostCSS → Lightning CSS | Medium-High | Plugin ecosystem not portable |
| tsc → ??? | **Impossible** | No Rust replacement exists |

---

## What to Watch in 2026

1. **Vite 8 + Rolldown stabilization** — If VoidZero delivers a stable Rolldown-powered Vite, it could become the default build tool for the majority of the JS ecosystem
2. **Biome plugin system** — If Biome ships a plugin API, it could rapidly close the ESLint ecosystem gap
3. **Microsoft's tsc-in-Go (Corsa)** — Not Rust, but if successful, validates the thesis that type checking in a compiled language is viable
4. **Oxlint type-aware rules** — Could become the de facto fast linter if type awareness lands
5. **Shared infrastructure** — Whether any parser becomes a shared standard or if fragmentation persists

---

## Sources

- [Joshua Goldberg: Rust-Based JavaScript Linters: Fast, But No Typed Linting Right Now](https://www.joshuakgoldberg.com/blog/rust-based-javascript-linters-fast-but-no-typed-linting-right-now/)
- [Marvin Hagemeister: Speeding Up JavaScript Ecosystem - Rust and JavaScript Plugins](https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-11/)
- [Medium: Why Rust & Go Took Over JavaScript Tooling in 2025](https://medium.com/@vinesheg/why-rust-go-took-over-javascript-tooling-in-2025-and-why-it-matters-283168dcacda)
- [endform.dev: JavaScript is Being Rewritten in Rust](https://endform.dev/blog/js-is-being-rewritten-in-rust)
- [JetBrains: Rust vs JavaScript & TypeScript](https://blog.jetbrains.com/rust/2026/01/27/rust-vs-javascript-typescript/)
- [stc (Abandoned)](https://github.com/dudykr/stc)
- [ezno (Experimental)](https://github.com/kaleidawave/ezno)
- [Biome 2026 Roadmap](https://biomejs.dev/blog/roadmap-2026/)
