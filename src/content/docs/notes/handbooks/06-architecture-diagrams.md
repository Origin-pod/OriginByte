---
title: "Chapter 6 — Architecture Diagrams"
---

# Chapter 6 — Architecture Diagrams

## 6.1 Backend + Database Architecture (Standard)

```
                                    ┌─────────────────────────────────────────────┐
                                    │              INTERNET                        │
                                    └─────────────────────┬───────────────────────┘
                                                          │
                                                          ▼
                                    ┌─────────────────────────────────────────────┐
                                    │              CDN (CloudFront / Cloudflare)   │
                                    │  - Static assets (JS, CSS, images)          │
                                    │  - DDoS protection                          │
                                    │  - TLS termination                          │
                                    │  - Edge caching                             │
                                    └─────────────────────┬───────────────────────┘
                                                          │
                                                          ▼
                                    ┌─────────────────────────────────────────────┐
                                    │           LOAD BALANCER (ALB / Nginx)        │
                                    │  - Health checks                            │
                                    │  - SSL termination (if not at CDN)          │
                                    │  - Request routing                          │
                                    │  - Connection draining                      │
                                    └──────┬──────────────┬───────────────────────┘
                                           │              │
                              ┌────────────▼──┐    ┌──────▼────────────┐
                              │  App Server 1 │    │  App Server 2     │
                              │  ┌──────────┐ │    │  ┌──────────┐    │
                              │  │Middleware │ │    │  │Middleware │    │
                              │  ├──────────┤ │    │  ├──────────┤    │
                              │  │ Business │ │    │  │ Business │    │
                              │  │ Logic    │ │    │  │ Logic    │    │
                              │  ├──────────┤ │    │  ├──────────┤    │
                              │  │ ORM/DAL  │ │    │  │ ORM/DAL  │    │
                              │  └──────────┘ │    │  └──────────┘    │
                              └───────┬───────┘    └──────┬───────────┘
                                      │                   │
                                      └────────┬──────────┘
                                               │
                              ┌────────────────┼────────────────┐
                              │                │                │
                      ┌───────▼──────┐  ┌──────▼───────┐  ┌────▼──────────┐
                      │  PostgreSQL  │  │    Redis     │  │  Message Queue│
                      │  (Primary)   │  │  (Cache +    │  │  (RabbitMQ/   │
                      │              │  │   Sessions)  │  │   Kafka)      │
                      │  ┌────────┐  │  └──────────────┘  └───────┬───────┘
                      │  │ Data   │  │                            │
                      │  │ WAL    │  │                    ┌───────▼───────┐
                      │  │ Indexes│  │                    │  Worker       │
                      │  └────────┘  │                    │  Processes    │
                      └──────┬───────┘                    │  (Background  │
                             │                            │   jobs)       │
                    ┌────────┼────────┐                   └───────────────┘
                    │                 │
            ┌───────▼──────┐  ┌──────▼───────┐
            │  Read Replica│  │  Read Replica │
            │  1           │  │  2            │
            └──────────────┘  └──────────────┘
```

---

## 6.2 Scalable Backend Architecture (Production-Grade)

