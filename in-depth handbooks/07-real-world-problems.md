# Chapter 7 — Real World Production Problems

## 7.1 Slow Queries

### The Scenario
Your API endpoint that used to respond in 50ms is now taking 5 seconds. Users are complaining. Your monitoring shows p99 latency has spiked.

### How Experts Debug This

```
    STEP 1: Identify the slow query
    ═══════════════════════════════════════════════════════

    -- PostgreSQL: Find the slowest queries
    SELECT
        calls,
        mean_exec_time::numeric(10,2) as avg_ms,
        max_exec_time::numeric(10,2) as max_ms,
        total_exec_time::numeric(10,2) as total_ms,
        rows,
        query
    FROM pg_stat_statements
    ORDER BY mean_exec_time DESC
    LIMIT 10;

    -- Or check the slow query log:
    -- postgresql.conf: log_min_duration_statement = 1000  (log queries > 1s)


    STEP 2: Analyze the execution plan
    ═══════════════════════════════════════════════════════

    EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT)
    SELECT o.*, u.name
    FROM orders o
    JOIN users u ON u.id = o.user_id
    WHERE o.status = 'pending'
    AND o.created_at > NOW() - INTERVAL '30 days'
    ORDER BY o.created_at DESC
    LIMIT 20;

    Red flags to look for:
    ┌──────────────────────────────────────────────────────────┐
    │  ⚠ Seq Scan on large table (should be Index Scan)        │
    │  ⚠ actual rows >> estimated rows (stale statistics)      │
    │  ⚠ Nested Loop with loops=100000 (missing index)         │
    │  ⚠ Sort Method: external merge Disk (work_mem too low)   │
    │  ⚠ Buffers: read=50000 (too much disk I/O)               │
    │  ⚠ Filter: Rows Removed by Filter: 999000 (poor index)  │
    └──────────────────────────────────────────────────────────┘


    STEP 3: Common fixes
    ═══════════════════════════════════════════════════════

    Fix 1: Missing index
    CREATE INDEX CONCURRENTLY idx_orders_status_created
      ON orders (status, created_at DESC);
    -- CONCURRENTLY: doesn't lock the table during creation

    Fix 2: Stale statistics
    ANALYZE orders;

    Fix 3: Query rewrite
    -- Before (slow): SELECT * with OR conditions
    SELECT * FROM orders WHERE status = 'pending' OR status = 'processing';

    -- After (fast): UNION ALL with index-friendly conditions
    SELECT * FROM orders WHERE status = 'pending'
    UNION ALL
    SELECT * FROM orders WHERE status = 'processing';

    Fix 4: Increase work_mem for sorting
    SET work_mem = '256MB';  -- Per-operation, not per-connection
    -- Only for the specific session/query, don't set globally too high
```

---

## 7.2 Database Overload (Connection Exhaustion)

### The Scenario
Application starts throwing "too many connections" errors. New requests fail with connection timeout. Database CPU at 100%.

```
    DIAGNOSIS
    ═══════════════════════════════════════════════════════

    -- Check current connections
    SELECT count(*), state, wait_event_type
    FROM pg_stat_activity
    GROUP BY state, wait_event_type
    ORDER BY count DESC;

    Typical output during overload:
    count | state              | wait_event_type
    ------+--------------------+------------------
     180  | active             | Lock            ← Blocked on locks!
      45  | idle in transaction| Client          ← App holding txns open
      20  | idle               | Client          ← Actually available
       5  | active             | IO              ← Doing real work

    Root causes:
    ┌──────────────────────────────────────────────────────────┐
    │  1. No connection pooling                                │
    │     → Each request creates a new connection               │
    │     → 500 concurrent requests = 500 connections           │
    │     → max_connections default is 100                     │
    │     Fix: Add PgBouncer, set pool size to 20-50           │
    │                                                          │
    │  2. "Idle in transaction" connections                     │
    │     → Application opens transaction but doesn't commit   │
    │     → Often: exception in code skips COMMIT/ROLLBACK     │
    │     → Connection is held hostage                         │
    │     Fix: Set idle_in_transaction_session_timeout = '30s' │
    │     Fix: Use context managers (with db.transaction():)   │
    │                                                          │
    │  3. Long-running queries holding locks                   │
    │     → One slow query locks a row/table                   │
    │     → 200 other queries wait for the lock                │
    │     → Connection pool fills up with waiting queries      │
    │     Fix: Set statement_timeout = '30s'                   │
    │     Fix: Optimize the slow query                         │
    │                                                          │
    │  4. Connection leak in application                       │
    │     → Connections checked out from pool but never returned│
    │     → Pool empties over time                             │
    │     Fix: Use connection pool leak detection               │
    │     Fix: Always return connections in finally block       │
    └──────────────────────────────────────────────────────────┘

    EMERGENCY RESPONSE
    ═══════════════════════════════════════════════════════

    -- Kill idle-in-transaction connections older than 5 minutes
    SELECT pg_terminate_backend(pid)
    FROM pg_stat_activity
    WHERE state = 'idle in transaction'
    AND state_change < NOW() - INTERVAL '5 minutes';

    -- Kill long-running queries
    SELECT pg_terminate_backend(pid)
    FROM pg_stat_activity
    WHERE state = 'active'
    AND query_start < NOW() - INTERVAL '2 minutes'
    AND query NOT LIKE '%pg_stat_activity%';
```

