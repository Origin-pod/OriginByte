# Chapter 2 — Full Topic Mind Map

## Complete Knowledge Hierarchy

This mind map represents the entire domain of backend engineering, database optimization, and security. Each node is a topic you should understand deeply. Leaf nodes represent specific techniques or concepts.

```
Backend Engineering
│
├── 1. DATABASE
│   │
│   ├── 1.1 Fundamentals
│   │   ├── ACID Properties
│   │   │   ├── Atomicity — all or nothing transactions
│   │   │   ├── Consistency — data always valid per constraints
│   │   │   ├── Isolation — concurrent txns don't interfere
│   │   │   └── Durability — committed data survives crashes
│   │   ├── CAP Theorem
│   │   │   ├── Consistency — every read gets most recent write
│   │   │   ├── Availability — every request gets a response
│   │   │   └── Partition Tolerance — system works despite network splits
│   │   ├── Data Models
│   │   │   ├── Relational (PostgreSQL, MySQL)
│   │   │   ├── Document (MongoDB, CouchDB)
│   │   │   ├── Key-Value (Redis, DynamoDB)
│   │   │   ├── Column-Family (Cassandra, HBase)
│   │   │   ├── Graph (Neo4j, ArangoDB)
│   │   │   └── Time-Series (TimescaleDB, InfluxDB)
│   │   └── Storage Engines
│   │       ├── B-tree based (InnoDB, PostgreSQL default)
│   │       ├── LSM-tree based (RocksDB, LevelDB, Cassandra)
│   │       └── In-memory (Redis, VoltDB)
│   │
│   ├── 1.2 Indexing
│   │   ├── B-tree Index
│   │   │   ├── How it works (balanced tree, O(log n) lookup)
│   │   │   ├── Range queries (efficient for >, <, BETWEEN)
│   │   │   ├── Write amplification (tree rebalancing on insert)
│   │   │   └── Page splits and fragmentation
│   │   ├── Hash Index
│   │   │   ├── O(1) exact lookups
│   │   │   ├── Cannot do range queries
│   │   │   └── Hash collisions and resizing
│   │   ├── Composite Index
│   │   │   ├── Column ordering matters (leftmost prefix rule)
│   │   │   ├── Covering indexes (include all SELECT columns)
│   │   │   └── Index skip scan (when leading column is skipped)
│   │   ├── GiST / GIN Indexes (PostgreSQL)
│   │   │   ├── Full-text search
│   │   │   ├── JSONB indexing
│   │   │   ├── Geometric/spatial data
│   │   │   └── Array containment
│   │   ├── Partial Index
│   │   │   ├── Index only rows matching a WHERE clause
│   │   │   └── Smaller index, faster updates
│   │   ├── Expression Index
│   │   │   ├── Index on function result: CREATE INDEX ON t (lower(email))
│   │   │   └── Useful for case-insensitive lookups
│   │   └── Index Anti-patterns
│   │       ├── Over-indexing (slows writes, wastes storage)
│   │       ├── Unused indexes (monitor with pg_stat_user_indexes)
│   │       ├── Wrong column order in composite indexes
│   │       └── Indexing low-cardinality columns (boolean, status)
│   │
│   ├── 1.3 Query Optimization
│   │   ├── Query Planner / Optimizer
│   │   │   ├── Cost-based optimization
│   │   │   ├── Statistics (pg_stats, ANALYZE command)
│   │   │   ├── Cardinality estimation
│   │   │   └── Plan caching and prepared statements
│   │   ├── EXPLAIN / EXPLAIN ANALYZE
│   │   │   ├── Reading execution plans
│   │   │   ├── Seq Scan vs Index Scan vs Bitmap Scan
│   │   │   ├── Nested Loop vs Hash Join vs Merge Join
│   │   │   └── Actual vs Estimated rows (cardinality errors)
│   │   ├── Join Optimization
│   │   │   ├── Join order selection
│   │   │   ├── Hash join (build hash table from smaller set)
│   │   │   ├── Merge join (both inputs sorted)
│   │   │   ├── Nested loop (small outer, indexed inner)
│   │   │   └── Join pruning and pushdown
│   │   ├── N+1 Problem
│   │   │   ├── ORM lazy loading causing 1 + N queries
│   │   │   ├── Solutions: eager loading, batch loading, dataloader
│   │   │   └── Detection: query logging, APM tools
│   │   ├── Subquery Optimization
│   │   │   ├── Correlated vs non-correlated subqueries
│   │   │   ├── EXISTS vs IN performance
│   │   │   ├── Subquery materialization
│   │   │   └── Lateral joins (PostgreSQL)
│   │   └── Common Anti-patterns
│   │       ├── SELECT * (fetches unnecessary columns)
│   │       ├── OR conditions preventing index use
│   │       ├── Functions on indexed columns in WHERE
│   │       ├── Implicit type casting
│   │       └── OFFSET for pagination (use keyset pagination)
│   │
│   ├── 1.4 Transactions & Concurrency
│   │   ├── Isolation Levels
│   │   │   ├── Read Uncommitted (dirty reads possible)
│   │   │   ├── Read Committed (default in PostgreSQL)
│   │   │   ├── Repeatable Read (snapshot isolation)
│   │   │   └── Serializable (full isolation, slowest)
│   │   ├── Locking
│   │   │   ├── Row-level locks (SELECT ... FOR UPDATE)
│   │   │   ├── Table-level locks (DDL operations)
│   │   │   ├── Advisory locks (application-controlled)
│   │   │   ├── Optimistic locking (version column)
│   │   │   └── Pessimistic locking (explicit lock acquisition)
│   │   ├── MVCC (Multi-Version Concurrency Control)
│   │   │   ├── How PostgreSQL implements MVCC (xmin/xmax)
│   │   │   ├── How MySQL/InnoDB implements MVCC (undo log)
│   │   │   ├── Snapshot isolation mechanics
│   │   │   └── Vacuum/purge of old versions
│   │   ├── Deadlocks
│   │   │   ├── Detection (wait-for graphs)
│   │   │   ├── Prevention (lock ordering)
│   │   │   ├── Resolution (transaction rollback)
│   │   │   └── Monitoring and alerting
│   │   └── Write Skew & Phantoms
│   │       ├── Write skew anomaly explained
│   │       ├── Phantom reads
│   │       └── Serializable isolation as solution
│   │
│   ├── 1.5 Replication
│   │   ├── Single-Leader (Master-Slave)
│   │   │   ├── Synchronous vs asynchronous replication
│   │   │   ├── Replication lag
│   │   │   ├── Read-after-write consistency
│   │   │   └── Failover (automatic vs manual)
│   │   ├── Multi-Leader
│   │   │   ├── Conflict resolution strategies
│   │   │   ├── Last-write-wins
│   │   │   ├── Custom merge functions
│   │   │   └── Use cases (multi-datacenter)
│   │   ├── Leaderless (Dynamo-style)
│   │   │   ├── Quorum reads/writes (W + R > N)
│   │   │   ├── Sloppy quorums and hinted handoff
│   │   │   ├── Anti-entropy (Merkle trees)
│   │   │   └── Conflict resolution (vector clocks, CRDTs)
│   │   └── Change Data Capture (CDC)
│   │       ├── WAL-based CDC (Debezium)
│   │       ├── Event sourcing integration
│   │       └── Real-time data pipelines
│   │
│   ├── 1.6 Sharding & Partitioning
│   │   ├── Horizontal Partitioning (Sharding)
│   │   │   ├── Hash-based sharding
│   │   │   ├── Range-based sharding
│   │   │   ├── Directory-based sharding
│   │   │   └── Consistent hashing
│   │   ├── Vertical Partitioning
│   │   │   ├── Splitting columns across tables
│   │   │   └── Splitting hot/cold data
│   │   ├── PostgreSQL Native Partitioning
│   │   │   ├── Range partitioning
│   │   │   ├── List partitioning
│   │   │   ├── Hash partitioning
│   │   │   └── Partition pruning
│   │   └── Sharding Challenges
│   │       ├── Cross-shard queries
│   │       ├── Distributed transactions
│   │       ├── Rebalancing shards
│   │       └── Hot partitions
│   │
│   ├── 1.7 Caching
│   │   ├── Cache Strategies
│   │   │   ├── Cache-aside (lazy loading)
│   │   │   ├── Write-through
│   │   │   ├── Write-behind (write-back)
│   │   │   ├── Read-through
│   │   │   └── Refresh-ahead
│   │   ├── Cache Technologies
│   │   │   ├── Redis (data structures, persistence, clustering)
│   │   │   ├── Memcached (simple key-value, multi-threaded)
│   │   │   ├── Application-level cache (in-process)
│   │   │   └── CDN cache (edge caching)
│   │   ├── Cache Invalidation
│   │   │   ├── TTL-based expiration
│   │   │   ├── Event-driven invalidation
│   │   │   ├── Version-based invalidation
│   │   │   └── Tag-based invalidation
│   │   └── Cache Problems
│   │       ├── Cache stampede (thundering herd)
│   │       ├── Cache penetration (non-existent keys)
│   │       ├── Cache avalanche (mass expiration)
│   │       ├── Stale data
│   │       └── Memory pressure and eviction policies (LRU, LFU)
│   │
│   ├── 1.8 Connection Pooling
│   │   ├── Why pooling matters (TCP + TLS + auth overhead)
│   │   ├── Pool sizing (connections = (cores * 2) + spindles)
│   │   ├── PgBouncer (external pooler for PostgreSQL)
│   │   ├── Application-level pools (HikariCP, SQLAlchemy pool)
│   │   ├── Connection lifecycle management
│   │   └── Pool exhaustion and timeouts
│   │
│   └── 1.9 Monitoring & Observability
│       ├── Key Metrics
│       │   ├── Query latency (p50, p95, p99)
│       │   ├── Queries per second
│       │   ├── Connection count and pool utilization
│       │   ├── Cache hit ratio
│       │   ├── Replication lag
│       │   ├── Lock waits
│       │   ├── Table/index bloat
│       │   └── Disk I/O and buffer cache hit ratio
│       ├── Tools
│       │   ├── pg_stat_statements (PostgreSQL)
│       │   ├── SHOW PROCESSLIST (MySQL)
│       │   ├── slow query log
│       │   ├── APM (Datadog, New Relic)
│       │   └── pganalyze, Percona Monitoring
│       └── Alerting
│           ├── Latency thresholds
│           ├── Error rate spikes
│           ├── Connection pool exhaustion
│           └── Replication lag exceeding SLA
│
├── 2. BACKEND SECURITY
│   │
│   ├── 2.1 Authentication
│   │   ├── Password-based
│   │   │   ├── Hashing (bcrypt, scrypt, Argon2id)
│   │   │   ├── Salting (unique per user)
│   │   │   ├── Pepper (application-level secret)
│   │   │   └── Password policies (length > complexity)
│   │   ├── Token-based (JWT)
│   │   │   ├── Header.Payload.Signature structure
│   │   │   ├── Symmetric (HS256) vs Asymmetric (RS256, ES256)
│   │   │   ├── Access token + Refresh token pattern
│   │   │   ├── Token storage (httpOnly cookie vs localStorage)
│   │   │   └── Token revocation strategies
│   │   ├── Session-based
│   │   │   ├── Server-side session storage
│   │   │   ├── Session ID in cookie
│   │   │   ├── Session fixation prevention
│   │   │   └── Session hijacking prevention
│   │   ├── OAuth 2.0 / OIDC
│   │   │   ├── Authorization Code flow (web apps)
│   │   │   ├── PKCE (mobile/SPA)
│   │   │   ├── Client Credentials (service-to-service)
│   │   │   └── Token exchange
│   │   └── Multi-Factor Authentication
│   │       ├── TOTP (Google Authenticator)
│   │       ├── WebAuthn / FIDO2 (hardware keys)
│   │       ├── SMS (insecure, SIM swapping risk)
│   │       └── Recovery codes
│   │
│   ├── 2.2 Authorization
│   │   ├── RBAC (Role-Based Access Control)
│   │   ├── ABAC (Attribute-Based Access Control)
│   │   ├── ReBAC (Relationship-Based Access Control)
│   │   ├── Policy engines (OPA, Cedar, Casbin)
│   │   ├── Row-level security (PostgreSQL RLS)
│   │   └── Principle of least privilege
│   │
│   ├── 2.3 API Security
│   │   ├── Rate limiting (token bucket, sliding window, leaky bucket)
│   │   ├── Input validation (whitelist > blacklist)
│   │   ├── Output encoding
│   │   ├── Content-Type validation
│   │   ├── Request size limits
│   │   ├── API versioning security
│   │   ├── GraphQL-specific: depth limiting, query cost analysis
│   │   └── gRPC security: TLS, auth interceptors
│   │
│   ├── 2.4 Common Vulnerabilities (OWASP Top 10)
│   │   ├── A01 Broken Access Control
│   │   ├── A02 Cryptographic Failures
│   │   ├── A03 Injection (SQL, NoSQL, OS, LDAP)
│   │   ├── A04 Insecure Design
│   │   ├── A05 Security Misconfiguration
│   │   ├── A06 Vulnerable Components
│   │   ├── A07 Auth Failures
│   │   ├── A08 Data Integrity Failures
│   │   ├── A09 Logging & Monitoring Failures
│   │   └── A10 SSRF
│   │
│   ├── 2.5 Encryption
│   │   ├── At rest (AES-256, disk encryption, column-level)
│   │   ├── In transit (TLS 1.3, mTLS)
│   │   ├── Key management (KMS, Vault, envelope encryption)
│   │   └── Hashing vs encryption vs encoding
│   │
│   └── 2.6 Security Headers & Hardening
│       ├── CORS (Cross-Origin Resource Sharing)
│       ├── CSP (Content Security Policy)
│       ├── HSTS (HTTP Strict Transport Security)
│       ├── X-Content-Type-Options
│       ├── X-Frame-Options
│       └── Referrer-Policy
│
├── 3. PERFORMANCE
│   │
│   ├── 3.1 Profiling & Measurement
│   │   ├── APM tools (Datadog, New Relic, Jaeger)
│   │   ├── Distributed tracing
│   │   ├── Flame graphs
│   │   ├── Load testing (k6, Locust, Gatling)
│   │   └── Benchmarking methodology
│   │
│   ├── 3.2 Optimization Techniques
│   │   ├── Connection pooling
│   │   ├── Batch processing
│   │   ├── Async processing (message queues)
│   │   ├── Lazy vs eager loading
│   │   ├── Pagination (keyset > offset)
│   │   ├── Compression (gzip, brotli)
│   │   └── HTTP/2 and HTTP/3
│   │
│   └── 3.3 Scalability Patterns
│       ├── Horizontal vs vertical scaling
│       ├── Stateless services
│       ├── Event-driven architecture
│       ├── CQRS (Command Query Responsibility Segregation)
│       ├── Bulkhead pattern
│       ├── Circuit breaker pattern
│       └── Backpressure
│
├── 4. SYSTEM DESIGN
│   │
│   ├── 4.1 Architectural Patterns
│   │   ├── Monolith
│   │   ├── Microservices
│   │   ├── Service mesh
│   │   ├── Event sourcing
│   │   ├── Serverless
│   │   └── Hexagonal architecture
│   │
│   ├── 4.2 Communication Patterns
│   │   ├── REST
│   │   ├── GraphQL
│   │   ├── gRPC
│   │   ├── WebSockets
│   │   ├── Message queues (Kafka, RabbitMQ, SQS)
│   │   └── Pub/Sub
│   │
│   ├── 4.3 Data Consistency
│   │   ├── Strong consistency
│   │   ├── Eventual consistency
│   │   ├── Saga pattern
│   │   ├── Two-phase commit
│   │   ├── Outbox pattern
│   │   └── Idempotency
│   │
│   └── 4.4 Reliability
│       ├── Redundancy
│       ├── Failover
│       ├── Health checks
│       ├── Graceful degradation
│       ├── Chaos engineering
│       └── Disaster recovery (RPO, RTO)
│
└── 5. OPERATIONS & OBSERVABILITY
    │
    ├── 5.1 Logging
    │   ├── Structured logging (JSON)
    │   ├── Log levels and when to use them
    │   ├── Correlation IDs
    │   └── Log aggregation (ELK, Loki)
    │
    ├── 5.2 Metrics
    │   ├── RED method (Rate, Errors, Duration)
    │   ├── USE method (Utilization, Saturation, Errors)
    │   ├── SLIs, SLOs, SLAs
    │   └── Prometheus, Grafana
    │
    ├── 5.3 Tracing
    │   ├── Distributed tracing
    │   ├── OpenTelemetry
    │   ├── Trace propagation
    │   └── Sampling strategies
    │
    └── 5.4 Incident Response
        ├── Runbooks
        ├── On-call rotations
        ├── Post-mortems (blameless)
        └── War rooms
```