```
    ┌─────────────────────────────────────────────────────────────────────────────┐
    │                              EDGE LAYER                                     │
    │                                                                             │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                      │
    │  │ CDN PoP      │  │ CDN PoP      │  │ CDN PoP      │                      │
    │  │ (US-East)    │  │ (EU-West)    │  │ (AP-South)   │                      │
    │  │ - WAF rules  │  │ - WAF rules  │  │ - WAF rules  │                      │
    │  │ - Rate limit │  │ - Rate limit │  │ - Rate limit │                      │
    │  │ - DDoS mitg. │  │ - DDoS mitg. │  │ - DDoS mitg. │                      │
    │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                      │
    └─────────┼─────────────────┼─────────────────┼──────────────────────────────┘
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
    ┌───────────────────────────▼─────────────────────────────────────────────────┐
    │                          API GATEWAY LAYER                                  │
    │                                                                             │
    │  ┌─────────────────────────────────────────┐                                │
    │  │  API Gateway (Kong / AWS API Gateway)    │                                │
    │  │  - Authentication (JWT verification)     │                                │
    │  │  - Rate limiting (per user/API key)      │                                │
    │  │  - Request transformation                │                                │
    │  │  - API versioning                        │                                │
    │  │  - Request/response logging              │                                │
    │  │  - Circuit breaker                       │                                │
    │  └──────────┬──────────────┬───────────────┘                                │
    └─────────────┼──────────────┼────────────────────────────────────────────────┘
                  │              │
    ┌─────────────▼──────────────▼────────────────────────────────────────────────┐
    │                       SERVICE LAYER                                          │
    │                                                                             │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
    │  │ User Service │  │ Order Service│  │ Payment Svc  │  │ Notification │    │
    │  │ (3 replicas) │  │ (5 replicas) │  │ (3 replicas) │  │ Service      │    │
    │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │
    │         │                 │                 │                 │             │
    │  ┌──────┴─────────────────┴─────────────────┴─────────────────┴──────┐      │
    │  │              SERVICE MESH (Istio / Linkerd)                        │      │
    │  │  - mTLS between services                                          │      │
    │  │  - Traffic management (canary, blue-green)                        │      │
    │  │  - Observability (distributed tracing)                            │      │
    │  │  - Retry/timeout policies                                         │      │
    │  └───────────────────────────────────────────────────────────────────┘      │
    └─────────────────────────────────────────────────────────────────────────────┘
                  │              │              │              │
    ┌─────────────▼──────────────▼──────────────▼──────────────▼──────────────────┐
    │                        DATA LAYER                                           │
    │                                                                             │
    │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
    │  │ PostgreSQL       │  │ Redis Cluster   │  │ Elasticsearch   │             │
    │  │ Cluster          │  │                 │  │ (Search)        │             │
    │  │ ┌─────┐ ┌─────┐ │  │ ┌────┐ ┌────┐  │  └─────────────────┘             │
    │  │ │Prim.│→│Repl.│ │  │ │Mstr│→│Repl│  │                                  │
    │  │ └─────┘ └─────┘ │  │ └────┘ └────┘  │  ┌─────────────────┐             │
    │  │          ┌─────┐ │  │ ┌────┐ ┌────┐  │  │ Kafka           │             │
    │  │          │Repl.│ │  │ │Mstr│→│Repl│  │  │ (Event Stream)  │             │
    │  │          └─────┘ │  │ └────┘ └────┘  │  │ 3 brokers       │             │
    │  └─────────────────┘  └─────────────────┘  └─────────────────┘             │
    │                                                                             │
    │  ┌─────────────────┐  ┌─────────────────┐                                  │
    │  │ S3              │  │ Secrets Manager  │                                  │
    │  │ (Object Store)  │  │ (Vault/AWS SM)   │                                  │
    │  └─────────────────┘  └─────────────────┘                                  │
    └─────────────────────────────────────────────────────────────────────────────┘
                                      │
    ┌─────────────────────────────────▼───────────────────────────────────────────┐
    │                      OBSERVABILITY LAYER                                    │
    │                                                                             │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
    │  │ Prometheus   │  │ Grafana      │  │ Jaeger       │  │ ELK Stack    │    │
    │  │ (Metrics)    │  │ (Dashboards) │  │ (Tracing)    │  │ (Logs)       │    │
    │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
    │                                                                             │
    │  ┌──────────────┐                                                          │
    │  │ PagerDuty    │                                                          │
    │  │ (Alerting)   │                                                          │
    │  └──────────────┘                                                          │
    └─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6.3 Secure API Request Flow

```
    ┌──────────┐
    │  Client  │
    └────┬─────┘
         │
         │ HTTPS (TLS 1.3)
         │ POST /api/v1/orders
         │ Authorization: Bearer eyJ...
         │ Content-Type: application/json
         │ X-Request-ID: uuid
         │
         ▼
    ┌─────────────────────────────────────────────────────────────┐
    │                     WAF / CDN                                │
    │  □ Check IP against blocklist                                │
    │  □ Check request against OWASP rules (SQLi, XSS patterns)  │
    │  □ Check request size < limit                                │
    │  □ DDoS detection (request rate anomaly)                    │
    └─────────────────────┬───────────────────────────────────────┘
                          │ PASS
                          ▼
    ┌─────────────────────────────────────────────────────────────┐
    │                   API GATEWAY                                │
    │  □ Rate limit check (user: 100/min, IP: 1000/min)           │
    │  □ JWT signature verification                                │
    │  □ JWT expiry check (exp > now)                              │
    │  □ JWT audience check (aud = "api.example.com")              │
    │  □ JWT issuer check (iss = "auth.example.com")               │
    │  □ Extract user identity from JWT claims                     │
    │  □ Log: request_id, user_id, endpoint, timestamp             │
    └─────────────────────┬───────────────────────────────────────┘
                          │ AUTHENTICATED
                          ▼
    ┌─────────────────────────────────────────────────────────────┐
    │                 APPLICATION SERVER                            │
    │                                                              │
    │  Middleware Pipeline:                                         │
    │  ┌────────────────────────────────────────────────────────┐  │
    │  │ 1. Request parsing                                     │  │
    │  │    □ Parse JSON body                                   │  │
    │  │    □ Validate Content-Type header                      │  │
    │  ├────────────────────────────────────────────────────────┤  │
    │  │ 2. Input validation                                    │  │
    │  │    □ Schema validation (Zod/Pydantic)                  │  │
    │  │    □ Type checking                                     │  │
    │  │    □ Length limits                                      │  │
    │  │    □ Business rule validation                          │  │
    │  ├────────────────────────────────────────────────────────┤  │
    │  │ 3. Authorization                                       │  │
    │  │    □ Check user role/permissions                       │  │
    │  │    □ Check resource ownership                          │  │
    │  │    □ Policy engine evaluation (OPA/Cedar)              │  │
    │  ├────────────────────────────────────────────────────────┤  │
    │  │ 4. Business logic                                      │  │
    │  │    □ Process the request                               │  │
    │  │    □ Database operations (parameterized queries)       │  │
    │  │    □ External service calls                            │  │
    │  ├────────────────────────────────────────────────────────┤  │
    │  │ 5. Response                                            │  │
    │  │    □ Sanitize output (no internal details in errors)   │  │
    │  │    □ Set security headers                              │  │
    │  │    □ Log response status                               │  │
    │  └────────────────────────────────────────────────────────┘  │
    └─────────────────────────────────────────────────────────────┘