---

## 7.3 Memory Leaks

### The Scenario
Application memory usage grows steadily over days. Eventually OOM killer terminates the process. Restarting temporarily fixes it.

```
    COMMON CAUSES IN BACKEND APPLICATIONS
    ═══════════════════════════════════════════════════════

    1. Unbounded in-memory cache
    ─────────────────────────────
    # BAD: Cache grows forever
    cache = {}
    def get_user(id):
        if id not in cache:
            cache[id] = db.query(f"SELECT * FROM users WHERE id = {id}")
        return cache[id]
    # After 1M unique users: cache consumes GBs of RAM

    # FIX: Use bounded cache with eviction
    from functools import lru_cache
    @lru_cache(maxsize=10000)  # Max 10K entries, LRU eviction
    def get_user(id):
        return db.query("SELECT * FROM users WHERE id = %s", (id,))


    2. Unclosed database connections / cursors
    ─────────────────────────────────────────
    # BAD: Connection never returned to pool
    def get_data():
        conn = pool.getconn()
        cursor = conn.cursor()
        cursor.execute("SELECT * FROM big_table")
        results = cursor.fetchall()
        return results  # Connection leaked! Never returned to pool

    # FIX: Use context manager
    def get_data():
        with pool.getconn() as conn:  # Auto-returned on exit
            with conn.cursor() as cursor:  # Auto-closed
                cursor.execute("SELECT * FROM big_table")
                return cursor.fetchall()


    3. Event listeners / callbacks not cleaned up
    ──────────────────────────────────────────────
    // Node.js: Adding listeners without removing them
    app.get('/stream', (req, res) => {
        const listener = (data) => res.write(data);
        eventEmitter.on('data', listener);
        // If client disconnects, listener is never removed!
        // Each request adds another listener → memory leak

        // FIX:
        req.on('close', () => {
            eventEmitter.removeListener('data', listener);
        });
    });


    4. Large ORM object graphs
    ─────────────────────────
    # Django: Loading entire related object tree into memory
    orders = Order.objects.select_related('user', 'product',
        'product__category', 'product__manufacturer').all()
    # With 1M orders: loads millions of objects into memory

    # FIX: Use pagination and only load needed fields
    orders = Order.objects.values('id', 'total', 'user__name')\
        .filter(status='pending')[:100]


    DEBUGGING MEMORY LEAKS
    ═══════════════════════════════════════════════════════

    Python:
    - tracemalloc: Track memory allocations
    - objgraph: Visualize object reference graphs
    - memory_profiler: Line-by-line memory usage

    Node.js:
    - --inspect flag + Chrome DevTools heap snapshot
    - heapdump module
    - clinic.js (doctor, bubbleprof, flame)

    Go:
    - pprof: Built-in profiling (go tool pprof)
    - runtime.ReadMemStats()

    Java:
    - VisualVM / JConsole
    - jmap -heap <pid>
    - Eclipse MAT for heap dump analysis
```

---

## 7.4 Lock Contention

### The Scenario
Under high write load, throughput plateaus and latency spikes. Most database connections show `wait_event_type = Lock`.

