---
title: "Chapter 10 — Compact Revision Notes"
---

# Chapter 10 — Compact Revision Notes

> Quick-reference notes for everything covered in this handbook.
> Use for interview prep, on-call reference, and daily work.

---

## DATABASE FUNDAMENTALS

```
ACID:
  Atomicity    — All or nothing. Transaction fully commits or fully rolls back.
  Consistency  — Data always satisfies constraints after transaction.
  Isolation    — Concurrent transactions don't see each other's uncommitted changes.
  Durability   — Once committed, data survives crashes (WAL).

CAP Theorem:
  Consistency + Availability + Partition Tolerance — pick 2.
  In practice: network partitions happen, so you choose CP or AP.
  CP: PostgreSQL, MySQL (primary)  → rejects writes during partition
  AP: Cassandra, DynamoDB          → serves stale reads during partition

Storage Engines:
  B-tree (PostgreSQL, InnoDB) — balanced tree, good for reads, random I/O on writes
  LSM-tree (RocksDB, Cassandra) — log-structured, great write throughput, read amplification
  In-memory (Redis) — fastest, limited by RAM, optional persistence
```

---

## INDEXING CHEAT SHEET

```
B-tree:     Default. Equality + range. O(log N). Most common.
Hash:       Equality only. O(1). Rarely better than B-tree in practice.
Composite:  Multi-column. Leftmost prefix rule. Equality cols first, range cols last.
Covering:   INCLUDE extra columns. Enables index-only scans. No heap access.
Partial:    WHERE clause in index definition. Smaller, faster for filtered queries.
Expression: Index on function result: CREATE INDEX ON t (lower(email)).
GIN:        Inverted index. Full-text search, JSONB, arrays.
GiST:       Geometric, spatial data, range types.

RULES:
  ✓ Always index foreign keys
  ✓ Index columns in WHERE, JOIN, ORDER BY
  ✓ Composite index column order = equality first, range last
  ✓ INCLUDE columns needed by SELECT for index-only scan
  ✗ Don't index low-cardinality columns alone (booleans)
  ✗ Don't over-index (each index slows writes)
  ✗ Don't put functions on indexed columns in WHERE (use expression index instead)
```

---

## QUERY OPTIMIZATION

```
EXPLAIN ANALYZE — always use ANALYZE to get actual timing, not just estimates.

Red flags:
  Seq Scan on large table          → Missing index
  Estimated vs actual rows differ  → Run ANALYZE (stale statistics)
  Nested Loop with loops=100000    → Missing index or bad join order
  Sort: external merge (Disk)      → Increase work_mem
  Buffers: read=50000              → Data not in cache, needs more RAM

Join algorithms:
  Nested Loop  — Small outer, indexed inner. O(N × log M).
  Hash Join    — Medium+ tables, equality join. O(N + M). Needs memory.
  Merge Join   — Pre-sorted inputs. O(N + M). Good for range joins.

Common fixes:
  N+1 problem     → Eager loading, batch loading, dataloader
  SELECT *         → Select only needed columns
  OFFSET pagination → Use keyset/cursor pagination
  OR conditions    → Rewrite as UNION ALL
  Functions in WHERE → Expression index
```

---

## TRANSACTIONS & CONCURRENCY

```
Isolation levels (PostgreSQL):
  Read Committed (default) — sees committed data at statement start
  Repeatable Read          — snapshot at transaction start, no phantom reads
  Serializable             — full isolation, detects write skew

MVCC: Each row has xmin (created by) and xmax (deleted by) transaction IDs.
  Readers don't block writers. Writers don't block readers.
  Dead tuples accumulate → VACUUM reclaims them.

Locking:
  Row lock: SELECT ... FOR UPDATE (pessimistic)
  Optimistic: version column, UPDATE WHERE version = N
  Advisory: pg_advisory_lock() for application-level coordination
  Deadlock: circular wait → PostgreSQL detects and kills one transaction

Prevention: Lock in consistent order. Keep transactions short. Use timeouts.
```

---

## REPLICATION & SHARDING

```
Replication:
  Single-leader: One primary (writes), N replicas (reads).
  Sync replica: 0 lag, slower commits. Async: <100ms lag, faster.
  Read-your-writes: Route to primary after writes for that user.
  Failover: Promote sync replica when primary fails.

Sharding:
  Hash-based: shard = hash(key) % N. Even distribution. Hard to rebalance.
  Range-based: shard by date/region. Easy range queries. Risk of hot shards.
  Consistent hashing: Only 1/N keys move when adding a shard.

  Challenges: Cross-shard queries, distributed transactions, rebalancing.
  Rule: Don't shard until you've exhausted vertical scaling + read replicas + caching.
```

