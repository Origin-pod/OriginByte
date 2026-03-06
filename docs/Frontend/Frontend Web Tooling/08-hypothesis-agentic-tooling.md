# Hypothesis: Tooling Built for Human Hands Cannot Serve Machine Minds

> **Core Thesis:** The entire frontend web tooling stack — compilers, bundlers, linters, formatters, dev servers — was designed around a single assumption: **a human is writing the code.** Every design decision optimizes for human cognition: readable error messages, interactive watch modes, incremental compilation for fast feedback, syntax highlighting, pretty-printed output. As AI agents become the primary code authors, this assumption breaks. We need a fundamentally different tooling layer — not replacing human tools, but sitting alongside them — optimized for how LLMs perceive, generate, and verify code.

---

## Part 1: The Mismatch — Why Current Tools Fail AI Agents

### 1.1 The Human-Centric Design Assumptions

Every tool in the current ecosystem encodes assumptions about its user being human:

| Human Assumption | Tool Design Decision | Why It Fails for AI Agents |
|------------------|---------------------|---------------------------|
| Humans read text linearly | Error messages are sentences: "Cannot find module './utils'" | LLMs waste tokens parsing natural language errors when structured data (`{type: "MODULE_NOT_FOUND", path: "./utils", from: "src/index.ts:14"}`) would be 5x more efficient |
| Humans work on one file at a time | Watch mode, HMR, incremental compilation | AI agents work on 5-50 files simultaneously in a single pass. Incremental compilation per-keystroke is wasted work |
| Humans need visual feedback | Pretty-printed output, colored terminal text, progress bars | AI agents need machine-readable success/failure signals. ANSI color codes are noise tokens |
| Humans iterate: write → save → see error → fix | Dev servers with live reload | AI agents should **verify before emitting**. The write-then-check loop wastes an entire generation cycle |
| Humans understand context implicitly | Errors reference line numbers assuming you'll open the file | AI agents need the **surrounding code context included in the error** to fix it without an additional file-read round trip |
| Humans choose tools interactively | `npm init`, `npx create-vite`, interactive prompts | AI agents need deterministic, non-interactive APIs |
| Humans read documentation | READMEs, tutorials, API docs in prose | AI agents need **type signatures, examples, and constraints** — not paragraphs |

### 1.2 The Token Tax: How Current Tools Waste LLM Capacity

Every interaction an AI agent has with existing tooling imposes a **token tax** — tokens spent on overhead instead of productive reasoning.

```
CURRENT AGENTIC WORKFLOW (wasteful):

Step 1: Agent generates code (text)                        ~500 tokens output
Step 2: Agent writes file to disk                          ~50 tokens (tool call)
Step 3: Agent runs linter                                  ~50 tokens (tool call)
Step 4: Agent reads linter output                          ~300 tokens input
        (parsing human-readable error messages)
Step 5: Agent reasons about errors                         ~200 tokens reasoning
Step 6: Agent reads the file again to understand context   ~800 tokens input
Step 7: Agent generates fix                                ~300 tokens output
Step 8: Agent writes fix to disk                           ~50 tokens (tool call)
Step 9: Agent runs linter again                            ~50 tokens (tool call)
Step 10: Agent reads result: "no errors"                   ~100 tokens input

TOTAL: ~2,400 tokens for a simple lint-fix cycle
       ~60% of tokens spent on I/O overhead, not reasoning
```

```
IDEAL AGENTIC WORKFLOW (with agent-native tooling):

Step 1: Agent generates code as structured AST operations   ~400 tokens output
Step 2: Tool validates AST operations instantly (in-process) 0 tokens (no I/O)
        → Returns structured result: {valid: true}           ~20 tokens input
Step 3: Tool applies operations and runs verification        ~30 tokens (tool call)
        → Returns: {lint: "pass", types: "pass", tests: [    ~80 tokens input
            {name: "auth.test", status: "pass"}]}

TOTAL: ~530 tokens — a 4.5x reduction
```

### 1.3 The Context Window is the New Bottleneck

The most expensive resource in AI-driven development is **context window space**. Current frontier models have 1-2M token windows. A typical production codebase is 5-50M tokens. This means:

- An AI agent can hold **2-10% of a production codebase** in context at once
- Every token spent on tool output formatting, error message prose, or re-reading files is a token not available for understanding code
- The "context engineering" discipline (structuring what goes into the context window) has become more important than prompt engineering