---

## Visual Mind Map (Simplified)

```
                              ┌──────────────────┐
                              │    BACKEND        │
                              │   ENGINEERING     │
                              └────────┬─────────┘
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            │                          │                          │
    ┌───────▼───────┐         ┌───────▼───────┐         ┌───────▼───────┐
    │   DATABASE    │         │   SECURITY    │         │  PERFORMANCE  │
    └───────┬───────┘         └───────┬───────┘         └───────┬───────┘
            │                         │                         │
     ┌──────┼──────┐          ┌──────┼──────┐          ┌──────┼──────┐
     │      │      │          │      │      │          │      │      │
  Index  Query  Scale      Auth   API   Vuln       Cache  Pool  Async
     │      │      │          │      │      │          │      │      │
  B-tree  Plan  Shard      JWT  Rate   SQLi       Redis  Size  Queue
  Hash   Join  Replic     OAuth Limit  XSS        TTL   PgB   Kafka
  GIN    N+1   Part       MFA  CORS   CSRF       LRU         Event
```

---

## Topic Dependency Graph

Understanding which topics depend on which others helps you plan your learning path:

```
    Fundamentals (ACID, CAP, SQL)
           │
           ├──▶ Indexing ──▶ Query Optimization ──▶ Execution Plans
           │
           ├──▶ Transactions ──▶ Isolation Levels ──▶ MVCC ──▶ Deadlocks
           │
           ├──▶ Replication ──▶ Consistency Models ──▶ Sharding
           │
           ├──▶ Connection Pooling ──▶ Performance Tuning
           │
           └──▶ Caching ──▶ Cache Invalidation ──▶ Distributed Caching

    Auth Fundamentals (Hashing, Sessions)
           │
           ├──▶ JWT ──▶ OAuth 2.0 ──▶ OIDC
           │
           ├──▶ RBAC ──▶ ABAC ──▶ Policy Engines
           │
           └──▶ API Security ──▶ Rate Limiting ──▶ DDoS Protection

    Networking (TCP, HTTP, TLS)
           │
           ├──▶ CORS ──▶ CSRF ──▶ Security Headers
           │
           ├──▶ Load Balancing ──▶ Service Mesh
           │
           └──▶ gRPC / WebSocket ──▶ Real-time Systems
```

Each arrow means "you should understand the source before the target." Use this to plan your study order.
