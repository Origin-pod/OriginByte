# Backend Go Programming Micro-Patterns (LLD Prep)

This document lists Go-specific design and implementation micro-patterns that a senior backend engineer should know.  
Focus is on concurrency, structuring code, networking, error handling, and Go idioms.

---

## 🧩 Go Language & Struct Patterns

- **Value vs. Pointer Receivers**
  - Value for immutability and small structs.
  - Pointer for mutability and large structs.
- **Interface-Oriented Design**
  - Define behavior contracts, not concrete types.
- **Empty Interface (`interface{}`)**
  - Avoid overuse, but useful for generics-like flexibility.
- **Struct Embedding**
  - Composition over inheritance.
- **Tagged Struct Fields**
  - JSON, DB, validation tags.
- **Option Pattern with Functional Options**
  - Use functions to configure struct initialization.
- **Zero Values as Defaults**
  - Ensure types work sensibly without explicit init.

---

## 🕳️ Error Handling Patterns

- **Error Wrapping (`fmt.Errorf("%w")`)**
  - Preserve context for error chains.
- **Sentinel Errors**
  - Predefined error values (`var ErrNotFound = errors.New("...")`).
- **Error Inspection**
  - `errors.Is` and `errors.As` for unwrapping.
- **Custom Error Types**
  - Include metadata (e.g., status codes).
- **Panic vs. Error**
  - Panic only for unrecoverable state; recover at boundaries.
- **Retry with Backoff**
  - Exponential with jitter for resilience.

---

## 🔄 Concurrency & Goroutine Patterns

- **Worker Pool**
  - Limit concurrent goroutines, reuse workers.
- **Pipeline Pattern**
  - Chain goroutines connected by channels.
- **Fan-Out / Fan-In**
  - Distribute work across goroutines, aggregate results.
- **Context Cancellation**
  - Use `context.Context` to propagate cancellation/deadlines.
- **Ticker / Timer**
  - Scheduled tasks with `time.Ticker` / `time.After`.
- **Semaphore via Channel**
  - Limit concurrency with buffered channels.
- **ErrGroup**
  - Coordinated goroutine lifecycles with error propagation.
- **Select Statement**
  - Multiplex channel operations.
- **Non-Blocking Channel Operations**
  - `select { case ... default: }`.

---

## 🌐 Networking & API Patterns

- **HTTP Middleware Chain**
  - Compose request handling (`net/http`, chi, echo).
- **Request Context Pattern**
  - Store request-scoped values in `context.Context`.
- **Connection Pooling**
  - Use `http.Client` properly (reuses TCP connections).
- **gRPC Interceptors**
  - Middleware for RPC calls.
- **Graceful Shutdown**
  - `http.Server.Shutdown(ctx)` to drain requests.
- **Rate Limiting**
  - Token bucket / leaky bucket via goroutines + channels.
- **Circuit Breaker / Retry**
  - With libraries like `sony/gobreaker`.
- **Idempotency Keys**
  - For safe retries in APIs.

---

## 📦 Data & Persistence Patterns

- **Repository Pattern**
  - Abstract DB queries behind interfaces.
- **Unit of Work**
  - Manage DB transaction lifecycle.
- **Connection Pooling**
  - Proper use of `database/sql` connection pools.
- **Context-Aware Queries**
  - Pass `ctx` to DB calls for cancellation/timeouts.
- **Migration Pattern**
  - Use tools like `golang-migrate` to manage schema evolution.
- **Caching**
  - In-memory (map + RWMutex), Redis, or Go LRU libs.
- **CQRS (Command Query Responsibility Segregation)**
  - Separate reads and writes for scalability.

---

## ⚡ Performance Optimization Patterns

- **Sync Pools**
  - Reuse objects with `sync.Pool`.
- **Avoiding Goroutine Leaks**
  - Ensure all goroutines exit on context cancel.
- **Zero-Copy I/O**
  - `io.Copy` streams without buffering overhead.
- **Batching**
  - Collect and process items in bulk.
- **Lock Striping**
  - Shard state to reduce contention.
- **Atomic Operations**
  - Use `sync/atomic` for counters, flags.

---

## 🔐 Security Patterns

- **Input Validation**
  - Validate struct tags (e.g., `validator` pkg).
- **Context-Aware Timeouts**
  - Prevent slowloris attacks by bounding request time.
- **Secure Config Management**
  - Load secrets from env, vaults.
- **JWT / Session Handling**
  - Secure token handling with rotation and expiry.
- **Rate Limiting & Throttling**
  - Protect endpoints from abuse.

---

## 🛠️ Testing & Debugging Patterns

- **Table-Driven Tests**
  - Use slices of structs for parameterized tests.
- **Golden Files**
  - Store expected outputs in external files.
- **Mock Interfaces**
  - Replace dependencies with fake implementations.
- **Integration Testing with `httptest.Server`**
  - Spin up test servers easily.
- **Benchmark Tests**
  - Use `testing.B` for performance tests.
- **Race Detector**
  - `go test -race` to catch concurrency issues.
- **pprof & Tracing**
  - Profiling with built-in tooling.

---

## 🧮 Utility Micro-Patterns

- **Context Keys**
  - Use custom types for keys to avoid collisions.
- **Functional Options**
  - Clean configuration instead of many constructor args.
- **Iterator via Channel**
  - Stream data with goroutines + channels.
- **Pub/Sub with Channels**
  - In-memory event bus.
- **Idempotent Initializers (`sync.Once`)**
  - Ensure single execution of initialization logic.
- **Cancellation via Context**
  - Stop goroutines cooperatively.

---

## ✅ Senior-Level Interview Expectations

- Be able to **compare trade-offs**:
  - Goroutines vs. worker pool vs. async I/O.
  - Context cancel vs. manual signaling.
  - Channels vs. locks.
  - Sentinel errors vs. wrapped errors.
- Demonstrate **idiomatic Go usage** (simplicity, clarity).
- Show awareness of **concurrency pitfalls** (deadlocks, leaks).
- Highlight **robustness** (graceful shutdown, retries, timeouts).