**Current tools don't care about this.** A webpack error dump can be 2,000 tokens. An ESLint report on a large file can be 1,500 tokens. A TypeScript error with its full "did you mean?" suggestions can be 500 tokens for a single error. These tools were designed when the reader had unlimited attention (a human scanning with their eyes). In a world where attention is measured in tokens, this is catastrophically wasteful.

---

## Part 2: The Agentic Tooling Layer — What We Need to Build

### Principle: Don't Replace Human Tools. Build a Parallel Layer.

Human developers aren't going away. The tools they use (Vite, ESLint, Prettier, VSCode) will continue to exist and evolve. What we need is a **parallel tooling layer** purpose-built for AI agent consumption — consuming the same source code but optimized for machine interaction patterns.

```
                    ┌─────────────────────┐
                    │    SOURCE CODE       │
                    │    (same .ts files)   │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                                  │
    ┌─────────▼──────────┐            ┌─────────▼──────────┐
    │  HUMAN TOOL LAYER   │            │  AGENT TOOL LAYER   │
    │                      │            │                      │
    │  Vite dev server     │            │  AST Operation API   │
    │  ESLint (pretty)     │            │  Structured Verify   │
    │  Prettier             │            │  Codebase Skeleton   │
    │  VSCode / terminals  │            │  Semantic Diff        │
    │  Interactive CLIs     │            │  Impact Analyzer     │
    │                      │            │  Type Constraint API  │
    │  Optimized for:      │            │  Optimized for:       │
    │  - Readability        │            │  - Token efficiency   │
    │  - Interactivity     │            │  - Structured I/O    │
    │  - Visual feedback    │            │  - Batch operations  │
    │  - Incremental work  │            │  - Verification-first │
    └──────────────────────┘            └──────────────────────┘
```

---

## Part 3: Nine Agentic Tools That Should Exist

### Tool 1: `ast-surgeon` — AST-Level Code Manipulation API

**The problem:** LLMs generate code as **text**. This means every character must be correct — one mismatched brace, one wrong indent, one typo in an import path, and the code is broken. The LLM then needs another full generation cycle to fix it.

**The idea:** Instead of generating raw text, give the LLM a **semantic operation API** that manipulates the AST directly.

```jsonc
// Instead of generating this text (error-prone):
"import { useState, useEffect } from 'react';\n\nexport function UserProfile({ userId }) {\n  const [user, setUser] = useState(null);\n  useEffect(() => {\n    fetch(`/api/users/${userId}`).then(r => r.json()).then(setUser);\n  }, [userId]);\n  return user ? <div>{user.name}</div> : <p>Loading...</p>;\n}"

// The agent issues these structured operations (correct by construction):
{
  "operations": [
    {
      "op": "add_import",
      "file": "src/components/UserProfile.tsx",
      "from": "react",
      "specifiers": ["useState", "useEffect"]
    },
    {
      "op": "add_function",
      "file": "src/components/UserProfile.tsx",
      "name": "UserProfile",
      "export": true,
      "params": [{"name": "userId", "destructured_from": "props"}],
      "body_template": "hook_fetch_pattern",
      "template_args": {
        "state_var": "user",
        "endpoint": "/api/users/${userId}",
        "deps": ["userId"],
        "render_loaded": "<div>{user.name}</div>",
        "render_loading": "<p>Loading...</p>"
      }
    }
  ]
}
```

**Why this works:**
- Operations are **syntactically valid by construction** — the tool generates correct AST nodes
- No wasted tokens on boilerplate (import syntax, function declaration syntax, JSX closing tags)
- The LLM focuses on **intent** (what code to write) not **syntax** (how to write it)
- Operations are composable and reversible — easy undo/redo

**Built on:** Oxc or SWC's AST infrastructure. The Rust parser constructs the nodes; the LLM just specifies which nodes to create and where.

**Token savings:** 40-60% reduction in output tokens for typical code generation tasks.

---

### Tool 2: `codebase-skeleton` — Token-Optimized Codebase Representation

**The problem:** When an AI agent needs to understand a codebase, it must read files — often many files. A typical `src/` directory might be 50,000 tokens. But the agent usually only needs the **structure and signatures**, not every implementation detail.

**The idea:** A tool that produces a **multi-resolution codebase map** optimized for LLM consumption.