```

---

## 6.4 Database Read Replica Setup

```
    WRITE PATH
    ═══════════════════════════════════════════════════════

    App Server ──(INSERT/UPDATE/DELETE)──▶ ┌──────────────┐
                                           │   PRIMARY    │
                                           │  (Leader)    │
                                           │              │
                                           │  Processes   │
                                           │  write,      │
                                           │  writes WAL  │
                                           └──────┬───────┘
                                                  │
                                           WAL Stream
                                     (Write-Ahead Log)
                                                  │
                              ┌────────────────────┼────────────────────┐
                              │                    │                    │
                       ┌──────▼──────┐     ┌──────▼──────┐     ┌──────▼──────┐
                       │  REPLICA 1  │     │  REPLICA 2  │     │  REPLICA 3  │
                       │  (Sync)     │     │  (Async)    │     │  (Async)    │
                       │             │     │             │     │  Cross-     │
                       │  0ms lag    │     │  <100ms lag │     │  region     │
                       │  Confirms   │     │  Eventually │     │  ~50ms lag  │
                       │  to primary │     │  consistent │     │  DR/Backup  │
                       └─────────────┘     └─────────────┘     └─────────────┘

    READ PATH
    ═══════════════════════════════════════════════════════

                              ┌─────────────────┐
                              │  App Server     │
                              │                 │
                              │  Read router:   │
                              │  ┌───────────┐  │
                              │  │ Recent    │  │
                              │  │ write by  │──┼──▶ PRIMARY (read-your-writes)
                              │  │ this user?│  │
                              │  └─────┬─────┘  │
                              │        │ No     │
                              │  ┌─────▼─────┐  │
                              │  │ Round-    │  │
                              │  │ robin to  │──┼──▶ REPLICA 1 or REPLICA 2
                              │  │ replicas  │  │
                              │  └───────────┘  │
                              └─────────────────┘


    FAILOVER SCENARIO
    ═══════════════════════════════════════════════════════

    Normal:  PRIMARY ──▶ REPLICA 1 (sync) ──▶ REPLICA 2 (async)

    Primary fails:
    1. Health check detects primary is down
    2. REPLICA 1 (sync, most up-to-date) promoted to PRIMARY
    3. REPLICA 2 now replicates from new PRIMARY
    4. Old primary, when recovered, joins as replica

    ┌──────────┐              ┌──────────────┐
    │ PRIMARY  │──── X ────   │ REPLICA 1    │ ← promoted to PRIMARY
    │ (down)   │              │ (now PRIMARY)│
    └──────────┘              └──────┬───────┘
                                     │
                              ┌──────▼───────┐
                              │ REPLICA 2    │
                              │ (re-pointed) │
                              └──────────────┘