```
    DIAGNOSIS
    ═══════════════════════════════════════════════════════

    -- Find blocking and blocked queries
    SELECT
        blocked.pid AS blocked_pid,
        blocked.query AS blocked_query,
        blocking.pid AS blocking_pid,
        blocking.query AS blocking_query,
        NOW() - blocked.query_start AS blocked_duration
    FROM pg_stat_activity blocked
    JOIN pg_locks bl ON bl.pid = blocked.pid
    JOIN pg_locks kl ON kl.locktype = bl.locktype
        AND kl.database IS NOT DISTINCT FROM bl.database
        AND kl.relation IS NOT DISTINCT FROM bl.relation
        AND kl.page IS NOT DISTINCT FROM bl.page
        AND kl.tuple IS NOT DISTINCT FROM bl.tuple
        AND kl.pid != bl.pid
    JOIN pg_stat_activity blocking ON kl.pid = blocking.pid
    WHERE NOT bl.granted;

    COMMON LOCK CONTENTION PATTERNS:

    Pattern 1: Hot row updates
    ──────────────────────────
    -- 1000 concurrent requests all updating the same counter:
    UPDATE product_stats SET view_count = view_count + 1 WHERE product_id = 42;
    -- Each must wait for the previous to commit

    Fix: Batch updates
    -- Increment in Redis, flush to DB periodically
    INCR product:42:views  -- Redis, lock-free, O(1)
    -- Background job every 30s:
    UPDATE product_stats SET view_count = view_count + $1 WHERE product_id = $2;

    Pattern 2: Long transactions holding locks
    ──────────────────────────────────────────
    BEGIN;
    SELECT * FROM orders WHERE id = 1 FOR UPDATE;  -- Locks row
    -- ... expensive business logic takes 30 seconds ...
    UPDATE orders SET status = 'processed' WHERE id = 1;
    COMMIT;  -- Row locked for 30 seconds!

    Fix: Minimize lock duration
    -- Do expensive work BEFORE taking the lock
    data = expensive_calculation()
    BEGIN;
    UPDATE orders SET status = 'processed', data = $1 WHERE id = 1;
    COMMIT;  -- Lock held for milliseconds

    Pattern 3: DDL operations on busy tables
    ─────────────────────────────────────────
    ALTER TABLE orders ADD COLUMN discount DECIMAL;
    -- Takes AccessExclusiveLock → blocks ALL reads and writes!
    -- On a table with millions of rows, this can take minutes.

    Fix: Use CONCURRENTLY where possible
    CREATE INDEX CONCURRENTLY idx_... ON orders (...);
    -- For column additions in PostgreSQL:
    -- Adding a column with a default is fast since PG 11
    -- (stored in catalog, not rewritten)
    ALTER TABLE orders ADD COLUMN discount DECIMAL DEFAULT 0;
```

---

## 7.5 Race Conditions

### The Scenario
Two users simultaneously purchase the last item in stock. Both succeed. You now have -1 inventory.

```
    THE RACE CONDITION
    ═══════════════════════════════════════════════════════

    Time    User A                         User B
    ────    ──────                         ──────
    T1      SELECT stock FROM products
            WHERE id = 42
            → stock = 1
    T2                                     SELECT stock FROM products
                                           WHERE id = 42
                                           → stock = 1
    T3      if stock > 0: proceed
    T4                                     if stock > 0: proceed
    T5      UPDATE products SET
            stock = stock - 1
            WHERE id = 42
            → stock = 0
    T6                                     UPDATE products SET
                                           stock = stock - 1
                                           WHERE id = 42
                                           → stock = -1 !!


    SOLUTIONS
    ═══════════════════════════════════════════════════════

    Solution 1: Atomic UPDATE with condition
    ─────────────────────────────────────────
    UPDATE products
    SET stock = stock - 1
    WHERE id = 42 AND stock > 0
    RETURNING stock;
    -- If 0 rows affected → out of stock (no separate SELECT needed)
    -- The WHERE clause and UPDATE are atomic within a single statement

    Solution 2: SELECT FOR UPDATE (pessimistic lock)
    ─────────────────────────────────────────────────
    BEGIN;
    SELECT stock FROM products WHERE id = 42 FOR UPDATE;
    -- Locks the row. Other transactions wait here.
    IF stock > 0:
        UPDATE products SET stock = stock - 1 WHERE id = 42;
    COMMIT;

    Solution 3: Optimistic locking (version column)
    ─────────────────────────────────────────────────
    -- Read current state
    SELECT stock, version FROM products WHERE id = 42;
    → stock=1, version=5

    -- Update only if version hasn't changed
    UPDATE products
    SET stock = stock - 1, version = version + 1
    WHERE id = 42 AND version = 5;
    -- If 0 rows affected → someone else modified it → retry

    Solution 4: Distributed lock (Redis)
    ─────────────────────────────────────
    -- Acquire lock
    SET lock:product:42 unique_id NX EX 10  -- NX=only if not exists, EX=10s TTL
    -- If OK → proceed with purchase
    -- If nil → someone else has lock → wait/retry
    -- After operation:
    DEL lock:product:42  -- Release (verify unique_id first!)
```