```
Level 0: Directory Tree (50 tokens)
├── src/
│   ├── components/ (14 files)
│   ├── hooks/ (6 files)
│   ├── api/ (8 files)
│   └── utils/ (4 files)

Level 1: File Signatures (500 tokens)
src/components/UserProfile.tsx
  exports: UserProfile(props: {userId: string}) → JSX
  imports: react{useState,useEffect}, ../api/users{fetchUser}
  dependencies: UserAvatar, LoadingSpinner

Level 2: Function Bodies for Target Files (2000 tokens)
[Full implementation of only the files the agent needs to modify]

Level 3: Test Context (300 tokens)
src/__tests__/UserProfile.test.tsx
  describes: "UserProfile"
  tests: ["renders loading state", "renders user name", "handles fetch error"]
  coverage: 87% lines
```

**How it works internally:**

1. **tree-sitter parse** all files → extract ASTs
2. **Extract signatures**: function names, parameter types, return types, exports, imports
3. **Build dependency graph**: which file imports which, PageRank for importance
4. **Compress using Repomix-style techniques**: ~70% token reduction from raw source
5. **Serve at requested resolution level**: agent asks for Level 1, gets 500 tokens instead of 50,000

**Built on:** tree-sitter (already used by Aider, Cline, VT Code for repo maps), Oxc's parser for deeper type extraction, graph algorithms for dependency ranking.

**Key insight from research:** Aider already pioneered this with its "Repository Map" pattern — tree-sitter parses code into ASTs, extracts function signatures, builds a dependency graph, and uses PageRank to rank what's important. But Aider's version is ad-hoc and embedded inside Aider. This should be a **standalone tool** that any agent framework can consume.

**Token savings:** 90-95% reduction vs reading raw source files.

---

### Tool 3: `verify-pipe` — Instant Structured Verification Pipeline

**The problem:** After generating code, an AI agent runs a sequence: lint → type-check → test. Each step is a separate tool call, produces human-readable output, and requires the agent to parse the output and reason about it. A single lint-fix-typecheck cycle can cost 3,000+ tokens in I/O.

**The idea:** A single tool that runs the full verification pipeline and returns a **structured, token-minimal result**.

```jsonc
// Agent calls:
{ "tool": "verify-pipe", "files": ["src/components/UserProfile.tsx"] }

// Tool returns (structured, ~100 tokens instead of ~2000):
{
  "status": "fail",
  "parse": "pass",
  "lint": {
    "status": "fail",
    "errors": [
      {
        "rule": "no-unused-vars",
        "symbol": "setLoading",
        "file": "src/components/UserProfile.tsx",
        "line": 8,
        "fix": "remove_declaration",
        "context": "const [loading, setLoading] = useState(false);"
      }
    ]
  },
  "types": {
    "status": "pass"
  },
  "tests": {
    "status": "pass",
    "ran": 3,
    "passed": 3,
    "files": ["src/__tests__/UserProfile.test.tsx"]
  },
  "suggested_operations": [
    {
      "op": "remove_variable",
      "file": "src/components/UserProfile.tsx",
      "line": 8,
      "var": "setLoading"
    }
  ]
}
```

**Critical features:**
- **Structured output**: JSON, not prose. No "Did you mean...?" suggestions in paragraph form.
- **Context included**: Each error includes the relevant code line so the agent doesn't need to re-read the file.
- **Suggested operations**: Where possible, the tool suggests AST operations to fix the issue — the agent can accept or reject without reasoning from scratch.
- **Cascading pipeline**: Stops early if parsing fails (no point running lint on unparseable code).
- **Affected-only testing**: Only runs tests that import the changed files (via dependency graph), not the full suite.

**Built on:** Oxc/Biome for linting, tsc or Biome's inference for type-checking, Vitest/Jest for testing. The innovation is the **orchestration layer** and structured output format.

---

### Tool 4: `semantic-diff` — Machine-Readable Change Representation

**The problem:** When an AI agent needs to understand what changed (e.g., reviewing a PR, understanding why a test broke), it reads a `git diff`. But git diffs are **line-level text diffs** — they show which lines were added/removed but not **what semantically changed**.

```diff
# Human-friendly (git diff) — 15 lines, ~200 tokens:
- function getUser(id) {
-   return db.query('SELECT * FROM users WHERE id = ?', [id]);
+ async function getUser(id: string): Promise<User> {
+   const result = await db.query('SELECT * FROM users WHERE id = ?', [id]);
+   if (!result) {
+     throw new UserNotFoundError(id);
+   }
+   return result;
  }
```

```jsonc
// Agent-friendly (semantic-diff) — ~80 tokens:
{
  "changes": [
    {"type": "made_async", "symbol": "getUser"},
    {"type": "added_param_type", "symbol": "getUser", "param": "id", "type": "string"},
    {"type": "added_return_type", "symbol": "getUser", "type": "Promise<User>"},
    {"type": "added_null_check", "on": "result", "throws": "UserNotFoundError"}
  ],
  "impact": {
    "callers": ["src/api/routes.ts:getUser", "src/services/auth.ts:validateUser"],
    "callers_need_await": true
  }
}
```

