---
title: "Chapter 9 — Questions for Deep Understanding"
---

# Chapter 9 — Questions for Deep Understanding

These questions are designed to test whether you truly understand the concepts, not just memorized definitions. For each question, think about the WHY before answering.

---

## Section A: Database Internals [E]

### A1. Index Selection
You have a table `orders` with 50M rows and this query:
```sql
SELECT order_id, total FROM orders
WHERE user_id = 42 AND status = 'shipped'
AND created_at BETWEEN '2024-01-01' AND '2024-06-30'
ORDER BY created_at DESC
LIMIT 20;
```

Design the optimal index. Explain:
1. Which columns should be in the index and in what order? Why that order?
2. Should any columns be INCLUDE'd rather than part of the key?
3. Would a partial index help here? Under what conditions?
4. How would the optimal index change if the `LIMIT 20` were removed?

---

### A2. MVCC and Bloat
A PostgreSQL table receives 10,000 updates per second to a hot set of 1,000 rows. After a week, the table has grown from 100MB to 10GB even though the actual live data is still 100MB.

1. Why did this happen? Explain the MVCC mechanism that causes this.
2. What is the role of VACUUM? Why doesn't autovacuum prevent this?
3. What are the performance implications of this bloat?
4. How would you fix this without downtime?
5. How would you prevent this from happening again?

---

### A3. Isolation Level Puzzle
Consider this scenario with two concurrent transactions at `READ COMMITTED` isolation:

```
T1: BEGIN;
T1: SELECT balance FROM accounts WHERE id = 1;  -- Returns 1000
T2: BEGIN;
T2: UPDATE accounts SET balance = balance - 200 WHERE id = 1;
T2: COMMIT;
T1: UPDATE accounts SET balance = balance - 500 WHERE id = 1;
T1: COMMIT;
```

1. What is the final balance? Is it 300 or 500? Explain why.
2. Would the answer change under `REPEATABLE READ`?
3. Would the answer change under `SERIALIZABLE`?
4. How does PostgreSQL's `READ COMMITTED` handle the UPDATE in T1 after T2 has committed?

---

### A4. Join Performance
You have:
- `users` table: 1M rows, `id` is primary key
- `orders` table: 50M rows, `user_id` has a B-tree index
- Query: `SELECT u.name, COUNT(*) FROM users u JOIN orders o ON u.id = o.user_id GROUP BY u.name`

1. What join algorithm will the planner likely choose? Why?
2. If you add `WHERE u.country = 'US'` (reducing users to 300K), does the join strategy change?
3. If you add `WHERE o.created_at > '2024-01-01'` (reducing orders to 5M), does it change?
4. What happens if statistics are stale and the planner thinks `u.country = 'US'` matches only 100 rows?

---

## Section B: Security [A/E]

### B1. JWT Vulnerability Analysis
A developer implements JWT authentication with this code:
```javascript
const token = jwt.sign({ userId: user.id, role: user.role }, SECRET, { expiresIn: '30d' });
```
And verification:
```javascript
const decoded = jwt.verify(token, SECRET);
req.user = decoded;
```

List at least 5 security issues with this implementation and how to fix each one.

---

### B2. SQL Injection Bypass
An application uses this "security" measure:
```python
def sanitize(input):
    return input.replace("'", "''").replace("--", "").replace(";", "")
```

1. Can you still perform SQL injection? How?
2. What are at least 3 different bypass techniques for this type of sanitization?
3. Why is this approach fundamentally flawed regardless of how many characters you blacklist?

---

### B3. OAuth Flow Attack
A web application implements OAuth 2.0 Authorization Code flow but makes these mistakes:
1. Does not use the `state` parameter
2. Stores tokens in localStorage
3. Uses the Implicit flow for the SPA instead of Authorization Code + PKCE

For each mistake:
- Describe the specific attack that becomes possible
- Describe the attack flow step by step
- Describe the correct implementation

---

### B4. Race Condition Exploit
A money transfer API:
```python
@app.route('/transfer', methods=['POST'])
def transfer():
    amount = request.json['amount']
    from_account = Account.query.get(request.json['from'])
    to_account = Account.query.get(request.json['to'])
    if from_account.balance >= amount:
        from_account.balance -= amount
        to_account.balance += amount
        db.session.commit()
        return {'status': 'success'}
    return {'status': 'insufficient funds'}, 400
```

1. How can an attacker exploit this to transfer more money than they have?
2. Write the exact sequence of concurrent requests that demonstrates the exploit.
3. Provide 3 different approaches to fix this, with tradeoffs for each.

---

## Section C: Architecture & System Design [A/E]

### C1. Caching Strategy Design
You're building a social media timeline feature. Users see posts from people they follow, ordered by recency. Requirements:
- 100M users, average 200 followers each
- 1M new posts per hour
- Timeline must load in < 200ms
- Posts should appear within 30 seconds of creation

Design the caching strategy:
1. What do you cache? (user timelines? individual posts? both?)
2. When do you invalidate? (push vs. pull model)
3. How do you handle celebrity users with 50M followers?
4. What happens during a cache failure? (graceful degradation)
5. How do you handle the cold start problem? (new user, empty cache)

---

