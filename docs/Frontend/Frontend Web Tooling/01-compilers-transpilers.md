# Compilers & Transpilers: SWC and Oxc

## SWC (Speedy Web Compiler)

**Repo:** [github.com/swc-project/swc](https://github.com/swc-project/swc) | ~31k stars
**Maintainer:** DongYoon Kang (now at Vercel)
**Language:** Rust
**License:** Apache-2.0
**Status:** Production-ready, foundational infrastructure

### What It Does

SWC is a **Babel replacement** — it parses JavaScript/TypeScript, transforms it (JSX, TypeScript stripping, decorator syntax, ES feature downleveling, polyfill injection), and emits JavaScript. It also includes a minifier (replacement for Terser).

### Internals & Architecture

```
Source Code (.ts/.tsx/.js/.jsx)
        │
        ▼
┌─────────────────┐
│   LEXER/TOKENIZER│   Converts source text into token stream
│   (swc_ecma_parser)   Handles JS, TS, JSX, TSX
└────────┬────────┘
         │ Token Stream
         ▼
┌─────────────────┐
│  RECURSIVE       │   Builds AST from tokens
│  DESCENT PARSER  │   Full ES2024+ and TypeScript 5.x support
│                  │   AST nodes defined in swc_ecma_ast crate
└────────┬────────┘
         │ AST (Abstract Syntax Tree)
         ▼
┌─────────────────┐
│  VISITOR-BASED   │   Transforms applied via Fold/Visit traits
│  TRANSFORMS      │   Each transform walks the AST and mutates/replaces nodes
│                  │   Runs in configurable pass order
│  Passes include: │
│  - TypeScript strip    (remove type annotations)
│  - JSX transform       (React.createElement / jsx runtime)
│  - Decorator transform (Stage 3 decorators)
│  - ES compat           (arrow fns, classes, async/await → ES5)
│  - Module transform    (ESM ↔ CJS ↔ AMD ↔ UMD)
│  - Polyfill injection  (core-js based)
└────────┬────────┘
         │ Transformed AST
         ▼
┌─────────────────┐
│  CODE GENERATOR  │   AST → JavaScript source text
│  (swc_ecma_codegen)   Handles whitespace, semicolons, sourcemaps
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  MINIFIER        │   Dead code elimination, name mangling,
│  (swc_ecma_minifier)  constant folding, inlining
│                  │   Targets Terser-level output quality
└─────────────────┘
```

**Key architectural details:**

- **Parallelized processing**: Leverages Rust's native multi-threading via Rayon. Multiple files are processed simultaneously across CPU cores.
- **AST representation**: Defined in `swc_ecma_ast` crate. Uses Rust enums for node types with `#[ast_node]` proc macro for boilerplate. Each node carries a `Span` for source location tracking.
- **Visitor pattern**: Two traits — `Visit` (immutable traversal) and `Fold` (transforming traversal that returns new nodes). Transforms implement one or both.
- **Plugin system**: SWC supports WASM-based plugins. Plugin authors write Rust, compile to WASM, and SWC loads them at runtime. This solves the "Rust compilation barrier" partially but adds WASM overhead.
- **Incremental compilation**: Internal design supports incremental recompilation for dev server scenarios.

### Performance

- **20x faster** than Babel on a single thread
- **70x faster** on four cores (parallel file processing)
- Instant startup (native binary, no JIT warmup)

### Ecosystem Position

| Used By | How |
|---------|-----|
| Next.js | Default transpiler (replaced Babel in Next 12+) |
| Parcel 2 | JS/TS transformation engine |
| Rspack | Via `builtin:swc-loader` for parsing and transformation |
| Deno | Uses SWC for TypeScript transpilation |

### Current Limitations

- **SWCpack (bundler) is not actively developed** — the bundling effort shifted to Turbopack
- Plugin ecosystem is small compared to Babel's thousands of plugins
- WASM plugins add ~2x overhead vs native Rust transforms
- No TypeScript type checking (intentional — SWC only strips types)

---

## Oxc (The JavaScript Oxidation Compiler)

**Repo:** [github.com/oxc-project/oxc](https://github.com/oxc-project/oxc) | ~12k stars
**Maintainer:** Boshen Chen + VoidZero team
**Sponsor:** VoidZero Inc (Evan You)
**Language:** Rust
**License:** MIT
**Status:** Parser and linter production-ready; transformer maturing rapidly

### What It Does

Oxc is a **collection of high-performance JavaScript/TypeScript tools** sharing a single parser and AST. It includes: parser, linter (Oxlint), transformer, resolver, minifier, and formatter (in development).

### Internals & Architecture

```
Source Code
     │
     ▼
┌──────────────────────┐
│   OXC PARSER          │
│                        │
│   - Recursive descent  │
│   - Full ES2024 + TS   │
│   - 3x faster than SWC │
│   - Arena allocator     │
│     (bumpalo crate)     │
│   - CompactString for   │
│     short string inline │
│   - ZERO heap allocs    │
│     beyond arena+strings│
└──────────┬─────────────┘
           │ AST (arena-allocated)
           ▼
┌──────────────────────┐
│  SEMANTIC ANALYZER    │   Built separately from parser
│                        │   - Scope binding
│                        │   - Symbol resolution
│                        │   - Reference tracking
│                        │   - Some syntax error detection
└──────────┬─────────────┘
           │ AST + Semantic Info
           │
    ┌──────┼──────┬──────────┬────────────┐
    ▼      ▼      ▼          ▼            ▼
┌──────┐┌──────┐┌────────┐┌──────────┐┌──────────┐
│LINTER││TRANS-││MINIFIER││RESOLVER  ││FORMATTER │
│(Oxlint)│FORMER││        ││(oxc_     ││(WIP)     │
│      ││      ││        ││resolver) ││          │
│340+  ││JSX,  ││        ││30x faster││          │
│rules ││TS    ││        ││than      ││          │
│      ││strip ││        ││enhanced- ││          │
│      ││etc.  ││        ││resolve   ││          │
└──────┘└──────┘└────────┘└──────────┘└──────────┘
```

**Key architectural details:**

- **Arena allocation (bumpalo)**: The entire AST lives in a memory arena. Allocation is a pointer bump (near-zero cost). Deallocation frees the entire arena at once. This is why Oxc can parse 3x faster than SWC — no individual heap allocations, no reference counting, no GC pressure.
- **CompactString**: Short strings (identifiers, keywords) are stored inline without heap allocation. Only strings exceeding the inline threshold go to the heap.
- **Zero extra heap allocations**: Beyond the arena and compact strings, the parser performs no heap allocations. This is extreme memory discipline.
- **Linear AST scan**: Because the arena stores nodes contiguously in memory, visiting the AST is a linear memory scan — highly cache-friendly. This matters enormously for linting (which visits every node).
- **Multi-threaded linting**: Files are linted in parallel across all CPU cores. Results are sent via `mpsc::channel` (multi-producer, single-consumer).
- **Shared AST**: All tools (linter, transformer, minifier, resolver) operate on the same AST. No re-parsing. This is the fundamental architectural advantage over tool ecosystems where each tool has its own parser.

### The Resolver: oxc_resolver

A standalone module resolution implementation compatible with Node.js and webpack's `enhanced-resolve`. Benchmarked at **30x faster** than `enhanced-resolve`. This is used by Rolldown for module resolution.

### Performance Benchmarks (from oxc.rs)

| Operation | Oxc | Comparison |
|-----------|-----|------------|
| Parsing | 3x faster than SWC parser |
| Linting (full suite) | 50-100x faster than ESLint |
| Module resolution | 30x faster than enhanced-resolve |

### Ecosystem Position

Oxc is the **compiler backbone of VoidZero's unified toolchain**:

```
Vite (build tool) → Rolldown (bundler) → Oxc (parser + resolver + transformer + minifier)
```

Rolldown currently uses Oxc's parser and resolver. As Oxc's transformer and minifier mature, Rolldown will consume those too, creating a fully integrated Rust pipeline.

---

## SWC vs Oxc: Head-to-Head

| Dimension | SWC | Oxc |
|-----------|-----|-----|
| **Parser speed** | Fast | ~3x faster than SWC |
| **Memory model** | Standard Rust heap alloc | Arena allocator (bumpalo) |
| **AST sharing** | Each tool re-parses | Single parse, all tools share |
| **Plugin system** | WASM plugins supported | Not yet (planned) |
| **Bundler integration** | Turbopack (Vercel) | Rolldown (VoidZero) |
| **Linter** | No built-in linter | Oxlint (340+ rules, prod-ready) |
| **Maturity** | Battle-tested (Next.js, Parcel, Deno) | Parser/linter mature; transformer emerging |
| **Backing** | Vercel | VoidZero (Evan You) |
| **TypeScript support** | Full syntax support (no type checking) | Full syntax support (no type checking) |

### Key Insight

SWC and Oxc represent two **generations** of Rust JS compiler design:
- **SWC** pioneered the approach, proved the thesis, got adopted everywhere
- **Oxc** learned from SWC's design and pushed further with arena allocation and the shared-AST architecture

They are **not interchangeable** — SWC powers the Vercel ecosystem (Turbopack, Next.js), while Oxc powers the VoidZero ecosystem (Rolldown, Vite). The JavaScript ecosystem now has two competing Rust compiler foundations.

---

## Sources

- [SWC Official Site](https://swc.rs/)
- [Oxc Official Site](https://oxc.rs/)
- [Oxc Performance Documentation](https://oxc.rs/docs/learn/performance)
- [Oxc Architecture: Linter](https://oxc.rs/docs/learn/architecture/linter)
- [Pursuit of Performance on Building a JavaScript Compiler (Rust Magazine)](https://rustmagazine.org/issue-3/javascript-compiler/)
- [InfoQ: Oxlint v1.0 Stable Released](https://www.infoq.com/news/2025/08/oxlint-v1-released/)
- [LogRocket: Why You Should Use SWC](https://blog.logrocket.com/why-you-should-use-swc/)