**Why this matters for agents:**
- The semantic diff tells the agent **what happened** (function became async, types were added, error handling added) in a fraction of the tokens
- It automatically computes **impact** — which callers need to be updated because the function is now async
- No tokens wasted on understanding diff syntax (the `+`/`-`/`@@` format)

**Built on:** Oxc/SWC parser to build ASTs of before/after, then a tree diff algorithm (similar to GumTree or ChangeDistiller) to compute semantic operations.

---

### Tool 5: `impact-graph` — Blast Radius Computation

**The problem:** When an AI agent modifies a function, it doesn't automatically know what else breaks. Current workflow: change code → run all tests → wait → read failures → figure out what else to change. This is the most expensive loop in agentic coding.

**The idea:** A precomputed **dependency graph** that instantly answers: "If I change function X in file Y, what other files/functions/tests are affected?"

```jsonc
// Agent asks:
{
  "tool": "impact-graph",
  "query": "modify",
  "symbol": "fetchUser",
  "file": "src/api/users.ts"
}

// Tool returns instantly (precomputed graph lookup):
{
  "direct_dependents": [
    {"file": "src/components/UserProfile.tsx", "symbol": "UserProfile", "usage": "call"},
    {"file": "src/hooks/useUser.ts", "symbol": "useUser", "usage": "call"},
    {"file": "src/services/auth.ts", "symbol": "validateSession", "usage": "call"}
  ],
  "transitive_dependents": [
    {"file": "src/pages/Dashboard.tsx", "via": "useUser"},
    {"file": "src/pages/Settings.tsx", "via": "useUser"}
  ],
  "affected_tests": [
    "src/__tests__/UserProfile.test.tsx",
    "src/__tests__/useUser.test.ts",
    "src/__tests__/auth.test.ts"
  ],
  "type_constraints": {
    "return_type_used_as": "User | null",
    "callers_expect": "Promise<User>",
    "breaking_if": "return type changes or becomes sync"
  }
}
```

**Built on:** Oxc's resolver (30x faster than enhanced-resolve) + tree-sitter for cross-file symbol resolution + a precomputed graph stored in memory. The graph is rebuilt incrementally on file changes (similar to Turbopack's incremental computation engine, but for dependency analysis rather than bundling).

**Token savings:** Instead of running all tests (and reading all failures), the agent knows exactly which 3 files and 3 test files to focus on. Could save 1,000-5,000 tokens per modification cycle.

---

### Tool 6: `type-scaffold` — Type-Driven Code Generation Constraints

**The problem:** When an LLM generates a function implementation, it has no guardrails except its own training data. It might generate code that doesn't match the expected types, misses edge cases, or uses wrong APIs.

**The idea:** Given type signatures, generate a **constraint scaffold** — a structured description of what the implementation MUST do to be correct.

```jsonc
// Input: a type signature
{
  "tool": "type-scaffold",
  "signature": "async function createOrder(items: CartItem[], userId: string): Promise<Order>"
}

// Output: constraints the implementation must satisfy
{
  "must_return": "Promise<Order>",
  "must_await": true,
  "parameters": {
    "items": {
      "type": "CartItem[]",
      "constraints": ["must handle empty array", "each CartItem has: id, quantity, price"],
      "likely_operations": ["map", "reduce", "filter"]
    },
    "userId": {
      "type": "string",
      "constraints": ["must validate non-empty"],
      "likely_usage": ["database lookup", "authorization check"]
    }
  },
  "return_type_shape": {
    "Order": {
      "fields": ["id: string", "items: OrderItem[]", "total: number", "userId: string", "createdAt: Date"],
      "required_fields": ["id", "items", "total", "userId"]
    }
  },
  "error_cases": [
    "userId not found → throw or return error",
    "items empty → validation error",
    "database failure → handle async rejection"
  ],
  "available_apis": [
    {"name": "db.orders.create", "signature": "(data: CreateOrderInput) => Promise<Order>"},
    {"name": "validateUser", "signature": "(id: string) => Promise<User>"}
  ]
}
```

**Why this helps:**
- The LLM gets a **structured spec** to code against, not a vague task
- Type constraints eliminate entire classes of bugs before generation
- Available APIs are surfaced so the LLM doesn't hallucinate non-existent functions
- Error cases are enumerated so the LLM doesn't forget edge cases