```

---

## 6.5 Caching Layer Architecture

```
    ┌──────────────────────────────────────────────────────────────────┐
    │                    MULTI-LAYER CACHE                             │
    │                                                                  │
    │  Layer 0: CLIENT CACHE                                           │
    │  ┌─────────────────────────────────────────────────────────────┐ │
    │  │  Browser cache (Cache-Control: max-age=3600)                │ │
    │  │  Service worker cache (offline-first)                       │ │
    │  │  Hit rate: ~30-50% for static assets                        │ │
    │  └─────────────────────────────────────────────────────────────┘ │
    │                            │ MISS                                │
    │                            ▼                                     │
    │  Layer 1: CDN / EDGE CACHE                                       │
    │  ┌─────────────────────────────────────────────────────────────┐ │
    │  │  CloudFront / Cloudflare (geographically distributed)       │ │
    │  │  Caches: static assets, API responses (Vary: Authorization) │ │
    │  │  Hit rate: ~60-80% for cacheable content                    │ │
    │  └─────────────────────────────────────────────────────────────┘ │
    │                            │ MISS                                │
    │                            ▼                                     │
    │  Layer 2: APPLICATION CACHE (in-process)                         │
    │  ┌─────────────────────────────────────────────────────────────┐ │
    │  │  In-memory cache per server instance                        │ │
    │  │  LRU cache, max 1000 entries, TTL 60s                       │ │
    │  │  e.g., Python lru_cache, Node.js node-cache, Go sync.Map   │ │
    │  │  Hit rate: ~20-40%                                          │ │
    │  │  Caveat: Not shared between instances (inconsistency risk)  │ │
    │  └─────────────────────────────────────────────────────────────┘ │
    │                            │ MISS                                │
    │                            ▼                                     │
    │  Layer 3: DISTRIBUTED CACHE (Redis / Memcached)                  │
    │  ┌─────────────────────────────────────────────────────────────┐ │
    │  │  Shared across all app server instances                     │ │
    │  │  Redis Cluster: 3 masters + 3 replicas                      │ │
    │  │  Stores: session data, query results, computed values       │ │
    │  │  Hit rate: ~80-95% for hot data                             │ │
    │  │  Latency: ~0.5ms (same VPC)                                 │ │
    │  └─────────────────────────────────────────────────────────────┘ │
    │                            │ MISS                                │
    │                            ▼                                     │
    │  Layer 4: DATABASE                                               │
    │  ┌─────────────────────────────────────────────────────────────┐ │
    │  │  PostgreSQL (buffer cache: shared_buffers = 25% of RAM)     │ │
    │  │  OS page cache (remaining RAM)                              │ │
    │  │  Disk I/O (SSD: ~16μs random read)                          │ │
    │  │  Latency: 1-50ms depending on query complexity              │ │
    │  └─────────────────────────────────────────────────────────────┘ │
    └──────────────────────────────────────────────────────────────────┘

    CACHE INVALIDATION FLOW
    ═══════════════════════════════════════════════════════

    ┌───────────┐  write   ┌───────────┐
    │ App Server│ ───────▶ │ Database  │
    └─────┬─────┘          └───────────┘
          │
          │  After successful write:
          │
          ├──▶ DELETE from Redis (cache-aside invalidation)
          │
          ├──▶ Publish event to Kafka: "user.42.updated"
          │
          └──▶ Other services consume event and invalidate
               their own caches
