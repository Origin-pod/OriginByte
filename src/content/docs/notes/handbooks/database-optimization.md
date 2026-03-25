---
title: "Chapter 3 — Database Optimization (Deep Dive)"
---

# Chapter 3 — Database Optimization (Deep Dive)

## 3.1 Query Optimization

### How a Query Executes Internally

When you send a SQL query to PostgreSQL (or MySQL), it goes through a multi-stage pipeline before a single row is touched:

```
    SQL Text
       │
       ▼
    ┌──────────────┐
    │   PARSER     │  Lexical analysis → tokens → parse tree (AST)
    │              │  Checks syntax: SELECT, FROM, WHERE are valid
    │              │  Does NOT check if tables/columns exist yet
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │   ANALYZER   │  Resolves names: does table "users" exist?
    │              │  Does column "email" exist in "users"?
    │              │  Type checking: comparing int to varchar?
    │              │  Outputs a "query tree"
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │   REWRITER   │  Applies rules: view expansion, RLS policies
    │              │  Transforms query tree
    └──────┬───────┘
           │
           ▼
    ┌──────────────────────────────────────────────────┐
    │                    PLANNER / OPTIMIZER            │
    │                                                  │
    │  This is the brain of the database.              │
    │                                                  │
    │  It considers:                                   │
    │  - Which indexes to use (or not)                 │
    │  - Join order (for multi-table queries)          │
    │  - Join algorithm (nested loop, hash, merge)     │
    │  - Whether to use sequential or index scan       │
    │  - Whether to materialize subqueries             │
    │  - Sort strategy (in-memory vs disk)             │
    │                                                  │
    │  It uses STATISTICS about your data:             │
    │  - Table row counts                              │
    │  - Column value distribution (histograms)        │
    │  - Most common values                            │
    │  - Null fraction                                 │
    │  - Correlation (physical vs logical ordering)    │
    │                                                  │
    │  It assigns a COST to each possible plan         │
    │  and picks the cheapest one.                     │
    │                                                  │
    │  Cost = (disk pages read × seq_page_cost)        │
    │       + (rows processed × cpu_tuple_cost)        │
    │       + (index entries × cpu_index_tuple_cost)   │
    │       + (operator evaluations × cpu_operator_cost)│
    └──────────────────┬───────────────────────────────┘
                       │
                       ▼
    ┌──────────────┐
    │   EXECUTOR   │  Follows the plan step by step
    │              │  Fetches pages from disk/buffer cache
    │              │  Applies filters, joins, sorts
    │              │  Returns result set to client
    └──────────────┘
```

### Why This Matters