**Built on:** TypeScript's type system (or Biome's inference engine) to extract type information, Oxc's resolver to find available APIs in scope.

---

### Tool 7: `batch-edit` — Atomic Multi-File Transactions

**The problem:** AI agents often need to make coordinated changes across 5-20 files. With current tools, each file edit is a separate operation. If the agent edits 3 of 5 files and then hits a context limit or error, the codebase is in an inconsistent state.

**The idea:** A transactional multi-file edit system with atomic commit-or-rollback semantics.

```jsonc
{
  "tool": "batch-edit",
  "transaction": [
    {
      "file": "src/api/users.ts",
      "operations": [
        {"op": "change_return_type", "function": "getUser", "new_type": "Promise<User | null>"},
        {"op": "make_async", "function": "getUser"}
      ]
    },
    {
      "file": "src/components/UserProfile.tsx",
      "operations": [
        {"op": "add_null_check", "variable": "user", "fallback": "<NotFound />"}
      ]
    },
    {
      "file": "src/hooks/useUser.ts",
      "operations": [
        {"op": "add_error_handling", "around": "fetchUser()", "catch": "setError(e)"}
      ]
    }
  ],
  "verify_after": true,
  "rollback_on_failure": true
}

// Result:
{
  "status": "committed",
  "files_modified": 3,
  "verification": {
    "parse": "pass",
    "lint": "pass",
    "types": "pass",
    "affected_tests": "3/3 pass"
  }
}
// If verification fails → all changes rolled back automatically
```

**Key properties:**
- **Atomic**: All files change together or none do
- **Verified**: The tool runs verification after all changes, before committing
- **Rollback**: If any verification fails, all changes are reverted
- **Coordinated**: Changes that span files (like making a function async and updating all callers) are expressed as a single unit

**Built on:** Git's staging area for rollback, Oxc for AST operations, verify-pipe for post-change validation.

---

### Tool 8: `context-budget` — Intelligent Context Allocation

**The problem:** AI agents have a fixed context window. Current agents waste tokens reading files that turn out to be irrelevant, reading entire files when only a function matters, and re-reading files they've already seen.

**The idea:** A context management layer that acts as a **smart cache and allocator** between the codebase and the LLM.

```jsonc
// Agent declares its task and budget:
{
  "tool": "context-budget",
  "task": "Add rate limiting to the /api/users endpoint",
  "token_budget": 8000,
  "priority": "implementation"
}

// Tool returns an optimized context package:
{
  "allocated_tokens": 7,842,
  "contents": [
    {
      "type": "skeleton",
      "scope": "full_project",
      "tokens": 200,
      "content": "... project structure ..."
    },
    {
      "type": "full_source",
      "file": "src/api/users.ts",
      "tokens": 1200,
      "reason": "primary modification target"
    },
    {
      "type": "signatures_only",
      "file": "src/middleware/index.ts",
      "tokens": 150,
      "reason": "existing middleware patterns to follow"
    },
    {
      "type": "full_source",
      "file": "src/middleware/auth.ts",
      "tokens": 800,
      "reason": "closest example of middleware implementation"
    },
    {
      "type": "type_definitions",
      "files": ["src/types/api.ts"],
      "tokens": 300,
      "reason": "type constraints for request/response"
    },
    {
      "type": "test_examples",
      "file": "src/__tests__/auth.test.ts",
      "tokens": 600,
      "reason": "test pattern to replicate for new middleware"
    },
    {
      "type": "dependency_info",
      "tokens": 200,
      "content": "Available packages: express-rate-limit@7.1.0 (already installed)"
    },
    {
      "type": "similar_implementations",
      "tokens": 400,
      "content": "... rate limiting patterns from codebase history ..."
    }
  ]
}
```

**How it works:**

1. **Task analysis**: Parse the task description to identify relevant code areas
2. **Dependency graph traversal**: From the target file(s), walk imports/exports to find related code
3. **Token budgeting**: Allocate tokens across files using a priority scheme:
   - Full source for files being modified
   - Signatures only for nearby files
   - Types only for distant dependencies
   - Examples from similar existing code
4. **Deduplication**: Don't include code the agent has already seen in this session
5. **Compression**: Use tree-sitter to strip unnecessary code bodies, keep only signatures

**Built on:** tree-sitter for AST extraction, Oxc resolver for dependency graphs, embedding-based similarity search for finding relevant examples.

---

### Tool 9: `error-doctor` — Structured Error Diagnosis with Fix Suggestions