```

---

## 6.6 Authentication Flow (Complete)

```
    ┌──────────┐                ┌──────────────┐              ┌──────────────┐
    │  Client  │                │  Auth Server │              │  API Server  │
    │  (SPA)   │                │  (IdP)       │              │  (Resource)  │
    └────┬─────┘                └──────┬───────┘              └──────┬───────┘
         │                             │                             │
         │  ══════ REGISTRATION ══════ │                             │
         │                             │                             │
         │  POST /auth/register        │                             │
         │  {email, password}          │                             │
         │ ───────────────────────────▶│                             │
         │                             │  Validate input             │
         │                             │  Hash password (Argon2id)   │
         │                             │  Store user + hash          │
         │  201 Created                │                             │
         │ ◀───────────────────────────│                             │
         │                             │                             │
         │  ══════ LOGIN ═════════════ │                             │
         │                             │                             │
         │  POST /auth/login           │                             │
         │  {email, password}          │                             │
         │ ───────────────────────────▶│                             │
         │                             │  Fetch user by email        │
         │                             │  Verify password vs hash    │
         │                             │  Generate access token      │
         │                             │    (JWT, 15min expiry)      │
         │                             │  Generate refresh token     │
         │                             │    (opaque, 7d expiry,      │
         │                             │     stored in DB)           │
         │                             │                             │
         │  Set-Cookie: refresh=...    │                             │
         │  (httpOnly, Secure,         │                             │
         │   SameSite=Strict, Path=/   │                             │
         │   auth/refresh)             │                             │
         │  Body: {access_token: ...}  │                             │
         │ ◀───────────────────────────│                             │
         │                             │                             │
         │  ══════ API REQUEST ══════  │                             │
         │                             │                             │
         │  GET /api/orders            │                             │
         │  Authorization: Bearer eyJ..│                             │
         │ ────────────────────────────┼────────────────────────────▶│
         │                             │                             │
         │                             │    Verify JWT signature     │
         │                             │    Check exp > now          │
         │                             │    Extract user claims      │
         │                             │    Check authorization      │
         │                             │    Process request          │
         │                             │                             │
         │  200 OK {orders: [...]}     │                             │
         │ ◀───────────────────────────┼─────────────────────────────│
         │                             │                             │
         │  ══════ TOKEN REFRESH ════  │                             │
         │  (access token expired)     │                             │
         │                             │                             │
         │  POST /auth/refresh         │                             │
         │  Cookie: refresh=...        │                             │
         │ ───────────────────────────▶│                             │
         │                             │  Look up refresh token in DB│
         │                             │  Verify not expired/revoked │
         │                             │  Rotate: issue new refresh  │
         │                             │    token, invalidate old    │
         │                             │  Issue new access token     │
         │                             │                             │
         │  New tokens                 │                             │
         │ ◀───────────────────────────│                             │
         │                             │                             │
         │  ══════ LOGOUT ════════════ │                             │
         │                             │                             │
         │  POST /auth/logout          │                             │
         │ ───────────────────────────▶│                             │
         │                             │  Revoke refresh token in DB │
         │                             │  Add access token JTI to    │
         │                             │    blocklist (Redis, TTL=   │
         │                             │    remaining token lifetime)│
         │                             │  Clear refresh cookie       │
         │  204 No Content             │                             │
         │ ◀───────────────────────────│                             │