### C2. Scaling Puzzle
Your application has grown to 100K requests per second. The database is the bottleneck:
- 80% reads, 20% writes
- Average read query: 3ms
- Average write query: 15ms
- Current: single PostgreSQL instance, 32 cores, 256GB RAM

Design a scaling strategy:
1. What is the theoretical maximum throughput of the current setup?
2. What changes would you make first? Second? Third?
3. At what point would you consider sharding? What would you shard by?
4. How do you handle cross-shard queries?
5. What monitoring would you add to validate each scaling step?

---

### C3. Incident Design
Design the monitoring and alerting for a payment processing system:
1. What metrics would you track? (list at least 10)
2. What are the alerting thresholds?
3. What's the difference between a warning and a critical alert?
4. How would you detect a subtle data integrity issue (e.g., double-charging customers 0.1% of the time)?
5. Design the on-call runbook for the top 3 most likely incidents.

---

### C4. Consistency Challenge
You're building an e-commerce checkout system across microservices:
- Inventory Service (decrements stock)
- Payment Service (charges credit card)
- Order Service (creates order record)
- Notification Service (sends email)

1. What happens if Payment succeeds but Inventory fails?
2. Design a saga pattern for this checkout flow.
3. How do you handle the compensation transaction for each step?
4. How do you ensure idempotency (user double-clicks checkout)?
5. What isolation level and locking strategy would you use for the inventory check?

---

## Section D: Performance Deep Dive [E]

### D1. Query Optimization Puzzle
This query runs in 30 seconds:
```sql
SELECT u.name, u.email,
       (SELECT COUNT(*) FROM orders o WHERE o.user_id = u.id) as order_count,
       (SELECT MAX(o.created_at) FROM orders o WHERE o.user_id = u.id) as last_order
FROM users u
WHERE u.status = 'active'
ORDER BY order_count DESC
LIMIT 50;
```

1. Why is it slow? What is the execution pattern?
2. Rewrite it to be fast. Show at least 2 different approaches.
3. What indexes support each rewrite?
4. Which approach is better for 1M users? For 100M users?

---

### D2. Connection Pool Sizing
Your application has 10 app server instances, each with a connection pool of 50 connections, hitting a single PostgreSQL database with `max_connections = 200`.

1. What's wrong with this configuration?
2. What will happen under high load?
3. What is the optimal pool size per instance?
4. If you add PgBouncer, how does the math change?
5. How do you handle connection pool exhaustion gracefully?

---

### D3. Cache Coherency
You have two application instances reading from and writing to the same PostgreSQL database, with a shared Redis cache.

Instance A updates a user's email:
1. Writes new email to PostgreSQL
2. Deletes cache key for that user

Between steps 1 and 2, Instance B reads the user:
1. Checks Redis cache → HIT (stale data with old email)
2. Returns old email to the client

1. Is this a real problem? Under what circumstances?
2. How would you fix this?
3. What are the tradeoffs between delete-then-write vs. write-then-delete?
4. Can you design a solution with zero-inconsistency window? What's the cost?

---

## Section E: Thought Experiments [E]

### E1. Zero-Downtime Migration
You need to change a column type from `VARCHAR(100)` to `TEXT` on a table with 500M rows in PostgreSQL, with zero downtime.

1. What happens if you just run `ALTER TABLE ... ALTER COLUMN ... TYPE TEXT`?
2. Design a migration strategy that doesn't lock the table.
3. What if the column change was `INTEGER` to `BIGINT`?
4. How do you verify data integrity after the migration?

---

### E2. The Impossible Request
Your VP asks: "Make the API respond in under 10ms for all requests, globally."

Your system: US-East database, users worldwide. Physics says the speed of light from US-East to Singapore is ~120ms round-trip.

1. Explain why this is physically impossible with a centralized database.
2. Design an architecture that gets as close as possible to the requirement.
3. What consistency tradeoffs are you making?
4. What data can be served from the edge vs. what must hit the origin?

---

### E3. Security Audit
You inherit a backend codebase. You have 1 week to do a security audit.

1. What do you check first? (Priority order)
2. What automated tools do you run?
3. What manual checks are most important?
4. How do you prioritize fixing the issues you find?
5. What quick wins can you implement in week 1?

---

## Answer Key Hints

These are not full answers, but hints to check your thinking:

- **A1:** Think about the leftmost prefix rule. Equality columns before range columns. The ORDER BY can use the index if it matches.
- **A3:** PostgreSQL re-evaluates the WHERE clause of the UPDATE after the conflicting transaction commits. The final balance depends on whether the UPDATE uses the old snapshot or the new committed value.
- **B2:** Think about encoding bypasses (Unicode, URL encoding), numeric context (no quotes needed), second-order injection, and encoding-specific attacks (GBK multibyte).
- **C2:** Don't jump to sharding. Read replicas + caching can handle most read-heavy workloads. Calculate: 100K req/s × 80% reads ÷ 3ms = need ~240 concurrent read capacity.
- **D1:** The correlated subqueries execute once per row in the outer query. With 1M active users, that's 2M subquery executions.
- **E1:** In PostgreSQL, changing VARCHAR to TEXT is a metadata-only change (no table rewrite). But INTEGER to BIGINT requires rewriting every row.