**The problem:** When code fails (compilation error, test failure, runtime crash), the error output is designed for humans to read and reason about. An AI agent must: (1) parse the error text, (2) understand what went wrong, (3) identify the fix, (4) locate the right file/line, (5) generate the fix. Steps 1-4 are pure overhead.

**The idea:** A tool that intercepts all error output, parses it into structured form, identifies the root cause, and suggests concrete AST operations to fix it.

```jsonc
// Raw TypeScript error (what the agent currently gets — ~300 tokens of noise):
//
// src/api/users.ts:14:5 - error TS2345: Argument of type 'string | undefined'
// is not assignable to parameter of type 'string'.
//   Type 'undefined' is not assignable to type 'string'.
//
// 14     return getUser(userId);
//                       ~~~~~~
//
// src/api/users.ts:8:3
//  8   const userId = req.params.id;
//                     ~~~~~~~~~~~~~~
// 'userId' is declared here.

// What error-doctor returns (~80 tokens):
{
  "error_type": "type_mismatch",
  "severity": "error",
  "location": {"file": "src/api/users.ts", "line": 14, "col": 5},
  "expected": "string",
  "actual": "string | undefined",
  "symbol": "userId",
  "root_cause": "req.params.id can be undefined",
  "fixes": [
    {
      "strategy": "narrow_type",
      "description": "Add undefined check before call",
      "operation": {
        "op": "wrap_in_guard",
        "variable": "userId",
        "check": "!== undefined",
        "else": "return res.status(400).json({error: 'userId required'})"
      },
      "confidence": 0.9
    },
    {
      "strategy": "assert_type",
      "description": "Non-null assertion (less safe)",
      "operation": {
        "op": "add_non_null_assertion",
        "expression": "userId",
        "line": 14
      },
      "confidence": 0.3
    }
  ]
}
```