---

## 7.6 Security Breaches — Incident Response

### The Scenario
Your monitoring alerts on unusual database activity. A large volume of data is being read from the users table at 3 AM. No deployments are scheduled.

```
    INCIDENT RESPONSE PLAYBOOK
    ═══════════════════════════════════════════════════════

    PHASE 1: DETECT & CONTAIN (First 15 minutes)
    ─────────────────────────────────────────────
    1. Verify the alert is real (not false positive)
       - Check audit logs
       - Check if it's an expected batch job
       - Check who/what is making the queries

    2. Identify the compromised component
       - Which server IP is making the queries?
       - Which database user/role?
       - Which application service?

    3. Contain the breach
       - Revoke the compromised credentials immediately
         ALTER USER compromised_user WITH NOLOGIN;
       - Block the source IP at the firewall
       - If application is compromised: isolate the server
       - Do NOT shut down servers (preserve evidence)

    PHASE 2: ASSESS (Next 1-2 hours)
    ──────────────────────────────────
    4. Determine scope of access
       - What data was accessed? (audit logs)
       - How long did the attacker have access?
       - What privileges did the compromised account have?
       - Were other systems accessed? (lateral movement)

    5. Determine attack vector
       - SQL injection? (check application logs for payloads)
       - Stolen credentials? (check for credential reuse, phishing)
       - SSRF? (check for internal requests to metadata endpoints)
       - Supply chain? (check for recent dependency updates)

    PHASE 3: ERADICATE & RECOVER (Hours to days)
    ──────────────────────────────────────────────
    6. Remove attacker access
       - Rotate ALL credentials (not just compromised ones)
       - Revoke all active sessions/tokens
       - Patch the vulnerability that was exploited
       - Review and remove any backdoors (new users, cron jobs, SSH keys)

    7. Restore from clean state
       - If data was modified: restore from pre-breach backup
       - Verify data integrity
       - Re-enable services one by one with monitoring

    PHASE 4: POST-MORTEM (Within 1 week)
    ─────────────────────────────────────
    8. Document everything
       - Timeline of events
       - Root cause analysis
       - What worked and didn't work in response
       - Action items to prevent recurrence
       - No blame — focus on systemic improvements
```

---

## 7.7 Cache Stampede

### The Scenario
A popular cache key expires. Thousands of requests simultaneously miss the cache and hit the database. Database CPU spikes to 100%. Latency goes from 5ms to 30 seconds.

```
    THE STAMPEDE
    ═══════════════════════════════════════════════════════

    Time 0:00 — Cache key "homepage_products" expires
                (TTL was 5 minutes)

    Time 0:01 — 500 requests arrive simultaneously
                All get cache MISS
                All query database:
                SELECT * FROM products
                JOIN categories ON ...
                WHERE featured = true
                ORDER BY popularity DESC
                LIMIT 50;
                (This query takes 200ms)

    Time 0:01 — Database receives 500 identical heavy queries
    Time 0:05 — Database CPU at 100%, all queries slowing down
    Time 0:10 — Connection pool exhausted
    Time 0:15 — Application returns 503 errors
    Time 0:20 — First query completes, populates cache
    Time 0:21 — Remaining 499 queries still running
    Time 1:00 — System recovers

    PREVENTION STRATEGIES
    ═══════════════════════════════════════════════════════

    Strategy 1: Locking / Single-flight
    ┌──────────────────────────────────────────────────────┐
    │  Request 1: MISS → acquire lock → query DB → set cache → release lock │
    │  Request 2: MISS → lock taken → WAIT → cache now populated → return   │
    │  Request 3: MISS → lock taken → WAIT → cache now populated → return   │
    │                                                                        │
    │  Only 1 request hits the database.                                    │
    │                                                                        │
    │  Redis implementation:                                                │
    │  SET cache_lock:homepage_products unique_id NX EX 10                  │
    │  If acquired: query DB, set cache, release lock                       │
    │  If not acquired: wait 50ms, retry GET cache key                      │
    └──────────────────────────────────────────────────────┘

    Strategy 2: Stale-while-revalidate
    ┌──────────────────────────────────────────────────────┐
    │  Cache has two TTLs:                                 │
    │  - Soft TTL (5 min): triggers background refresh     │
    │  - Hard TTL (1 hour): actual expiration              │
    │                                                      │
    │  Request arrives, key is past soft TTL but not hard: │
    │  → Return stale cached data immediately              │
    │  → Trigger async background refresh                  │
    │  → Next request gets fresh data                      │
    │                                                      │
    │  Users always get a fast response (stale or fresh).  │
    │  Database never gets stampeded.                       │
    └──────────────────────────────────────────────────────┘

    Strategy 3: Probabilistic early expiration
    ┌──────────────────────────────────────────────────────┐
    │  Before cache expires, each request has a small       │
    │  probability of refreshing:                          │
    │                                                      │
    │  should_refresh = (random() < exp(-ttl_remaining /   │
    │                                     beta))           │
    │                                                      │
    │  As TTL approaches 0, probability → 1                │
    │  One request will "volunteer" to refresh before       │
    │  actual expiry, preventing mass stampede.            │
    └──────────────────────────────────────────────────────┘
```