---

## CACHING

```
Strategies:
  Cache-aside     — App checks cache, misses go to DB, app populates cache.
  Write-through   — App writes to cache first, cache writes to DB.
  Write-behind    — App writes to cache, cache async writes to DB (risk of loss).

Redis vs Memcached:
  Redis: Data structures, persistence, clustering, pub/sub, Lua scripting.
  Memcached: Simple key-value, multi-threaded, slightly faster for simple ops.

Problems:
  Stampede    — Lock/single-flight, probabilistic early refresh.
  Penetration — Cache "not found" results (empty marker with short TTL).
  Avalanche   — Stagger TTLs (add random jitter).
  Stale data  — Event-driven invalidation, short TTLs.

Eviction policies: LRU (most common), LFU (frequency-based), TTL.
```

---

## CONNECTION POOLING

```
Formula: connections = (cores × 2) + effective_spindle_count
  4-core SSD: (4 × 2) + 1 = 9 connections

More connections ≠ more throughput. Beyond optimal, context switching kills performance.

PgBouncer modes:
  Session:     Connection per session (least efficient, most compatible)
  Transaction: Connection per transaction (recommended, best throughput)
  Statement:   Connection per statement (most efficient, breaks multi-statement txns)

Pool exhaustion: Set timeouts. Monitor pool utilization. Alert at 80%.
```

---

## AUTHENTICATION

```
Password hashing: Argon2id > bcrypt > scrypt >> SHA-256 (never SHA for passwords)
  Argon2id: memory-hard, configurable time/memory/parallelism
  bcrypt: cost=12 minimum, ~250ms per hash

JWT:
  Header.Payload.Signature (base64url encoded, NOT encrypted)
  Access token: short-lived (5-15 min), stateless verification
  Refresh token: long-lived (days), stored server-side, revocable
  Store in httpOnly Secure SameSite=Strict cookie (not localStorage!)

  Pitfalls: Algorithm confusion, no revocation without blocklist,
            sensitive data in payload, token stored in localStorage

Sessions vs Tokens:
  Sessions: Server-side state, easy revocation, sticky/shared store needed.
  Tokens: Stateless, cross-domain, can't revoke, larger payload.

OAuth 2.0:
  Auth Code flow: For web apps (server exchanges code for tokens).
  Auth Code + PKCE: For SPAs/mobile (no client_secret needed).
  Client Credentials: Service-to-service.
  Always use state parameter (CSRF protection).
```

---

## AUTHORIZATION

```
RBAC: User → Role → Permissions. Simple, most common.
ABAC: Rules based on attributes (user.department, resource.classification, time).
ReBAC: Based on relationships (user is owner of document). Google Zanzibar model.
RLS:  PostgreSQL Row-Level Security. Database enforces access per row.

Principle of least privilege: Give minimum access needed. Always.
```

---

## COMMON ATTACKS & DEFENSES

```
SQL Injection:
  Attack: ' OR 1=1 --  (modifies query structure)
  Defense: Parameterized queries (ALWAYS). Never string concatenation.

XSS:
  Attack: <script>steal(cookie)</script> injected into page
  Defense: Output encoding, CSP header, httpOnly cookies.

CSRF:
  Attack: Hidden form on evil.com auto-submits to your-bank.com
  Defense: SameSite=Strict cookie, CSRF token, double-submit cookie.

SSRF:
  Attack: Server fetches attacker-controlled URL (e.g., metadata endpoint)
  Defense: URL allowlist, resolve IP before fetch, block private ranges.

Broken Auth:
  Attack: Credential stuffing, brute force, JWT manipulation
  Defense: Rate limiting, MFA, strong password hashing, short-lived tokens.

IDOR:
  Attack: Change /api/users/42 to /api/users/43 to access other user's data
  Defense: Authorization check on EVERY request (is this user allowed to access this resource?)
```

---

## SECURITY HEADERS

```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
Content-Security-Policy: default-src 'self'; script-src 'self' 'nonce-{random}'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
```

---

## RATE LIMITING