**Built on:** Pattern matching on tsc/Biome error codes (they're enumerable — TS2345 always means the same thing), Oxc for generating fix operations.

---

## Part 4: The Meta-Architecture — How These Tools Compose

These nine tools aren't standalone. They form a **pipeline** that fundamentally changes how an AI agent interacts with a codebase:

```
┌─────────────────────────────────────────────────────────────────────┐
│                     AGENTIC TOOLING LAYER                           │
│                                                                     │
│  ┌──────────────┐                                                   │
│  │ LLM Agent     │                                                   │
│  │ (Claude, GPT, │                                                   │
│  │  Codex, etc.) │                                                   │
│  └──────┬───────┘                                                   │
│         │                                                           │
│         │  1. "I need to add rate limiting to /api/users"            │
│         ▼                                                           │
│  ┌──────────────────┐                                               │
│  │ context-budget     │  → Assembles optimal context for the task    │
│  └────────┬─────────┘                                               │
│           │  Returns: skeleton + target files + examples + types     │
│           ▼                                                         │
│  ┌──────────────────┐                                               │
│  │ type-scaffold     │  → Extracts constraints for the code to write │
│  └────────┬─────────┘                                               │
│           │  Returns: type requirements, available APIs, error cases │
│           ▼                                                         │
│  ┌──────────────────┐                                               │
│  │ impact-graph      │  → Shows what will be affected by changes     │
│  └────────┬─────────┘                                               │
│           │  Returns: dependent files, affected tests                │
│           ▼                                                         │
│  ┌──────────────────┐                                               │
│  │ LLM REASONING    │  → Agent plans the changes                    │
│  │ (minimal tokens   │     (with full context and constraints)       │
│  │  because context  │                                               │
│  │  was pre-optimized│                                               │
│  └────────┬─────────┘                                               │
│           │  Outputs: structured AST operations                      │
│           ▼                                                         │
│  ┌──────────────────┐                                               │
│  │ ast-surgeon       │  → Applies operations (correct by construction)│
│  │ + batch-edit      │  → Atomically across all affected files       │
│  └────────┬─────────┘                                               │
│           │                                                         │
│           ▼                                                         │
│  ┌──────────────────┐                                               │
│  │ verify-pipe       │  → Lint + typecheck + test (affected only)    │
│  └────────┬─────────┘                                               │
│           │                                                         │
│     ┌─────┴─────┐                                                   │
│     │           │                                                   │
│   PASS        FAIL                                                  │
│     │           │                                                   │
│     ▼           ▼                                                   │
│   Done    ┌──────────────┐                                          │
│           │ error-doctor  │  → Structured diagnosis + fix suggestions│
│           └──────┬───────┘                                          │
│                  │                                                   │
│                  ▼                                                   │
│           Agent applies suggested fix (often a single AST op)       │
│           → Back to verify-pipe                                     │
│                                                                     │
│   Throughout: semantic-diff tracks all changes for review            │
│   Throughout: codebase-skeleton keeps the agent oriented             │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Part 5: Why Rust Is the Right Foundation

The existing Rust web tooling ecosystem is **already 80% of the infrastructure** needed to build these agentic tools:

| Agentic Tool | Existing Rust Infrastructure to Build On |
|-------------|------------------------------------------|
| `ast-surgeon` | Oxc parser + AST manipulation (arena-allocated, fast) |
| `codebase-skeleton` | tree-sitter (multi-language) + Oxc resolver (dependency graph) |
| `verify-pipe` | Biome (lint) + Oxc (parse) + test runner integration |
| `semantic-diff` | Oxc parser (before/after ASTs) + tree diff algorithms |
| `impact-graph` | Oxc resolver (30x faster than enhanced-resolve) + dependency graph |
| `type-scaffold` | Biome's type inference engine + TypeScript declaration parsing |
| `batch-edit` | Oxc AST operations + Git (rollback) |
| `context-budget` | tree-sitter (compression) + Oxc (signatures) + graph algorithms |
| `error-doctor` | Biome/tsc error codes (enumerable) + Oxc (fix generation) |

**The key insight:** These Rust tools were built to make **compilation faster** for humans. But the same infrastructure — fast parsers, efficient ASTs, dependency graphs, type information — is exactly what's needed to make **AI-driven development more token-efficient**.

The arena-allocated AST in Oxc that was designed for fast linting is also perfect for fast impact analysis. The 30x-faster resolver was built for bundlers but is equally valuable for building dependency graphs for context allocation. Lightning CSS's parser was built for transformation but could power CSS-aware code skeletons.

**The Rust speed advantage matters differently for agents than for humans.** For humans, fast tools mean shorter wait times (2s vs 30s). For agents, fast tools mean more verification cycles per token budget. If verify-pipe takes 50ms instead of 5s, the agent can verify-fix-verify 100 times in the time it would take to verify once with JS tools. This fundamentally changes the error correction economics.

---

## Part 6: Open Questions and Research Directions

### 6.1 Should LLMs Generate AST Operations or Text?

The `ast-surgeon` proposal assumes LLMs can effectively generate structured AST operations. But current LLMs are trained overwhelmingly on **text** code — not AST representations. Would an LLM fine-tuned on AST operations outperform one generating raw text?

**Hypothesis:** Yes, because:
- AST operations have a smaller action space (fewer valid next tokens)
- They eliminate syntax errors entirely
- They compose better for multi-file changes
- But: they require training data in AST-operation format, which doesn't exist at scale yet

**Research needed:** Fine-tune a code LLM on paired (natural language intent → AST operations) data and compare error rates against text generation.

### 6.2 What's the Optimal Code Representation for LLMs?

Raw source code is not the most token-efficient representation. ASTs are too verbose. Type signatures lose implementation details. What's the sweet spot?

**Candidates:**
- **Skeleton + holes**: Show structure with `...` for irrelevant implementation details
- **Semantic summaries**: "This function validates user input, calls the API, and handles three error cases"
- **Hybrid**: Signatures for distant code, full source for nearby code, summaries for tangential code
- **Compressed AST**: A linearized AST that preserves structure but removes syntactic noise

### 6.3 Can We Build a "Code Verifier" as Fast as a Linter?

Full type checking (tsc) takes seconds to minutes. Linting (Biome) takes milliseconds. For agentic workflows, we need verification at linting speeds. Can we build a "good enough" type checker that catches 90% of type errors in 50ms instead of 100% in 5s?

**Biome's inference engine is a step toward this** — it infers types without running tsc. But it doesn't cover the full type system. How far can we push this?

### 6.4 The Training Data Feedback Loop

If AI agents increasingly write code, and AI training data comes from code repositories, then:
- AI-generated code trains the next generation of AI
- Tool-assisted code (AST operations, structured fixes) would be higher quality
- The tools themselves become part of the training pipeline

**Implication:** The agentic tooling layer doesn't just help agents write code — it improves the quality of training data for future models.

---

## Part 7: Competitive Landscape — Who Might Build This?

| Player | Position | Likelihood |
|--------|----------|------------|
| **VoidZero** (Evan You) | Has Oxc parser, resolver, transformer. Building unified toolchain. Adding agent-oriented APIs would be natural extension. | High — they have the infrastructure and vision |
| **Anthropic** | Builds Claude Code (agentic CLI). Deeply understands agent workflow bottlenecks. | High — they experience the pain firsthand |
| **Vercel** | Has SWC + Turbopack + invested in Biome. Plus v0 (AI code generation). | Medium — focused on Next.js ecosystem |
| **Cursor/Anysphere** | Leading AI code editor. Understands agent UX. | Medium — more focused on editor layer |
| **ByteDance** | Has Rspack infrastructure. Massive internal AI coding usage. | Medium — focused on webpack migration |
| **Factory.ai** | Already building "context engineering" infrastructure for AI agents. Coined "drafter" agents. | High — this is their explicit mission |
| **Open source community** | Could build this as modular tools composed via MCP. | High potential — MCP provides the protocol layer |

### The MCP Angle

The **Model Context Protocol (MCP)** — which exploded in 2025 — is the natural distribution mechanism for agentic tools. Each tool described above could be an MCP server:

```
Agent ←→ MCP Protocol ←→ ast-surgeon MCP server
Agent ←→ MCP Protocol ←→ verify-pipe MCP server
Agent ←→ MCP Protocol ←→ impact-graph MCP server
Agent ←→ MCP Protocol ←→ codebase-skeleton MCP server
```

This means these tools don't need to be monolithic. They can be built independently, by different teams, and composed by any agent framework that speaks MCP.

---

## Summary: The Thesis in One Paragraph

The $100B+ JavaScript ecosystem spent a decade optimizing developer tools for human cognition — readable errors, visual feedback, interactive workflows. Rust-based tools (SWC, Oxc, Biome, Rspack, Rolldown) made these tools 10-100x faster, but didn't change the fundamental assumption: a human is driving. As AI agents become the primary code authors (Anthropic's 2026 report shows agents generating >25% of code at some companies), we need a parallel tooling layer that optimizes for **token efficiency** (compact structured I/O), **verification-first workflows** (validate before emit), **semantic operations** (AST manipulation, not text generation), and **batch atomicity** (multi-file transactions). The Rust infrastructure already exists — fast parsers, efficient ASTs, type inference, dependency graphs. The missing piece is the **orchestration layer** that repackages this infrastructure for machine consumption instead of human consumption.

---

## Sources

- [The New Stack: 5 Key Trends Shaping Agentic Development in 2026](https://thenewstack.io/5-key-trends-shaping-agentic-development-in-2026/)
- [The New Stack: AI Coding Tools in 2025 - Welcome to the Agentic CLI Era](https://thenewstack.io/ai-coding-tools-in-2025-welcome-to-the-agentic-cli-era/)
- [Anthropic: 2026 Agentic Coding Trends Report](https://resources.anthropic.com/hubfs/2026%20Agentic%20Coding%20Trends%20Report.pdf)
- [Factory.ai: The Context Window Problem](https://factory.ai/news/context-window-problem)
- [TU Wien: Reducing Token Usage of Software Engineering Agents](https://repositum.tuwien.at/bitstream/20.500.12708/224666/1/Hrubec%20Nicolas%20-%202025%20-%20Reducing%20Token%20Usage%20of%20Software%20Engineering%20Agents.pdf)
- [LongCodeZip: Compress Long Context for Code Language Models](https://arxiv.org/html/2510.00446v1)
- [Repomix: Pack Your Codebase into AI-Friendly Formats](https://repomix.com/)
- [Medium: Semantic Code Indexing with AST and Tree-sitter for AI Agents](https://medium.com/@email2dineshkuppan/semantic-code-indexing-with-ast-and-tree-sitter-for-ai-agents-part-1-of-3-eb5237ba687a)
- [SemanticDiff: Language-Aware Diff](https://semanticdiff.com/)
- [HN: CodeRLM — Tree-sitter-backed Code Indexing for LLM Agents](https://news.ycombinator.com/item?id=46974515)
- [arxiv: AI Agentic Programming: A Survey](https://arxiv.org/html/2508.11126v1)
- [Mike Mason: AI Coding Agents in 2026 — Coherence Through Orchestration](https://mikemason.ca/writing/ai-coding-agents-jan-2026/)
- [Faros AI: Best AI Coding Agents for 2026](https://www.faros.ai/blog/best-ai-coding-agents-2026)
- [Oxc Official Site](https://oxc.rs/)
- [Biome Roadmap 2026](https://biomejs.dev/blog/roadmap-2026/)
- [VoidZero: Announcing VoidZero Inc](https://voidzero.dev/posts/announcing-voidzero-inc)
