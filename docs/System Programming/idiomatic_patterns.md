# System Programming Micro-Patterns (LLD Prep)

This document lists micro-patterns and idioms commonly used in system programming.  
Covers memory, concurrency, networking, file systems, performance, and low-level design decisions.

---

## 🧮 Memory Management Patterns

- **RAII (Resource Acquisition Is Initialization)**
  - Acquire resource in constructor, release in destructor (C++, Rust ownership model).
- **Arena / Pool Allocation**
  - Allocate memory in chunks for fast allocation/deallocation.
- **Slab Allocation**
  - Fixed-size objects from preallocated memory pools.
- **Copy-on-Write**
  - Share until modified; used in OS fork, strings, shared buffers.
- **Reference Counting**
  - Smart pointers (`shared_ptr`, intrusive counts).
- **Garbage Collection Hooks**
  - Finalizers, destructors, explicit free lists.
- **Placement New / Custom Allocators**
  - Explicit memory placement.

---

## 🔄 Concurrency & Synchronization Patterns

- **Thread Pool**
  - Reuse fixed number of worker threads for tasks.
- **Event Loop**
  - Single-threaded non-blocking IO with queue (libuv, epoll).
- **Reactor Pattern**
  - Event demultiplexing and dispatching handlers.
- **Proactor Pattern**
  - OS completes async operations, then notifies handler.
- **Producer-Consumer / Bounded Buffer**
  - Thread-safe queue with synchronization.
- **Reader-Writer Lock**
  - Multiple readers, single writer synchronization.
- **Double-Checked Locking**
  - Safe lazy initialization with minimal lock cost.
- **Spinlocks**
  - Busy-wait synchronization for short critical sections.
- **Lock-Free / Wait-Free Structures**
  - Atomic CAS, ring buffers, lock-free queues.
- **Futures & Promises**
  - Synchronization abstraction for async results.

---

## 📦 Data Storage & File I/O Patterns

- **Memory-Mapped Files (mmap)**
  - Direct mapping of file into process memory space.
- **Buffered I/O**
  - Reduce syscalls via read/write buffering.
- **Direct I/O**
  - Bypass kernel cache for performance-critical workloads.
- **Write-Ahead Logging**
  - Ensure durability by logging before applying changes.
- **Journaling**
  - Consistency pattern in file systems and databases.
- **Append-Only Log**
  - Sequential write pattern (Kafka, commit logs).
- **Zero-Copy I/O**
  - Use `sendfile`, `splice` to avoid user-space copy.

---

## 🌐 Networking Patterns

- **Connection Pooling**
  - Reuse established connections.
- **Keep-Alive / Persistent Connections**
  - Avoid reconnect cost.
- **Exponential Backoff with Jitter**
  - Retry strategy for robustness.
- **Heartbeat / Liveness Probes**
  - Detect dead peers.
- **Nagle’s Algorithm / Disable Nagle**
  - Batch small packets, or send immediately.
- **TCP Fast Open**
  - Data piggybacked on SYN.
- **UDP with Reliability Layer**
  - Retransmission, sequencing (QUIC-like).
- **Multithreaded Server Models**
  - Thread-per-connection, event-driven, hybrid.

---

## ⚡ Performance & Optimization Patterns

- **Batching**
  - Group small operations into fewer syscalls.
- **Cache Locality Optimization**
  - Struct packing, SOA vs. AOS layouts.
- **Loop Unrolling**
  - Reduce branch/jump overhead.
- **Branch Prediction Hints**
  - Use predictable branches.
- **Prefetching**
  - Software prefetch for cache warmup.
- **Hot Path vs. Cold Path Splitting**
  - Optimize frequent path, separate rare path.
- **Inline Expansion**
  - Reduce call overhead for small functions.

---

## 🔐 Reliability & Fault Tolerance Patterns

- **Watchdog Timers**
  - Detect hangs or deadlocks.
- **Checkpoint & Rollback**
  - Save state and recover on failure.
- **Redundancy / Replication**
  - Process or memory redundancy.
- **Circuit Breaker**
  - Prevent overwhelming a failing subsystem.
- **Graceful Degradation**
  - Reduce features under stress instead of failing.

---

## 🛠️ Debugging & Testing Patterns

- **Assertions**
  - Fail-fast on impossible conditions.
- **Canary Values**
  - Guard memory boundaries.
- **Sentinel Values**
  - Special markers in data structures.
- **Logging with Levels**
  - Trace, debug, info, warn, error.
- **Deterministic Replay**
  - Reproduce concurrency bugs.
- **Fuzz Testing Harness**
  - Randomized input for robustness.

---

## 🧱 Common Low-Level Design Idioms

- **Singleton (Process-Wide Resource)**
  - E.g., global logger, configuration.
- **Factory for OS Resources**
  - Abstract creation of sockets, threads, files.
- **Adapter for System Calls**
  - Wrap syscalls with higher-level abstractions.
- **Opaque Pointers**
  - Hide implementation details (`void*`, handle pattern).
- **RAII Wrappers for Syscalls**
  - Ensure `close`, `free`, `unlock` always called.
- **Callback vs. Continuation Passing**
  - Choose async control flow model.
- **Error Codes vs. Exceptions**
  - Propagation strategies in low-level APIs.

---

## 🧮 Algorithmic / Utility Micro-Patterns

- **Ring Buffer / Circular Queue**
  - Lock-free queue, streaming data.
- **Bloom Filter**
  - Space-efficient set membership test.
- **LRU / LFU Cache**
  - Caching eviction strategies.
- **Trie / Radix Trees**
  - Efficient lookup structures.
- **Skip Lists**
  - Alternative to balanced trees.
- **Hash Table with Open Addressing / Chaining**
- **Epoch-Based Reclamation**
  - Memory reclamation in concurrent systems.

---

## ✅ Senior-Level Interview Expectations

- Be able to **compare trade-offs**:
  - Event loop vs. thread pool vs. hybrid models.
  - Lock-based vs. lock-free concurrency.
  - Memory pool vs. malloc/free.
  - Zero-copy vs. buffered I/O.
- Show awareness of **performance, fault tolerance, and correctness**.
- Demonstrate **debugging and resilience techniques**.