```

---

## 6.7 Attack Surface Diagram

```
    ┌─────────────────────────────────────────────────────────────────────────────┐
    │                        ATTACK SURFACE MAP                                   │
    │                                                                             │
    │  ┌─────────────────────────────────────────────────────────────────┐        │
    │  │                     EXTERNAL ATTACKS                            │        │
    │  │                                                                 │        │
    │  │  ┌──── DDoS ────┐  ┌── Bot Attacks ──┐  ┌── DNS Attack ──┐    │        │
    │  │  │ Volumetric   │  │ Credential      │  │ DNS poisoning  │    │        │
    │  │  │ Application  │  │ stuffing        │  │ DNS hijacking  │    │        │
    │  │  │ Protocol     │  │ Scraping        │  │ Subdomain      │    │        │
    │  │  └──────────────┘  └─────────────────┘  │ takeover       │    │        │
    │  │                                          └────────────────┘    │        │
    │  └─────────────────────────────┬───────────────────────────────────┘        │
    │                                │                                            │
    │                                ▼                                            │
    │  ┌─────────────────────────────────────────────────────────────────┐        │
    │  │                    NETWORK LAYER ATTACKS                        │        │
    │  │                                                                 │        │
    │  │  ● Man-in-the-middle (if TLS misconfigured)                    │        │
    │  │  ● TLS downgrade (POODLE, BEAST)                               │        │
    │  │  ● Certificate spoofing                                        │        │
    │  │  ● Port scanning / service enumeration                        │        │
    │  └─────────────────────────────┬───────────────────────────────────┘        │
    │                                │                                            │
    │                                ▼                                            │
    │  ┌─────────────────────────────────────────────────────────────────┐        │
    │  │                   APPLICATION LAYER ATTACKS                     │        │
    │  │                                                                 │        │
    │  │  Authentication:           │  Injection:                        │        │
    │  │  ● Brute force             │  ● SQL injection                  │        │
    │  │  ● Credential stuffing     │  ● NoSQL injection                │        │
    │  │  ● Session fixation        │  ● OS command injection           │        │
    │  │  ● JWT manipulation        │  ● LDAP injection                 │        │
    │  │  ● OAuth token theft       │  ● Template injection             │        │
    │  │                            │                                    │        │
    │  │  Client-side:              │  Server-side:                      │        │
    │  │  ● XSS (stored/reflected)  │  ● SSRF                          │        │
    │  │  ● CSRF                    │  ● File inclusion (LFI/RFI)       │        │
    │  │  ● Clickjacking            │  ● Insecure deserialization       │        │
    │  │  ● Open redirect           │  ● XML External Entity (XXE)     │        │
    │  │                            │  ● Path traversal                 │        │
    │  │  Business logic:           │                                    │        │
    │  │  ● IDOR (Insecure Direct   │  API-specific:                    │        │
    │  │    Object Reference)       │  ● Broken function-level auth    │        │
    │  │  ● Race conditions         │  ● Mass assignment                │        │
    │  │  ● Price manipulation      │  ● Excessive data exposure       │        │
    │  │  ● Privilege escalation    │  ● GraphQL batching attacks      │        │
    │  └─────────────────────────────┬───────────────────────────────────┘        │
    │                                │                                            │
    │                                ▼                                            │
    │  ┌─────────────────────────────────────────────────────────────────┐        │
    │  │                     DATA LAYER ATTACKS                          │        │
    │  │                                                                 │        │
    │  │  ● SQL injection (bypassed app layer)                          │        │
    │  │  ● Database credential theft                                   │        │
    │  │  ● Unencrypted backup theft                                    │        │
    │  │  ● Privilege escalation within DB                              │        │
    │  │  ● Data exfiltration via error messages                        │        │
    │  │  ● Side-channel attacks (timing-based data extraction)         │        │
    │  └─────────────────────────────┬───────────────────────────────────┘        │
    │                                │                                            │
    │                                ▼                                            │
    │  ┌─────────────────────────────────────────────────────────────────┐        │
    │  │                  INFRASTRUCTURE ATTACKS                          │        │
    │  │                                                                 │        │
    │  │  ● Container escape                                             │        │
    │  │  ● IAM misconfiguration (overly permissive roles)              │        │
    │  │  ● Exposed metadata endpoints                                  │        │
    │  │  ● Supply chain attacks (compromised dependencies)             │        │
    │  │  ● Secrets in source code / environment variables              │        │
    │  │  ● Unpatched vulnerabilities (CVEs)                            │        │
    │  └─────────────────────────────────────────────────────────────────┘        │
    └─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6.8 CQRS + Event Sourcing Architecture

```
    ┌──────────┐                                                    ┌──────────┐
    │  Client  │                                                    │  Client  │
    │ (Write)  │                                                    │ (Read)   │
    └────┬─────┘                                                    └────┬─────┘
         │                                                               │
         │ POST /orders                                   GET /orders    │
         │ (Command)                                      (Query)        │
         ▼                                                               ▼
    ┌──────────────┐                                         ┌──────────────┐
    │  Command     │                                         │  Query       │
    │  Service     │                                         │  Service     │
    │              │                                         │              │
    │  Validates   │                                         │  Simple      │
    │  business    │                                         │  reads from  │
    │  rules       │                                         │  read model  │
    └──────┬───────┘                                         └──────┬───────┘
           │                                                        │
           │ Append event                                           │ SELECT
           ▼                                                        ▼
    ┌──────────────┐    Event Stream    ┌──────────────┐    ┌──────────────┐
    │  Event Store │ ──────────────────▶│  Projector   │───▶│  Read DB     │
    │  (append-    │    (Kafka/         │  (Builds     │    │  (Denormal-  │
    │   only log)  │     EventStore)    │   read       │    │   ized,      │
    │              │                    │   models)    │    │   optimized  │
    │  OrderCreated│                    └──────────────┘    │   for reads) │
    │  OrderPaid   │                                        └──────────────┘
    │  OrderShipped│
    └──────────────┘

    Benefits:
    - Write model optimized for consistency (normalized)
    - Read model optimized for queries (denormalized)
    - Complete audit trail (event store)
    - Can rebuild read models from events
    - Write and read scale independently

    Tradeoff:
    - Eventual consistency between write and read
    - Complexity of event schema evolution
    - Debugging across events is harder
```
