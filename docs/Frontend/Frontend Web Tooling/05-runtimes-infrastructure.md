# Runtimes & Infrastructure: Deno, Volta, fnm, napi-rs, wasm-pack

## JavaScript Runtimes with Rust DNA

### Deno

**Repo:** [github.com/denoland/deno](https://github.com/denoland/deno) | ~100k stars
**Creator:** Ryan Dahl (also created Node.js)
**Language:** Rust (core) + V8 (JS engine)
**License:** MIT
**Status:** Production-ready (Deno 2.x)

**Architecture:**

```
┌──────────────────────────────────────────┐
│                DENO CORE                  │
│                                          │
│  ┌──────────────┐  ┌──────────────────┐  │
│  │  Rust Runtime │  │  V8 Engine       │  │
│  │  (Tokio-based │  │  (JavaScript     │  │
│  │   async I/O)  │  │   execution)     │  │
│  └──────┬───────┘  └────────┬─────────┘  │
│         │                    │            │
│         │  ops (Rust ↔ V8 bridge)        │
│         └────────┬───────────┘            │
│                  │                        │
│  ┌───────────────▼───────────────────┐   │
│  │  Built-in Toolchain               │   │
│  │  - TypeScript compiler (SWC)      │   │
│  │  - Formatter (dprint)             │   │
│  │  - Linter (custom, Rust-based)    │   │
│  │  - Test runner                    │   │
│  │  - Bundler                        │   │
│  │  - Package manager (JSR + npm)    │   │
│  └───────────────────────────────────┘   │
│                                          │
│  ┌───────────────────────────────────┐   │
│  │  Security Sandbox                 │   │
│  │  - Explicit permissions model     │   │
│  │  - --allow-read, --allow-net, etc │   │
│  └───────────────────────────────────┘   │
└──────────────────────────────────────────┘
```

**How Rust is used in Deno:**

- **Tokio**: The async runtime. All I/O (file system, network, timers) goes through Tokio's event loop.
- **SWC**: TypeScript transpilation. When you run a `.ts` file, Deno uses SWC to strip types and downlevel syntax before passing it to V8.
- **dprint**: The built-in formatter (`deno fmt`).
- **rusty_v8**: Rust bindings to V8, the JavaScript engine. This is how Rust code calls into JavaScript and vice versa.
- **ops system**: Deno defines "ops" — Rust functions exposed to JavaScript. Each built-in API (file I/O, HTTP, crypto) is implemented as a Rust op.

**Deno 2.x key changes:**

- Full Node.js/npm compatibility (can run most npm packages)
- JSR (JavaScript Registry) — a new package registry designed for Deno and modern runtimes
- Workspaces support
- Private npm registry support
- Backwards compatible with Node.js APIs

### Bun (Zig-based, not Rust — but relevant context)

**Repo:** [github.com/oven-sh/bun](https://github.com/oven-sh/bun) | ~75k stars
**Creator:** Jarred Sumner
**Language:** Zig + C++ (JavaScriptCore engine)

Bun is **not Rust-based** but is part of the same movement: rewriting JavaScript infrastructure in systems languages for performance. Bun bundles runtime + package manager + bundler + test runner into one binary, using Zig instead of Rust. It uses Apple's JavaScriptCore instead of V8.

**Why mention it:** Bun validates the same thesis as the Rust tools — that JS tooling performance matters and systems languages deliver it. Bun competes with Deno for the "next-gen runtime" space.

---

## Toolchain Management

### Volta

**Repo:** [github.com/volta-cli/volta](https://github.com/volta-cli/volta) | ~11k stars
**Language:** Rust
**License:** BSD-2-Clause
**Status:** Production-ready

**What it does:** Volta is a **JavaScript tool manager** (similar to nvm/fnm but broader). It manages Node.js, npm, yarn, and per-project tool versions.

**Key architecture:**
- Written entirely in Rust — instant binary, no runtime dependencies
- Uses **shims**: installs lightweight wrapper scripts on your PATH that intercept commands (`node`, `npm`, `yarn`) and route them to the correct version
- Project pinning: reads `volta` section in `package.json` to auto-switch versions
- Cross-platform: macOS, Linux, Windows

**vs nvm:** Volta is dramatically faster (Rust binary vs bash script), supports pinning in `package.json`, and handles npm/yarn versions too.

### fnm (Fast Node Manager)

**Repo:** [github.com/Schniz/fnm](https://github.com/Schniz/fnm) | ~18k stars
**Language:** Rust
**License:** GPL-3.0
**Status:** Production-ready

**What it does:** fnm is a **fast Node.js version manager** — a direct nvm replacement focused purely on Node.js version management.

**Key features:**
- Written in Rust — single binary, ~5MB
- **40x faster** than nvm for version switching
- `.node-version` and `.nvmrc` file support
- Cross-platform (macOS, Linux, Windows)
- Shell completions for bash, zsh, fish, PowerShell

**fnm vs Volta:**
- fnm: focused on Node.js version management only (nvm replacement)
- Volta: broader scope — manages Node.js + npm + yarn + project-level pinning

---

## Rust ↔ JavaScript Bridge Technologies

These are the technologies that make it possible to write performance-critical code in Rust and call it from JavaScript.

### napi-rs

**Repo:** [github.com/napi-rs/napi-rs](https://github.com/napi-rs/napi-rs) | ~6k stars
**Creator:** LongYinan (also works on Rspack)
**Language:** Rust
**License:** MIT
**Status:** Production-ready (v2)

**What it does:** napi-rs is a framework for building **Node.js native addons in Rust** using the N-API (Node-API) interface.

**Architecture:**

```
┌──────────────────┐     N-API      ┌──────────────────┐
│   Your Rust Code  │◄──────────────►│   Node.js Runtime │
│                    │  (stable ABI)  │                    │
│  #[napi]           │               │  const result =    │
│  fn fibonacci(n)   │               │    fibonacci(40)   │
│    -> u64 { ... }  │               │                    │
└──────────────────┘               └──────────────────┘
```

**How it works:**
- You write Rust functions annotated with `#[napi]`
- napi-rs generates the C ABI glue code at compile time
- The output is a `.node` native addon that Node.js can `require()` or `import()`
- N-API is a **stable ABI** — your addon works across Node.js versions without recompilation

**Used by:**
| Tool | What It Uses napi-rs For |
|------|--------------------------|
| SWC | Node.js bindings (`@swc/core`) |
| Lightning CSS | Node.js bindings (`lightningcss`) |
| Rspack | Node.js bindings (the Rust ↔ JS bridge for loaders/plugins) |
| Biome | Node.js bindings |
| Oxc | Node.js bindings |
| Rolldown | Node.js bindings |

**napi-rs is arguably the most critical piece of infrastructure** in this ecosystem — it's the bridge that allows all these Rust tools to be consumed as npm packages.

### wasm-bindgen

**Repo:** [github.com/rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) | ~7.5k stars
**Maintainer:** Rust and WebAssembly Working Group
**Language:** Rust
**License:** MIT + Apache-2.0
**Status:** Production-ready

**What it does:** wasm-bindgen facilitates **high-level interactions between Rust and JavaScript** via WebAssembly. It generates glue code so Rust functions can be called from JS (and vice versa) across the WASM boundary.

**Key capabilities:**
- Import JavaScript functions into Rust (`#[wasm_bindgen]` extern blocks)
- Export Rust functions to JavaScript
- Support for complex types: strings, structs, enums, closures
- Integration with Web APIs (DOM, fetch, console, etc.)
- TypeScript type generation

### wasm-pack

**Repo:** [github.com/rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack) | ~6k stars
**Maintainer:** Rust and WebAssembly Working Group
**Language:** Rust
**License:** MIT + Apache-2.0
**Status:** Production-ready

**What it does:** wasm-pack is the **build tool** for Rust-to-WebAssembly workflows. It compiles Rust to WASM, generates JS/TS glue code (via wasm-bindgen), and packages everything as an npm-publishable package.

**Workflow:**

```
Your Rust Crate
     │
     │ wasm-pack build
     ▼
┌────────────────────────────┐
│  Output (npm-ready package) │
│                              │
│  ├── package.json            │
│  ├── my_crate_bg.wasm       │  ← compiled WASM binary
│  ├── my_crate.js            │  ← JS glue code
│  └── my_crate.d.ts          │  ← TypeScript types
└────────────────────────────┘
     │
     │ npm publish / import
     ▼
  Consumable in any JS project
```

### napi-rs vs wasm-bindgen: When to Use Which

| Dimension | napi-rs (Native) | wasm-bindgen (WASM) |
|-----------|-------------------|---------------------|
| **Runtime** | Node.js only | Browsers + Node.js + Deno + any WASM host |
| **Performance** | Native speed (fastest) | ~2x overhead vs native |
| **Portability** | Platform-specific binary (must precompile for mac/linux/win) | Universal WASM binary (runs everywhere) |
| **File I/O** | Full access | Sandboxed (no direct filesystem) |
| **Multi-threading** | Full Rust threading | Limited (SharedArrayBuffer + workers) |
| **Distribution** | Prebuild for each platform | Single .wasm file for all platforms |
| **Use case** | CLI tools, build tools, server-side | Browser libraries, plugins, portable tools |

**The pattern in practice:**

Most Rust JS tools ship **both**:
- napi-rs bindings for Node.js CLI usage (fastest)
- WASM builds for browser/Deno usage or plugin systems

Example: Lightning CSS ships `lightningcss` (napi-rs, Node.js) and `lightningcss-wasm` (browser/Deno).

---

## Sources

- [Deno Official Site](https://deno.com)
- [napi-rs Official Site](https://napi.rs/)
- [napi-rs: Announcing v2](https://napi.rs/blog/announce-v2)
- [wasm-bindgen GitHub](https://github.com/rustwasm/wasm-bindgen)
- [wasm-pack GitHub](https://github.com/rustwasm/wasm-pack)
- [Volta Official Site](https://volta.sh/)
- [fnm GitHub](https://github.com/Schniz/fnm)
- [Deno: wasmbuild](https://deno.com/blog/wasmbuild)