---

## 7.8 DDoS Attacks

### The Scenario
Your API is receiving 10x normal traffic from distributed IPs. Legitimate users can't access the service.

```
    DDOS TYPES & DEFENSES
    ═══════════════════════════════════════════════════════

    TYPE 1: Volumetric (L3/L4)
    ── UDP flood, SYN flood, amplification attacks
    ── Saturates network bandwidth
    ── Defense: CDN/DDoS provider (Cloudflare, AWS Shield)
    ── You cannot handle this at application level

    TYPE 2: Protocol (L4)
    ── SYN flood, Slowloris
    ── Exhausts server connection tables
    ── Defense:
       - SYN cookies (kernel-level)
       - Connection timeouts (keep-alive limits)
       - Load balancer connection limits

    TYPE 3: Application (L7)
    ── HTTP flood, expensive API calls, login brute force
    ── Looks like legitimate traffic
    ── Most dangerous — hardest to distinguish from real users
    ── Defense:
       ┌──────────────────────────────────────────────────┐
       │  Layer 1: WAF rules                              │
       │  - Block known bad patterns                      │
       │  - Challenge suspicious IPs (CAPTCHA)            │
       │                                                  │
       │  Layer 2: Rate limiting                          │
       │  - Per IP: 100 req/min                           │
       │  - Per user: 1000 req/min                        │
       │  - Per endpoint: /login = 5 req/min              │
       │                                                  │
       │  Layer 3: Adaptive throttling                    │
       │  - Monitor overall request rate                  │
       │  - When above threshold, increase strictness     │
       │  - Prioritize authenticated users                │
       │  - Serve cached responses for GET requests       │
       │                                                  │
       │  Layer 4: Resource protection                    │
       │  - Limit request body size                       │
       │  - Timeout slow requests                         │
       │  - Queue expensive operations                    │
       │  - Circuit breaker on overloaded services        │
       └──────────────────────────────────────────────────┘
```

---

## 7.9 Hot Partitions

### The Scenario
You sharded your database by user_id. A celebrity user with 50M followers causes their shard to be overwhelmed while other shards are idle.

```
    THE PROBLEM
    ═══════════════════════════════════════════════════════

    Shard 0: 5% of traffic  (normal users)
    Shard 1: 5% of traffic  (normal users)
    Shard 2: 80% of traffic (celebrity user + followers)  ← HOT!
    Shard 3: 10% of traffic (normal users)

    SOLUTIONS
    ═══════════════════════════════════════════════════════

    Solution 1: Sub-sharding / Salting
    ──────────────────────────────────
    Instead of shard_key = user_id
    Use shard_key = user_id + random(0..N)

    Celebrity user's data spread across N sub-shards:
    user_42_0 → Shard 0
    user_42_1 → Shard 1
    user_42_2 → Shard 2
    user_42_3 → Shard 3

    Reads must query all N sub-shards and merge.
    Tradeoff: Write distribution vs read complexity.

    Solution 2: Caching layer absorbs hot reads
    ────────────────────────────────────────────
    Celebrity's profile and posts cached with longer TTL.
    Cache handles 99% of reads, DB only handles writes.

    Solution 3: Read replicas per hot shard
    ───────────────────────────────────────
    Shard 2 gets 3 read replicas, other shards get 1.
    Scale the hot shard independently.

    Solution 4: Separate hot data into dedicated infrastructure
    ──────────────────────────────────────────────────────────
    Detect "hot" entities automatically.
    Route them to a dedicated, scaled-up instance.
    Route normal entities to normal shards.
```