```
Token Bucket: Allows bursts. Refills at constant rate. Most flexible.
Sliding Window Counter: Low memory. Approximation. Good tradeoff.
Sliding Window Log: Precise. Memory-intensive (stores every timestamp).

Apply at: API Gateway level. Per IP, per user, per endpoint.
Headers: X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, Retry-After
```

---

## MONITORING & OBSERVABILITY

```
RED Method (for services):
  Rate    — Requests per second
  Errors  — Error rate (%)
  Duration — Latency (p50, p95, p99)

USE Method (for resources):
  Utilization — How busy (CPU %, disk %, connection pool %)
  Saturation  — How queued (queue depth, wait time)
  Errors      — Error count

Key DB metrics:
  Query latency p99, queries/sec, connection count, cache hit ratio,
  replication lag, lock waits, table bloat, disk I/O

Alerting: Alert on symptoms (high latency), not causes (high CPU).
  Warning: approaching threshold (p99 > 500ms)
  Critical: SLO breached (p99 > 2s) or error rate > 1%
```

---

## SYSTEM DESIGN PATTERNS

```
CQRS:        Separate read and write models. Scale independently.
Event Sourcing: Store events, not state. Rebuild state from events.
Saga:         Distributed transaction via compensating actions.
Outbox:       Write event to DB in same transaction, process async.
Circuit Breaker: Fail fast when downstream is unhealthy.
Bulkhead:     Isolate failures (separate pools per dependency).
Backpressure: Signal upstream to slow down when overwhelmed.
Idempotency:  Same request N times = same result. Use idempotency keys.
```

---

## SCALING DECISION TREE

```
Is the database the bottleneck?
├── YES: Reads are slow?
│   ├── YES: Add read replicas + caching
│   └── NO: Writes are slow?
│       ├── YES: Optimize queries → batch writes → async writes → shard
│       └── NO: Connection exhaustion?
│           └── YES: Add PgBouncer, reduce pool size
├── NO: Application CPU-bound?
│   ├── YES: Add more app server instances (horizontal scale)
│   └── NO: External dependency slow?
│       ├── YES: Circuit breaker + cache + async
│       └── NO: Profile to find actual bottleneck
```

---

## EXPERT HEURISTICS

```
1. Measure first, optimize second. Intuition is often wrong.
2. The simplest architecture that meets requirements wins.
3. Normalize first, denormalize only when measured data proves you should.
4. Design for the concurrent case, not the single-user case.
5. Trust nothing, verify everything. Especially user input.
6. Defense in depth. No single security control is sufficient.
7. Assume breach. Plan for when (not if) security is compromised.
8. More connections ≠ more throughput. Small pools outperform large ones.
9. Don't shard until you've exhausted simpler solutions.
10. The first rule of distributed systems: don't distribute unless you must.
```

---

## EMERGENCY COMMANDS

```sql
-- Kill long-running queries
SELECT pg_terminate_backend(pid) FROM pg_stat_activity
WHERE state = 'active' AND query_start < NOW() - INTERVAL '5 min'
AND pid != pg_backend_pid();

-- Kill idle-in-transaction connections
SELECT pg_terminate_backend(pid) FROM pg_stat_activity
WHERE state = 'idle in transaction' AND state_change < NOW() - INTERVAL '5 min';

-- Find blocking queries
SELECT blocked.query AS blocked_query, blocking.query AS blocking_query
FROM pg_stat_activity blocked
JOIN pg_locks bl ON bl.pid = blocked.pid AND NOT bl.granted
JOIN pg_locks kl ON kl.locktype = bl.locktype AND kl.relation = bl.relation
  AND kl.pid != bl.pid AND kl.granted
JOIN pg_stat_activity blocking ON kl.pid = blocking.pid;

-- Current connection count
SELECT count(*), state FROM pg_stat_activity GROUP BY state;

-- Top queries by total time
SELECT calls, mean_exec_time::int as avg_ms, query
FROM pg_stat_statements ORDER BY total_exec_time DESC LIMIT 10;

-- Table bloat estimate
SELECT relname, n_dead_tup, n_live_tup,
  round(n_dead_tup::numeric / NULLIF(n_live_tup, 0) * 100, 1) as dead_pct
FROM pg_stat_user_tables ORDER BY n_dead_tup DESC LIMIT 10;

-- Cache hit ratio (should be > 99%)
SELECT sum(heap_blks_hit) / (sum(heap_blks_hit) + sum(heap_blks_read)) as ratio
FROM pg_statio_user_tables;
```