The planner makes decisions based on **statistics**. If your statistics are stale (you haven't run `ANALYZE`), the planner will make bad decisions — like choosing a sequential scan when an index scan would be 100x faster, or choosing a nested loop join when a hash join would be appropriate.

**Critical Rule:** Always keep statistics up to date. PostgreSQL's autovacuum daemon runs `ANALYZE` periodically, but after bulk loads or major data changes, run it manually:

```sql
ANALYZE tablename;           -- Update stats for one table
ANALYZE;                     -- Update stats for entire database
```

---

## 3.2 Index Types — Deep Dive

### B-tree Index (Default)

The B-tree is the workhorse index of relational databases. Understanding its internal structure is essential.

```
    B-tree Structure (simplified, order = 3)

                        ┌───────────┐
                        │  [30, 60] │           ← Root node
                        └─┬───┬───┬─┘
                          │   │   │
              ┌───────────┘   │   └───────────┐
              │               │               │
        ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
        │ [10, 20]  │  │ [40, 50]  │  │ [70, 80]  │  ← Internal nodes
        └─┬──┬──┬───┘  └─┬──┬──┬───┘  └─┬──┬──┬───┘
          │  │  │         │  │  │         │  │  │
          ▼  ▼  ▼         ▼  ▼  ▼         ▼  ▼  ▼
        Leaf nodes containing actual data pointers (ctid in PostgreSQL)

    Leaf nodes are linked together as a doubly-linked list
    for efficient range scans:

    [5,8,10] ↔ [12,15,20] ↔ [25,28,30] ↔ [35,38,40] ↔ ...
```

**How a B-tree lookup works:**
1. Start at root node
2. Binary search within the node to find the right child pointer
3. Follow pointer to next level
4. Repeat until you reach a leaf node
5. Leaf node contains the heap tuple pointer (ctid)

**Time complexity:** O(log_B(N)) where B is the branching factor (typically 200-500 for 8KB pages)

For a table with 10 million rows and branching factor 300:
- log_300(10,000,000) ≈ 2.8 → only 3 page reads to find any row

**When B-tree excels:**
- Equality lookups: `WHERE id = 42`
- Range queries: `WHERE created_at > '2024-01-01'`
- Sorting: `ORDER BY created_at` (already sorted in index)
- Min/Max: `SELECT MAX(id)` (just read the rightmost leaf)

**When B-tree struggles:**
- Pattern matching: `WHERE name LIKE '%smith'` (leading wildcard)
- Low-cardinality columns: `WHERE is_active = true` (index not selective)
- Lots of writes: each INSERT/UPDATE/DELETE must update every index

### Hash Index

```
    Hash Index Structure:

    Key: "alice@example.com"
           │
           ▼
    hash("alice@example.com") = 0x7A3F
           │
           ▼
    Bucket 0x7A3F:
    ┌──────────────────────────────────────┐
    │  hash_value | ctid                   │
    │  0x7A3F0001 | (42, 3)  → row 42     │
    │  0x7A3F0002 | (99, 1)  → row 99     │  ← hash collision
    └──────────────────────────────────────┘
```

**When to use Hash:**
- Exact equality only: `WHERE email = 'alice@example.com'`
- O(1) amortized lookup vs O(log N) for B-tree
- In PostgreSQL 10+, hash indexes are WAL-logged and crash-safe

**When NOT to use Hash:**
- Range queries (impossible)
- Sorting (no order)
- Pattern matching
- Generally, B-tree is preferred unless you benchmark and prove hash is faster

### Composite (Multi-Column) Index

```sql
CREATE INDEX idx_user_status_created ON orders (user_id, status, created_at);
```

```
    Composite Index Structure:

    The index is sorted by (user_id, status, created_at) lexicographically:

    user_id | status    | created_at          | ctid
    --------+-----------+---------------------+------
    1       | active    | 2024-01-01 10:00:00 | (1,1)
    1       | active    | 2024-01-15 08:00:00 | (1,5)
    1       | completed | 2024-01-02 12:00:00 | (1,2)
    1       | completed | 2024-01-20 09:00:00 | (1,8)
    2       | active    | 2024-01-03 14:00:00 | (2,1)
    2       | active    | 2024-01-18 11:00:00 | (2,4)
    ...
```

**The Leftmost Prefix Rule:**

This index can be used for:
- `WHERE user_id = 1` ✓ (uses first column)
- `WHERE user_id = 1 AND status = 'active'` ✓ (uses first two columns)
- `WHERE user_id = 1 AND status = 'active' AND created_at > '2024-01-01'` ✓ (uses all three)

This index CANNOT be used for:
- `WHERE status = 'active'` ✗ (skips first column)
- `WHERE created_at > '2024-01-01'` ✗ (skips first two columns)
- `WHERE user_id = 1 AND created_at > '2024-01-01'` ⚠️ (uses first column only, created_at range requires scan within user_id results)

**Column ordering strategy:**
1. Equality columns first (WHERE col = value)
2. Range columns last (WHERE col > value)
3. High-cardinality columns before low-cardinality

### Covering Index (Index-Only Scans)

A covering index includes all columns needed by the query, so the database never needs to visit the heap (table data).

```sql
-- Query:
SELECT user_id, email FROM users WHERE user_id = 42;

-- Covering index:
CREATE INDEX idx_users_covering ON users (user_id) INCLUDE (email);
```

```
    Without covering index:
    Index Scan → find ctid → Heap Fetch → return row
                                 ↑
                          Extra I/O operation

    With covering index:
    Index-Only Scan → return from index directly
                         ↑
                  No heap access needed (much faster)
```

**PostgreSQL INCLUDE clause** (v11+): Adds non-key columns to the leaf level of the index. These columns are not part of the search key but are available for index-only scans.

### GIN Index (Generalized Inverted Index)

```
    GIN Index for Full-Text Search:

    Document 1: "PostgreSQL is a powerful database"
    Document 2: "Redis is a powerful cache"
    Document 3: "PostgreSQL supports full-text search"

    GIN Index Structure:
    ┌─────────────┬──────────────┐
    │  Term       │  Posting List│
    ├─────────────┼──────────────┤
    │  cache      │  {2}         │
    │  database   │  {1}         │
    │  full-text  │  {3}         │
    │  is         │  {1, 2}      │
    │  postgresql │  {1, 3}      │
    │  powerful   │  {1, 2}      │
    │  redis      │  {2}         │
    │  search     │  {3}         │
    │  supports   │  {3}         │
    └─────────────┴──────────────┘

    Query: WHERE to_tsvector(content) @@ to_tsquery('postgresql & powerful')
    → Intersect posting lists: {1, 3} ∩ {1, 2} = {1}
    → Return Document 1
```

**GIN use cases:**
- Full-text search (`tsvector/tsquery`)
- JSONB containment (`@>`, `?`, `?|`, `?&`)
- Array containment (`@>`, `<@`, `&&`)
- Trigram similarity (`pg_trgm`)

### Partial Index

```sql
-- Only index active orders (90% of queries only care about active ones)
CREATE INDEX idx_active_orders ON orders (created_at)
  WHERE status = 'active';
```

**Why partial indexes matter:**
- Table has 10M rows, but only 50K are active
- Full index: 10M entries, ~250MB
- Partial index: 50K entries, ~1.5MB
- The partial index is 167x smaller, fits entirely in RAM, and is faster to update

### Expression Index

```sql
-- Index on lower(email) for case-insensitive lookups
CREATE INDEX idx_users_email_lower ON users (lower(email));

-- Now this query uses the index:
SELECT * FROM users WHERE lower(email) = 'alice@example.com';

-- But this does NOT use the index:
SELECT * FROM users WHERE email = 'Alice@Example.com';
-- The expression must match exactly
```

---

## 3.3 Execution Plans — Reading EXPLAIN Output

### The Basics

```sql
EXPLAIN ANALYZE SELECT u.name, COUNT(o.id) as order_count
FROM users u
JOIN orders o ON o.user_id = u.id
WHERE u.created_at > '2024-01-01'
GROUP BY u.name
ORDER BY order_count DESC
LIMIT 10;
```

```
    QUERY PLAN
    ──────────────────────────────────────────────────────────────
    Limit  (cost=1234.56..1234.58 rows=10 width=40)
           (actual time=45.123..45.130 rows=10 loops=1)
      → Sort  (cost=1234.56..1236.78 rows=890 width=40)
              (actual time=45.120..45.125 rows=10 loops=1)
            Sort Key: (count(o.id)) DESC
            Sort Method: top-N heapsort  Memory: 25kB
            → HashAggregate  (cost=1200.00..1208.90 rows=890 width=40)
                             (actual time=44.500..44.800 rows=890 loops=1)
                  Group Key: u.name
                  → Hash Join  (cost=50.00..1150.00 rows=10000 width=36)
                               (actual time=2.500..40.000 rows=10000 loops=1)
                        Hash Cond: (o.user_id = u.id)
                        → Seq Scan on orders o
                          (cost=0.00..800.00 rows=50000 width=8)
                          (actual time=0.010..15.000 rows=50000 loops=1)
                        → Hash  (cost=45.00..45.00 rows=400 width=36)
                                (actual time=2.000..2.000 rows=400 loops=1)
                              Buckets: 1024  Memory Usage: 30kB
                              → Index Scan using idx_users_created
                                on users u
                                (cost=0.29..45.00 rows=400 width=36)
                                (actual time=0.050..1.500 rows=400 loops=1)
                                    Index Cond: (created_at > '2024-01-01')
```

### How to Read This

```
    Reading order: INSIDE OUT, BOTTOM UP

    Step 1: Index Scan on users (find users created after 2024-01-01)
            → Uses idx_users_created index
            → Returns 400 rows
            → Takes 1.5ms

    Step 2: Build Hash table from those 400 users
            → In-memory hash table (30kB)
            → Takes 2ms total

    Step 3: Sequential Scan on orders (read all 50,000 orders)
            → Full table scan (no index used)
            → Takes 15ms

    Step 4: Hash Join (match orders to users via hash table)
            → Probes hash table for each order's user_id
            → Returns 10,000 matching rows
            → Takes 40ms total

    Step 5: HashAggregate (GROUP BY u.name, COUNT)
            → Groups 10,000 rows into 890 groups
            → Takes 44.8ms

    Step 6: Sort (ORDER BY count DESC)
            → Only needs top 10 (top-N heapsort optimization)
            → Uses 25kB memory

    Step 7: Limit (return only 10 rows)
            → Total time: 45.13ms
```

### Key Metrics in EXPLAIN ANALYZE

```
    cost=START..TOTAL
    ─────────────────
    START = cost to return first row (startup cost)
    TOTAL = cost to return all rows

    Unit: arbitrary cost units (default: seq_page_cost = 1.0)
    Higher cost = more work = slower

    actual time=START..TOTAL
    ─────────────────────────
    Real wall-clock time in milliseconds

    rows=N
    ──────
    Estimated rows (in EXPLAIN) or actual rows (in EXPLAIN ANALYZE)
    BIG discrepancy between estimated and actual = bad statistics
    → Run ANALYZE on the table

    loops=N
    ───────
    How many times this node executed
    Multiply actual time × loops for true time
    (Common gotcha: nested loop with loops=10000 and time=0.1ms
     = actual total of 1000ms, not 0.1ms!)
```

### Scan Types Explained

```
    ┌────────────────────┬──────────────────────────────────────────────────┐
    │ Scan Type          │ When Used / What It Means                       │
    ├────────────────────┼──────────────────────────────────────────────────┤
    │ Seq Scan           │ Full table scan. Reads every row.               │
    │                    │ Used when: no useful index, or table is small,  │
    │                    │ or query returns >5-10% of rows                 │
    │                    │ RED FLAG if table is large and query selective   │
    ├────────────────────┼──────────────────────────────────────────────────┤
    │ Index Scan         │ Traverses B-tree, then fetches heap tuple.      │
    │                    │ Two I/O operations per row (index + heap).      │
    │                    │ Best for: selective queries (<5% of rows)       │
    ├────────────────────┼──────────────────────────────────────────────────┤
    │ Index Only Scan    │ All needed columns are in the index.            │
    │                    │ No heap access needed. Fastest scan type.       │
    │                    │ Requires: covering index + visibility map OK    │
    ├────────────────────┼──────────────────────────────────────────────────┤
    │ Bitmap Index Scan  │ Builds a bitmap of matching heap pages,         │
    │                    │ then fetches them in physical order.            │
    │                    │ Used when: too many rows for index scan,        │
    │                    │ too few for seq scan. Can combine multiple      │
    │                    │ indexes with BitmapAnd/BitmapOr.               │
    ├────────────────────┼──────────────────────────────────────────────────┤
    │ TID Scan           │ Direct access by tuple ID. Very rare.           │
    │                    │ Used for: WHERE ctid = '(page, offset)'         │
    └────────────────────┴──────────────────────────────────────────────────┘
```

---

## 3.4 Join Optimization

### Join Algorithms Deep Dive

```
    ┌─────────────────────────────────────────────────────────────┐
    │                    NESTED LOOP JOIN                         │
    │                                                             │
    │  FOR each row in outer_table:                               │
    │      FOR each row in inner_table:                           │
    │          IF join_condition matches:                          │
    │              output combined row                            │
    │                                                             │
    │  Complexity: O(N × M)                                       │
    │                                                             │
    │  Best when:                                                 │
    │  - Outer table is small (few rows)                          │
    │  - Inner table has index on join column                     │
    │  - Then it's O(N × log M), which is great for small N      │
    │                                                             │
    │  Example:                                                   │
    │  users (10 rows) JOIN orders (1M rows) ON orders.user_id    │
    │  With index on orders.user_id:                              │
    │  10 × log(1M) ≈ 10 × 20 = 200 index lookups. Very fast.   │
    └─────────────────────────────────────────────────────────────┘

    ┌─────────────────────────────────────────────────────────────┐
    │                      HASH JOIN                              │
    │                                                             │
    │  Phase 1 (Build): Read smaller table into hash table        │
    │      FOR each row in build_table:                           │
    │          hash(join_key) → insert into hash table            │
    │                                                             │
    │  Phase 2 (Probe): Scan larger table, probe hash table       │
    │      FOR each row in probe_table:                           │
    │          hash(join_key) → look up in hash table             │
    │          IF found: output combined row                      │
    │                                                             │
    │  Complexity: O(N + M) — linear!                             │
    │                                                             │
    │  Best when:                                                 │
    │  - Both tables are medium-large                             │
    │  - No useful index on join column                           │
    │  - One table fits in memory (for hash table)                │
    │                                                             │
    │  Limitation:                                                │
    │  - Only works for equality joins (=)                        │
    │  - Hash table must fit in work_mem                          │
    │  - If too large, spills to disk (batch processing)          │
    └─────────────────────────────────────────────────────────────┘

    ┌─────────────────────────────────────────────────────────────┐
    │                     MERGE JOIN                              │
    │                                                             │
    │  Prerequisite: Both inputs must be sorted on join key       │
    │                                                             │
    │  Walk through both sorted lists simultaneously:             │
    │  pointer_a = first row of sorted_table_a                    │
    │  pointer_b = first row of sorted_table_b                    │
    │  WHILE both have rows:                                      │
    │      IF a.key = b.key: output, advance both                 │
    │      ELIF a.key < b.key: advance a                          │
    │      ELSE: advance b                                        │
    │                                                             │
    │  Complexity: O(N log N + M log M) for sort + O(N + M) merge │
    │                                                             │
    │  Best when:                                                 │
    │  - Both inputs are already sorted (from index)              │
    │  - Large tables where hash table won't fit in memory        │
    │  - Range joins (>, <, BETWEEN)                              │
    └─────────────────────────────────────────────────────────────┘
```

### Join Order Matters

```sql
-- Three tables: users (1K rows), orders (1M rows), products (10K rows)
-- Query: Find users who ordered a specific product

SELECT u.name
FROM users u
JOIN orders o ON o.user_id = u.id
JOIN products p ON o.product_id = p.id
WHERE p.name = 'Widget X';
```

```
    BAD plan (wrong join order):
    ─────────────────────────────────
    users (1K) × orders (1M) = 1M intermediate rows
    Then filter by products.name = 'Widget X'
    Result: 1M rows processed to find maybe 50

    GOOD plan (optimal join order):
    ─────────────────────────────────
    products WHERE name = 'Widget X' (1 row)
    → Join with orders on product_id (500 rows)
    → Join with users on user_id (500 rows)
    Result: Only 500 rows processed at each step
```

The planner usually gets this right, but with many joins (5+), it may not explore all orderings. You can influence it with `join_collapse_limit` in PostgreSQL or query hints in MySQL.

---

## 3.5 The N+1 Query Problem

This is the single most common performance problem in applications using ORMs.

```
    THE PROBLEM
    ═══════════════════════════════════════════════════

    # Python/SQLAlchemy example (lazy loading)
    users = session.query(User).limit(100).all()    # Query 1: SELECT * FROM users LIMIT 100
    for user in users:
        print(user.orders)                          # Query 2-101: SELECT * FROM orders
                                                    #   WHERE user_id = ? (once per user!)

    Total queries: 1 + 100 = 101 queries
    If each query takes 2ms: 101 × 2ms = 202ms

    THE SOLUTION
    ═══════════════════════════════════════════════════

    # Eager loading (joinedload)
    users = session.query(User)\
        .options(joinedload(User.orders))\
        .limit(100).all()                           # 1 query with JOIN

    # SQL generated:
    # SELECT users.*, orders.*
    # FROM users
    # LEFT JOIN orders ON orders.user_id = users.id
    # LIMIT 100

    Total queries: 1
    Time: ~5ms

    # Or batch loading (subqueryload / selectinload)
    users = session.query(User)\
        .options(selectinload(User.orders))\
        .limit(100).all()

    # SQL generated:
    # Query 1: SELECT * FROM users LIMIT 100
    # Query 2: SELECT * FROM orders WHERE user_id IN (1, 2, 3, ..., 100)

    Total queries: 2
    Time: ~6ms
```

```
    N+1 Detection Checklist:
    ┌──────────────────────────────────────────────────┐
    │  ✓ Enable query logging in development           │
    │  ✓ Count queries per request (should be < 10)    │
    │  ✓ Look for repeated similar queries             │
    │  ✓ Use APM tools to trace query patterns         │
    │  ✓ Review ORM relationship loading strategies    │
    │  ✓ Use tools like django-debug-toolbar,          │
    │    bullet gem (Rails), nplusone (Python)         │
    └──────────────────────────────────────────────────┘
```

---

## 3.6 Transactions Deep Dive

### Isolation Levels Explained

```
    ┌─────────────────┬──────────────┬──────────────┬──────────────┬──────────────┐
    │ Isolation Level │ Dirty Read   │ Non-Repeatable│ Phantom Read │ Write Skew  │
    │                 │              │ Read          │              │             │
    ├─────────────────┼──────────────┼──────────────┼──────────────┼─────────────┤
    │ Read Uncommitted│ Possible     │ Possible     │ Possible     │ Possible    │
    │ Read Committed  │ Not possible │ Possible     │ Possible     │ Possible    │
    │ Repeatable Read │ Not possible │ Not possible │ Possible*    │ Possible*   │
    │ Serializable    │ Not possible │ Not possible │ Not possible │ Not possible│
    └─────────────────┴──────────────┴──────────────┴──────────────┴─────────────┘

    * PostgreSQL's Repeatable Read actually prevents phantoms via MVCC
      but write skew is still possible
```

### What Each Anomaly Looks Like

```
    DIRTY READ
    ═══════════════════════════════════════
    Transaction A                Transaction B
    ─────────────                ─────────────
    BEGIN                        BEGIN
    UPDATE balance SET            │
      amount = 0                  │
      WHERE id = 1                │
                                 SELECT amount FROM balance
                                   WHERE id = 1
                                 → Returns 0 (uncommitted!)
    ROLLBACK                      │
    (amount is still 100)        Uses 0 for business logic
                                 → WRONG! Data never existed.

    NON-REPEATABLE READ
    ═══════════════════════════════════════
    Transaction A                Transaction B
    ─────────────                ─────────────
    BEGIN                        BEGIN
    SELECT amount FROM balance    │
      WHERE id = 1                │
    → Returns 100                 │
                                 UPDATE balance SET
                                   amount = 50
                                   WHERE id = 1
                                 COMMIT
    SELECT amount FROM balance    │
      WHERE id = 1                │
    → Returns 50 (!!)            │
    Same query, different result  │

    PHANTOM READ
    ═══════════════════════════════════════
    Transaction A                Transaction B
    ─────────────                ─────────────
    BEGIN                        BEGIN
    SELECT COUNT(*) FROM orders   │
      WHERE status = 'pending'    │
    → Returns 5                   │
                                 INSERT INTO orders
                                   (status) VALUES ('pending')
                                 COMMIT
    SELECT COUNT(*) FROM orders   │
      WHERE status = 'pending'    │
    → Returns 6 (!!)             │
    New row appeared (phantom)    │

    WRITE SKEW
    ═══════════════════════════════════════
    Both doctors are on call. Hospital requires at least one on call.

    Transaction A (Dr. Alice)     Transaction B (Dr. Bob)
    ─────────────────────────     ─────────────────────────
    BEGIN                         BEGIN
    SELECT COUNT(*) FROM doctors   SELECT COUNT(*) FROM doctors
      WHERE on_call = true           WHERE on_call = true
    → Returns 2 (both on call)   → Returns 2 (both on call)

    -- "2 on call, I can leave"  -- "2 on call, I can leave"
    UPDATE doctors SET            UPDATE doctors SET
      on_call = false               on_call = false
      WHERE name = 'Alice'          WHERE name = 'Bob'
    COMMIT                        COMMIT

    → Result: ZERO doctors on call! Constraint violated.
    → Neither transaction saw the other's write.
```

### MVCC (Multi-Version Concurrency Control)

```
    How PostgreSQL MVCC Works:

    Each row has hidden system columns:
    ┌──────────────────────────────────────────────────────┐
    │  xmin  │ Transaction ID that created this row version│
    │  xmax  │ Transaction ID that deleted/updated this row│
    │  ctid  │ Physical location (page, offset)            │
    └──────────────────────────────────────────────────────┘

    Example:
    Transaction 100: INSERT INTO users (name) VALUES ('Alice')
    → Row stored with xmin=100, xmax=0

    Transaction 200: UPDATE users SET name='ALICE' WHERE name='Alice'
    → Old row: xmin=100, xmax=200 (marked as deleted by txn 200)
    → New row: xmin=200, xmax=0 (new version created by txn 200)

    Transaction 150 (started before 200, still running):
    → Sees the OLD row (xmin=100 < 150, xmax=200 > 150)
    → Does NOT see the new row (xmin=200 > 150)

    This is how PostgreSQL provides snapshot isolation without locks!

    ┌─────────────────────────────────────────────────────────┐
    │  HEAP (Table)                                           │
    │                                                         │
    │  Page 1:                                                │
    │  ┌─────────────────────────────────────────────────┐    │
    │  │ xmin=100 xmax=200  name='Alice'  (dead tuple)  │    │
    │  │ xmin=200 xmax=0    name='ALICE'  (live tuple)  │    │
    │  │ xmin=300 xmax=0    name='Bob'    (live tuple)  │    │
    │  └─────────────────────────────────────────────────┘    │
    │                                                         │
    │  Dead tuples accumulate until VACUUM reclaims them.     │
    │  This is why autovacuum is critical for PostgreSQL!     │
    └─────────────────────────────────────────────────────────┘
```

---

## 3.7 Locks and Deadlocks

### Lock Hierarchy in PostgreSQL

```
    ┌──────────────────────────────────────────────────────────────┐
    │  LOCK TYPE          │ BLOCKS              │ USE CASE          │
    ├──────────────────────────────────────────────────────────────┤
    │  AccessShareLock    │ AccessExclusive      │ SELECT            │
    │  RowShareLock       │ Exclusive,           │ SELECT FOR UPDATE │
    │                     │ AccessExclusive       │                   │
    │  RowExclusiveLock   │ Share, ShareRow,     │ INSERT, UPDATE,   │
    │                     │ Exclusive,            │ DELETE            │
    │                     │ AccessExclusive       │                   │
    │  ShareLock          │ RowExclusive,        │ CREATE INDEX      │
    │                     │ ShareRowExclusive,    │ (non-concurrent)  │
    │                     │ Exclusive,            │                   │
    │                     │ AccessExclusive       │                   │
    │  AccessExclusiveLock│ ALL other locks      │ ALTER TABLE,      │
    │                     │                       │ DROP TABLE,       │
    │                     │                       │ VACUUM FULL       │
    └──────────────────────────────────────────────────────────────┘
```

### Deadlock Scenario

```
    Transaction A                    Transaction B
    ─────────────                    ─────────────
    BEGIN                            BEGIN
    UPDATE accounts SET              UPDATE accounts SET
      balance = balance - 100          balance = balance - 50
      WHERE id = 1                     WHERE id = 2
    -- Holds lock on row 1           -- Holds lock on row 2

    UPDATE accounts SET              UPDATE accounts SET
      balance = balance + 100          balance = balance + 50
      WHERE id = 2                     WHERE id = 1
    -- Wants lock on row 2           -- Wants lock on row 1
    -- BLOCKED! B holds it           -- BLOCKED! A holds it

    ┌──────────────────────────────────────────┐
    │         DEADLOCK DETECTED!               │
    │                                          │
    │    A ──wants──▶ row 2 ──held by──▶ B     │
    │    ▲                                │     │
    │    └──held by── row 1 ◀──wants─────┘     │
    │                                          │
    │  Wait-for graph has a cycle!             │
    │  PostgreSQL detects this and kills       │
    │  one transaction (usually the younger)   │
    └──────────────────────────────────────────┘
```

**Prevention strategies:**
1. **Lock ordering:** Always lock resources in the same order (e.g., by ID ascending)
2. **Lock timeouts:** `SET lock_timeout = '5s'` — fail fast instead of waiting forever
3. **Keep transactions short:** Less time holding locks = less chance of deadlock
4. **Use optimistic locking for low-contention scenarios:**

```sql
-- Optimistic locking with version column
UPDATE products SET
  price = 29.99,
  version = version + 1
WHERE id = 42 AND version = 5;
-- If 0 rows affected → someone else updated it first → retry
```

---

## 3.8 Connection Pooling

### Why Connection Pooling Matters

```
    Creating a new database connection:
    ┌────────────────────────────────────────────────┐
    │  1. TCP handshake (SYN, SYN-ACK, ACK)    1ms  │
    │  2. TLS handshake (if SSL)               2-5ms │
    │  3. PostgreSQL auth (password/cert)       1ms  │
    │  4. Backend process forked                5ms  │
    │  5. Session initialization               1ms  │
    │  ─────────────────────────────────────────────  │
    │  Total: ~10-15ms per new connection             │
    │                                                 │
    │  If you create a new connection per request:    │
    │  1000 req/s × 15ms = 15 seconds of CPU just    │
    │  for connection setup!                          │
    └────────────────────────────────────────────────┘

    With connection pooling:
    ┌────────────────────────────────────────────────┐
    │  Connections are pre-created and reused.        │
    │  "Checkout" a connection: ~0.01ms               │
    │  Return it when done.                           │
    │                                                 │
    │  1000 req/s with pool of 20 connections:        │
    │  Each connection handles 50 req/s               │
    │  No connection creation overhead!               │
    └────────────────────────────────────────────────┘
```

### Pool Sizing Formula

```
    ┌────────────────────────────────────────────────────────────┐
    │                                                            │
    │  connections = (core_count * 2) + effective_spindle_count  │
    │                                                            │
    │  For SSD:  effective_spindle_count ≈ 1                     │
    │  For HDD:  effective_spindle_count = number of disks       │
    │                                                            │
    │  Example: 4-core server with SSD                           │
    │  connections = (4 * 2) + 1 = 9                             │
    │                                                            │
    │  WHY SO FEW?                                               │
    │  More connections ≠ more throughput!                        │
    │  Beyond the optimal point, connections compete for:         │
    │  - CPU time slices (context switching overhead)             │
    │  - Disk I/O bandwidth                                      │
    │  - Memory (each connection uses ~5-10MB)                   │
    │  - Lock contention increases                               │
    │                                                            │
    │  A database with 200 connections often performs WORSE       │
    │  than one with 20 connections under the same load!         │
    │                                                            │
    └────────────────────────────────────────────────────────────┘
```

### PgBouncer Architecture

```
    ┌─────────┐  ┌─────────┐  ┌─────────┐
    │ App 1   │  │ App 2   │  │ App 3   │     ← 300 app connections
    │ (100    │  │ (100    │  │ (100    │
    │  conns) │  │  conns) │  │  conns) │
    └────┬────┘  └────┬────┘  └────┬────┘
         │            │            │
         └────────────┼────────────┘
                      │
              ┌───────▼───────┐
              │   PgBouncer   │
              │               │
              │  Pool of 20   │     ← Only 20 real DB connections
              │  connections  │
              │               │
              │  Modes:       │
              │  - session    │     Connection assigned for session lifetime
              │  - transaction│     Connection assigned per transaction (recommended)
              │  - statement  │     Connection assigned per statement (limited)
              └───────┬───────┘
                      │
              ┌───────▼───────┐
              │  PostgreSQL   │
              │  (max 20      │
              │   backends)   │
              └───────────────┘

    300 application connections multiplexed onto 20 database connections.
    15x reduction in database load!
```

---

## 3.9 Caching Strategies

### Cache-Aside (Lazy Loading)

```
    ┌─────────────┐     1. GET user:42     ┌──────────┐
    │ Application │ ──────────────────────▶ │  Cache   │
    │             │ ◀────────────────────── │ (Redis)  │
    │             │     2. Cache MISS       │          │
    │             │                         └──────────┘
    │             │     3. SELECT * FROM    ┌──────────┐
    │             │        users WHERE      │ Database │
    │             │        id = 42          │          │
    │             │ ──────────────────────▶ │          │
    │             │ ◀────────────────────── │          │
    │             │     4. Return data      └──────────┘
    │             │
    │             │     5. SET user:42      ┌──────────┐
    │             │        {data} EX 300    │  Cache   │
    │             │ ──────────────────────▶ │ (Redis)  │
    └─────────────┘                         └──────────┘

    Next request:
    ┌─────────────┐     1. GET user:42     ┌──────────┐
    │ Application │ ──────────────────────▶ │  Cache   │
    │             │ ◀────────────────────── │ (Redis)  │
    │             │     2. Cache HIT!       │          │
    │             │     Return cached data  └──────────┘
    │             │
    │             │     Database not touched!
    └─────────────┘
```

**Pros:** Simple, only caches data that is actually requested
**Cons:** First request always slow (cache miss), stale data if DB updated directly

### Write-Through

```
    ┌─────────────┐  1. Write request    ┌──────────┐
    │ Application │ ──────────────────▶  │  Cache   │
    │             │                      │ (Redis)  │
    │             │  2. Write to DB      └────┬─────┘
    │             │                           │
    │             │                      ┌────▼─────┐
    │             │                      │ Database │
    │             │ ◀──────────────────  └──────────┘
    │             │  3. Confirm write
    └─────────────┘

    Cache is ALWAYS in sync with DB.
    Every write goes through cache first.
```

**Pros:** Cache always consistent with DB
**Cons:** Write latency increased (two writes), cache may store data that's never read

### Cache Stampede Prevention

```
    THE PROBLEM:
    ═══════════════════════════════════════════════════════════
    A popular cache key expires.
    1000 concurrent requests all get a cache miss simultaneously.
    All 1000 hit the database at once.
    Database collapses under load.

    SOLUTION 1: Locking (single-flight)
    ═══════════════════════════════════════════════════════════
    Request 1: Cache miss → Acquire lock → Query DB → Populate cache → Release lock
    Request 2: Cache miss → Lock taken → Wait → Cache now populated → Return cached
    Request 3: Cache miss → Lock taken → Wait → Cache now populated → Return cached

    Only ONE request hits the database.

    SOLUTION 2: Probabilistic early expiration
    ═══════════════════════════════════════════════════════════
    Each request checks: should I refresh the cache early?
    probability = exp(-time_until_expiry / beta)
    As expiry approaches, probability increases.
    One request will refresh before actual expiry,
    preventing mass stampede.

    SOLUTION 3: Background refresh
    ═══════════════════════════════════════════════════════════
    Cache TTL is set high (e.g., 1 hour).
    A background job refreshes the cache every 5 minutes.
    Cache never actually expires under normal operation.
```

---

## 3.10 Read Replicas

```
    ┌──────────────────────────────────────────────────────────┐
    │                    WRITE (PRIMARY)                        │
    │                                                          │
    │  ┌────────────┐                                          │
    │  │  Primary   │ ──── WAL Stream ────┬─────────────┐      │
    │  │  (Leader)  │                     │             │      │
    │  └────────────┘                     │             │      │
    │       ▲                             │             │      │
    │       │ Writes only                 ▼             ▼      │
    │                              ┌──────────┐  ┌──────────┐  │
    │                              │ Replica 1│  │ Replica 2│  │
    │                              │ (Reader) │  │ (Reader) │  │
    │                              └──────────┘  └──────────┘  │
    │                                    ▲             ▲       │
    │                                    │             │       │
    │                              Reads only    Reads only    │
    └──────────────────────────────────────────────────────────┘

    Application routing:
    ┌─────────────┐
    │    App      │
    │             │──── INSERT/UPDATE/DELETE ──▶ Primary
    │             │──── SELECT ────────────────▶ Replica (round-robin)
    └─────────────┘

    CAUTION: Replication lag means replicas may be
    slightly behind. A user creates a record, then
    immediately reads it from a replica → NOT FOUND!

    Solution: "Read your own writes"
    After a write, route that user's reads to the
    primary for a few seconds, then back to replicas.
```

---

## 3.11 Sharding

```
    HASH-BASED SHARDING
    ═══════════════════════════════════════════════════════

    shard_id = hash(user_id) % num_shards

    user_id = 42  → hash(42) % 4 = 2  → Shard 2
    user_id = 99  → hash(99) % 4 = 3  → Shard 3
    user_id = 100 → hash(100) % 4 = 0 → Shard 0

    ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
    │ Shard 0  │  │ Shard 1  │  │ Shard 2  │  │ Shard 3  │
    │ users:   │  │ users:   │  │ users:   │  │ users:   │
    │ 100,104  │  │ 1,5,9    │  │ 42,46    │  │ 99,103   │
    │ 108,...  │  │ 13,...   │  │ 50,...   │  │ 107,...  │
    └──────────┘  └──────────┘  └──────────┘  └──────────┘

    Problem: Adding a 5th shard requires rehashing ALL data.
    hash(42) % 5 = 2 (still shard 2)
    hash(99) % 5 = 4 (now shard 4! needs to move)

    Solution: CONSISTENT HASHING
    ═══════════════════════════════════════════════════════

    Hash ring with virtual nodes:
    ┌────────────────────────────────────────────────┐
    │                    0                           │
    │               ╱         ╲                      │
    │           S0-v1          S1-v1                 │
    │          ╱                  ╲                  │
    │      S3-v2                    S2-v1            │
    │      │                          │              │
    │      S2-v2                    S3-v1            │
    │          ╲                  ╱                  │
    │           S1-v2          S0-v2                 │
    │               ╲         ╱                      │
    │                  2^32-1                        │
    └────────────────────────────────────────────────┘

    When a shard is added, only ~1/N of keys need to move.
    Virtual nodes (v1, v2) ensure even distribution.
```

---

## 3.12 Materialized Views

```sql
-- Expensive query that aggregates millions of rows
CREATE MATERIALIZED VIEW monthly_revenue AS
SELECT
    date_trunc('month', created_at) as month,
    product_category,
    SUM(amount) as total_revenue,
    COUNT(*) as order_count
FROM orders
GROUP BY 1, 2;

-- Create index on the materialized view
CREATE INDEX idx_monthly_revenue ON monthly_revenue (month, product_category);

-- Querying is instant (reads pre-computed data)
SELECT * FROM monthly_revenue WHERE month = '2024-01-01';

-- Refresh when data changes (blocks reads during refresh)
REFRESH MATERIALIZED VIEW monthly_revenue;

-- Refresh concurrently (allows reads during refresh, requires unique index)
REFRESH MATERIALIZED VIEW CONCURRENTLY monthly_revenue;
```

```
    ┌────────────────────────────────────────────────────────┐
    │  Regular View:                                         │
    │  SELECT * FROM my_view                                 │
    │  → Runs the underlying query every time                │
    │  → Always fresh, potentially slow                      │
    │                                                        │
    │  Materialized View:                                    │
    │  SELECT * FROM my_mat_view                             │
    │  → Reads pre-computed results from stored table        │
    │  → Fast, but stale between refreshes                   │
    │                                                        │
    │  Best for:                                             │
    │  - Dashboards and reports                              │
    │  - Analytics queries over large datasets               │
    │  - Data that doesn't need real-time freshness          │
    │                                                        │
    │  Refresh strategies:                                   │
    │  - Cron job (REFRESH every 5 minutes)                  │
    │  - Trigger-based (after INSERT on source table)        │
    │  - Application-driven (refresh after batch import)     │
    └────────────────────────────────────────────────────────┘
```

---

## 3.13 Pagination: Offset vs. Keyset

```
    OFFSET PAGINATION (bad for large offsets)
    ═══════════════════════════════════════════════════════
    Page 1: SELECT * FROM orders ORDER BY id LIMIT 20 OFFSET 0;
    Page 2: SELECT * FROM orders ORDER BY id LIMIT 20 OFFSET 20;
    Page 500: SELECT * FROM orders ORDER BY id LIMIT 20 OFFSET 9980;

    Problem with OFFSET 9980:
    Database must read and discard 9,980 rows to find the 20 you want!
    Time grows linearly with page number.

    ┌────────────────────────────────────────────────┐
    │  Page 1:   2ms  (skip 0, return 20)            │
    │  Page 10:  5ms  (skip 180, return 20)          │
    │  Page 100: 50ms (skip 1980, return 20)         │
    │  Page 1000: 500ms (skip 19980, return 20)      │
    │  Page 10000: 5s+ (skip 199980, return 20)      │
    └────────────────────────────────────────────────┘

    KEYSET PAGINATION (constant time regardless of page)
    ═══════════════════════════════════════════════════════
    Page 1: SELECT * FROM orders ORDER BY id LIMIT 20;
            → Last row has id = 20

    Page 2: SELECT * FROM orders WHERE id > 20 ORDER BY id LIMIT 20;
            → Uses index, starts exactly where we left off

    Page 500: SELECT * FROM orders WHERE id > 9980 ORDER BY id LIMIT 20;
              → Same speed as page 1! Index seek directly to id > 9980

    ┌────────────────────────────────────────────────┐
    │  Page 1:     2ms                               │
    │  Page 10:    2ms                               │
    │  Page 100:   2ms                               │
    │  Page 1000:  2ms                               │
    │  Page 10000: 2ms                               │
    └────────────────────────────────────────────────┘

    Limitation: Can't jump to arbitrary page (no "go to page 500").
    Works for infinite scroll and "next page" patterns.
```

---

## 3.14 Common Anti-Patterns Summary

```
    ┌─────────────────────────────┬──────────────────────────────────────┐
    │ Anti-Pattern                │ Fix                                  │
    ├─────────────────────────────┼──────────────────────────────────────┤
    │ SELECT *                    │ Select only needed columns           │
    │ N+1 queries                 │ Eager/batch loading                  │
    │ No indexes on FK columns    │ Always index foreign keys            │
    │ OFFSET pagination           │ Use keyset pagination                │
    │ Functions on indexed cols    │ Use expression indexes               │
    │ OR conditions               │ Use UNION ALL or partial indexes     │
    │ NOT IN (subquery)           │ Use NOT EXISTS or LEFT JOIN IS NULL  │
    │ Implicit type casting       │ Match types exactly                  │
    │ Missing ANALYZE             │ Run ANALYZE after bulk changes       │
    │ Too many indexes            │ Remove unused indexes                │
    │ Long transactions           │ Keep transactions short              │
    │ No connection pooling       │ Use PgBouncer or app-level pool      │
    │ Storing large blobs in DB   │ Use object storage (S3)              │
    │ UUID v4 as primary key      │ Use UUID v7 or BIGSERIAL             │
    └─────────────────────────────┴──────────────────────────────────────┘
```