---

## 7.10 Token Leaks

### The Scenario
An engineer accidentally commits an API key or JWT secret to a public GitHub repository. The key is harvested by automated bots within minutes.

```
    TIMELINE OF A TOKEN LEAK
    ═══════════════════════════════════════════════════════

    T+0s:    Developer commits .env file with API keys
    T+30s:   Pushes to public GitHub repository
    T+60s:   GitHub secret scanning alerts (if enabled)
    T+90s:   Automated bots scrape GitHub for leaked secrets
    T+2min:  Attacker uses AWS keys to spin up crypto miners
    T+5min:  $10,000 in cloud charges already accruing
    T+1hr:   AWS billing alert fires
    T+2hr:   Team notices and revokes keys

    Cost: $10,000+ and a weekend of incident response

    PREVENTION
    ═══════════════════════════════════════════════════════

    Pre-commit:
    ✓ git-secrets or truffleHog as pre-commit hook
    ✓ .gitignore includes .env, *.pem, *.key, credentials.*
    ✓ IDE plugins that warn on secret patterns

    Repository:
    ✓ GitHub secret scanning (enabled by default for public repos)
    ✓ Branch protection rules (require review)
    ✓ Git history scanning (secrets in old commits are still exposed!)

    Runtime:
    ✓ Use environment variables or secret managers, never hardcoded
    ✓ Short-lived tokens (expire in minutes/hours, not months/years)
    ✓ Rotate secrets on every deploy

    Response plan:
    ✓ Immediately revoke the leaked credential (don't just remove from code)
    ✓ Audit usage (check CloudTrail/audit logs for unauthorized use)
    ✓ Force-rotate all secrets of the same type
    ✓ Scrub from git history:
      git filter-branch or BFG Repo-Cleaner
      (But assume the secret is already compromised!)
```

---

## 7.11 Debugging Checklist (Senior Engineer's Playbook)

```
    ┌──────────────────────────────────────────────────────────────┐
    │              PRODUCTION DEBUGGING PLAYBOOK                    │
    │                                                              │
    │  1. DON'T PANIC. Take a breath. Assess.                     │
    │                                                              │
    │  2. Is this a NEW problem or a RECURRING one?                │
    │     → Check for recent deployments (git log, deploy history) │
    │     → Check for infrastructure changes                       │
    │     → Check for data volume changes                          │
    │                                                              │
    │  3. SCOPE the impact:                                        │
    │     → Which users are affected? All? Some? One?              │
    │     → Which endpoints? All? Specific ones?                   │
    │     → Which regions? All? Specific DCs?                      │
    │                                                              │
    │  4. CHECK the usual suspects (in this order):                │
    │     a. Recent deploys → rollback if obvious                  │
    │     b. Database (slow queries, locks, connections)            │
    │     c. External dependencies (third-party APIs down?)        │
    │     d. Infrastructure (disk full, network issues)            │
    │     e. Traffic spike (DDoS, viral content, bot)              │
    │                                                              │
    │  5. USE observability tools:                                 │
    │     → Metrics dashboard (Grafana): overall health            │
    │     → Distributed traces (Jaeger): specific request path     │
    │     → Logs (ELK): error details                              │
    │     → Database monitoring: query performance                 │
    │                                                              │
    │  6. MITIGATE before fixing:                                  │
    │     → Can we cache it? (temporary cache for hot endpoint)    │
    │     → Can we rate limit it? (protect downstream)             │
    │     → Can we kill it? (disable non-critical feature)         │
    │     → Can we scale it? (add instances)                       │
    │                                                              │
    │  7. FIX the root cause:                                      │
    │     → Fix in development, test, then deploy                  │
    │     → Don't hotfix production directly unless critical       │
    │                                                              │
    │  8. POST-MORTEM:                                             │
    │     → What happened? Timeline.                               │
    │     → Why? Root cause.                                       │
    │     → How do we prevent it? Action items.                    │
    │     → Blameless. Focus on systems, not people.               │
    └──────────────────────────────────────────────────────────────┘
```
