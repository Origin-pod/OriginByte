---
format: md
---

# Backend Implementation Plan: DB Design Simulator

**Last Updated**: 2026-01-20
**Author**: Senior Backend Architect
**Status**: Design Phase

---

## Executive Summary

This document specifies the backend architecture for a **canvas-based DB Design Simulator** — a system that allows users to visually compose database architectures and run realistic simulations against them.

**Core Challenge**: Build a backend that powers an interactive canvas while faithfully simulating database internals (storage, indexing, query execution, transactions) in a way that is both educational and technically accurate.

**Key Decisions**:
1. **Hybrid Architecture**: Simulation engine runs client-side (WebAssembly/TypeScript), backend handles orchestration, persistence, and analytics
2. **Event Sourcing**: All simulation steps emit events for replay and debugging
3. **Stateless API**: Backend is stateless; simulation state lives in browser
4. **PostgreSQL**: Single database for all persistence (designs, runs, metrics)

---

## Part 1: High-Level Architecture

### System Boundaries

```
┌────────────────────────────────────────────────────────────────┐
│                         Frontend (Browser)                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
│  │   Canvas     │  │  Simulation  │  │   Visualization      │ │
│  │   Editor     │  │   Engine     │  │   (Metrics, Charts)  │ │
│  │  (React)     │  │  (WASM/TS)   │  │                      │ │
│  └──────┬───────┘  └──────┬───────┘  └──────────────────────┘ │
│         │                 │                                     │
└─────────┼─────────────────┼─────────────────────────────────────┘
          │                 │
          │ REST/GraphQL    │ Events (optional WebSocket)
          │                 │
┌─────────▼─────────────────▼─────────────────────────────────────┐
│                      Backend (Node.js/Go)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
│  │   Canvas     │  │  Simulation  │  │   Analytics          │ │
│  │   Service    │  │   Metadata   │  │   Service            │ │
│  │              │  │   Service    │  │                      │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────────────┘ │
│         │                 │                  │                  │
│  ┌──────▼─────────────────▼──────────────────▼───────────────┐ │
│  │              Persistence Layer (PostgreSQL)               │ │
│  │  - Canvas Designs   - Simulation Runs   - Metrics        │ │
│  └──────────────────────────────────────────────────────────-┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Why Hybrid Architecture?

**Simulation runs client-side**:
- ✅ Zero backend load (scales to millions of users)
- ✅ Instant feedback (`<100ms` latency)
- ✅ Works offline
- ❌ Limited by browser memory (~2GB practical limit)
- ❌ Can't run extremely large simulations (`>1M` records)

**Backend handles orchestration**:
- ✅ Persist designs (share, collaborate, version)
- ✅ Aggregate metrics across users (leaderboards, benchmarks)
- ✅ Heavy lifting (pre-compute large datasets, long-running simulations)
- ❌ Adds deployment complexity

**Decision**: Start with client-side simulation, add optional backend simulation in Phase 2.

---

## Part 2: Core Services

### Service 1: Canvas Service

**Responsibility**: Manage canvas designs (CRUD, versioning, collaboration)

**Capabilities**:
- Create/read/update/delete canvas designs
- Version control (save checkpoints, branch, merge)
- Share designs (public links, permissions)
- Templates (OLTP, OLAP, write-heavy presets)

**Data Owned**:
- `designs` table (canvas layouts, block placements, connections)
- `design_versions` table (git-like versioning)
- `templates` table (pre-built designs)

**APIs** (high-level):
```
POST   /api/designs                    # Create new design
GET    /api/designs/:id                # Fetch design
PUT    /api/designs/:id                # Update design
DELETE /api/designs/:id                # Delete design
GET    /api/designs/:id/versions       # List versions
POST   /api/designs/:id/fork           # Duplicate design
GET    /api/templates                  # List templates
```

**Key Design Decisions**:
- **Optimistic Locking**: Use ETags to prevent concurrent edit conflicts
- **Soft Deletes**: Never hard-delete (archive flag + deleted_at timestamp)
- **Immutable Versions**: Each save creates a new version (append-only)

---

### Service 2: Simulation Metadata Service

**Responsibility**: Store simulation definitions and configurations (NOT execution)

**Capabilities**:
- Save workload definitions (queries, operations, distributions)
- Store block configurations (parameters, connections)
- Persist "golden" datasets for benchmarking

**Data Owned**:
- `workloads` table (query patterns, distributions, concurrency)
- `block_configs` table (B-tree fanout, buffer sizes, etc.)
- `datasets` table (pre-generated data for consistent benchmarking)

**APIs**:
```
POST   /api/workloads                  # Create workload
GET    /api/workloads/:id              # Fetch workload
GET    /api/workloads/templates        # YCSB, TPC-C presets
POST   /api/datasets/generate          # Generate synthetic data
GET    /api/datasets/:id               # Fetch dataset
```

**Key Design Decisions**:
- **Workloads are reusable**: One workload can be run against multiple designs
- **Datasets are immutable**: Once generated, never modified (cache-friendly)
- **Parameterized workloads**: Support templates with variables (e.g., "read_percent: 70%")

---

### Service 3: Simulation Execution Service (Optional - Phase 2)

**Responsibility**: Run simulations backend-side (for large-scale or comparative experiments)

**Capabilities**:
- Execute workloads against designs (backend variant of client-side engine)
- Queue management (async jobs)
- Resource limits (timeout, memory caps)

**Data Owned**:
- `simulation_jobs` table (queued, running, completed)
- `job_logs` table (execution traces)

**APIs**:
```
POST   /api/simulations/execute        # Submit simulation job
GET    /api/simulations/:job_id        # Poll job status
GET    /api/simulations/:job_id/logs   # Stream logs (SSE)
DELETE /api/simulations/:job_id        # Cancel job
```

**Key Design Decisions**:
- **Async by default**: All simulations are jobs (no blocking requests)
- **Idempotent**: Retry-safe (use idempotency keys)
- **Timeouts**: Hard limit (60s for MVP, configurable later)
- **Priority queue**: Premium users get faster execution

**Implementation Note**: This is **Phase 2**. MVP runs simulations client-side only.

---

### Service 4: Results & Metrics Service

**Responsibility**: Persist and analyze simulation results

**Capabilities**:
- Store simulation results (metrics, events, traces)
- Compare runs (A/B testing designs)
- Aggregate analytics (which designs are fastest, most popular blocks)

**Data Owned**:
- `simulation_runs` table (metadata: design_id, workload_id, timestamp)
- `run_metrics` table (throughput, latency, IO counts)
- `run_events` table (event sourcing: every simulation step)

**APIs**:
```
POST   /api/runs                       # Save simulation result
GET    /api/runs/:id                   # Fetch result
GET    /api/runs/compare?ids=1,2,3     # Compare multiple runs
GET    /api/analytics/popular-blocks   # Which blocks are most used
GET    /api/analytics/benchmarks       # Leaderboard (fastest designs)
```

**Key Design Decisions**:
- **Event Sourcing**: Store all events (page reads, index lookups) for replay
- **Time-series data**: Use TimescaleDB extension for metrics (efficient queries)
- **Aggregations**: Pre-compute summaries (p50, p95, p99 latencies)

---

### Service 5: Analytics & Insights Service

**Responsibility**: Derive insights from simulation data

**Capabilities**:
- Detect bottlenecks (e.g., "Heap Storage is slowest")
- Suggest optimizations (e.g., "Add a buffer pool")
- Generate reports (PDF/Markdown summaries)

**Data Owned**:
- `insights` table (bottleneck detections, suggestions)
- `reports` table (generated PDFs, Markdown exports)

**APIs**:
```
POST   /api/insights/detect            # Analyze run, return suggestions
POST   /api/reports/generate           # Generate PDF/Markdown report
GET    /api/reports/:id                # Download report
```

**Key Design Decisions**:
- **Rule-based initially**: Hardcoded heuristics (if IO > 80%, suggest buffer)
- **ML later**: Train models on user data (which suggestions are followed)

---

## Part 3: Data Models

### Core Domain Model

```
User
 ├─ Designs (1:N)
 │   ├─ Blocks (nodes on canvas)
 │   ├─ Connections (edges)
 │   └─ Versions (snapshots)
 │
 ├─ Workloads (1:N)
 │   ├─ Operations (INSERT, SELECT, etc.)
 │   └─ Distributions (Zipfian, Uniform)
 │
 └─ Simulation Runs (1:N)
     ├─ Metrics (aggregate results)
     └─ Events (detailed trace)
```

### Entity Definitions

#### 1. Design

```sql
CREATE TABLE designs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id),
  name TEXT NOT NULL,
  description TEXT,
  canvas_state JSONB NOT NULL,  -- Blocks, connections, positions
  is_template BOOLEAN DEFAULT FALSE,
  is_public BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_designs_user ON designs(user_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_designs_templates ON designs(is_template) WHERE is_template = TRUE;
```

**`canvas_state` Schema**:
```typescript
interface CanvasState {
  blocks: Block[];
  connections: Connection[];
  viewport: { x: number; y: number; zoom: number };
}

interface Block {
  id: string;
  type: string;  // 'heap-storage', 'btree-index', etc.
  position: { x: number; y: number };
  parameters: Record<string, any>;  // Block-specific config
}

interface Connection {
  id: string;
  source: { blockId: string; port: string };
  target: { blockId: string; port: string };
}
```

**Why JSONB?**
- ✅ Flexible schema (blocks have different parameters)
- ✅ PostgreSQL JSONB is fast (indexed, queryable)
- ✅ Can query by block type: `WHERE canvas_state @> '{"blocks": [{"type": "lsm-tree"}]}'`

---

#### 2. Design Versions

```sql
CREATE TABLE design_versions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  design_id UUID NOT NULL REFERENCES designs(id),
  version_number INT NOT NULL,
  canvas_state JSONB NOT NULL,
  commit_message TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),

  UNIQUE(design_id, version_number)
);

CREATE INDEX idx_versions_design ON design_versions(design_id);
```

**Why separate table?**
- ✅ Designs table stays small (only latest version)
- ✅ Versions are append-only (never updated)
- ✅ Can diff versions (canvas_state JSON diff)

---

#### 3. Workload

```sql
CREATE TABLE workloads (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id),  -- NULL if system template
  name TEXT NOT NULL,
  description TEXT,
  operations JSONB NOT NULL,  -- Array of operations
  config JSONB NOT NULL,      -- Concurrency, distribution, etc.
  is_template BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**`operations` Schema**:
```typescript
interface Operation {
  type: 'INSERT' | 'SELECT' | 'UPDATE' | 'DELETE';
  template: string;  // SQL-like template or description
  weight: number;    // Percentage (0-100)
  parameters?: Record<string, any>;
}
```

**`config` Schema**:
```typescript
interface WorkloadConfig {
  total_operations: number;       // e.g., 10000
  concurrency: number;            // Parallel threads
  distribution: 'uniform' | 'zipfian';
  zipfian_theta?: number;         // 0.99 = extreme skew
  key_range: { min: number; max: number };
}
```

---

#### 4. Simulation Run

```sql
CREATE TABLE simulation_runs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  design_id UUID NOT NULL REFERENCES designs(id),
  workload_id UUID NOT NULL REFERENCES workloads(id),
  user_id UUID NOT NULL REFERENCES users(id),

  -- Metadata
  started_at TIMESTAMPTZ DEFAULT NOW(),
  completed_at TIMESTAMPTZ,
  status TEXT NOT NULL CHECK (status IN ('running', 'completed', 'failed')),

  -- Summary metrics (denormalized for fast queries)
  throughput_qps DECIMAL,
  latency_p50_ms DECIMAL,
  latency_p95_ms DECIMAL,
  latency_p99_ms DECIMAL,
  total_io_pages INT,
  cache_hit_ratio DECIMAL,

  -- Full results (detailed)
  metrics JSONB,  -- All metrics, not just summary

  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_runs_design ON simulation_runs(design_id);
CREATE INDEX idx_runs_user ON simulation_runs(user_id);
CREATE INDEX idx_runs_completed ON simulation_runs(completed_at) WHERE status = 'completed';
```

**Why denormalize summary metrics?**
- ✅ Fast queries for comparison ("show me runs faster than 1000 QPS")
- ✅ Avoid parsing JSONB for common queries
- ❌ Adds write overhead (acceptable: writes are rare)

---

#### 5. Run Events (Event Sourcing)

```sql
CREATE TABLE run_events (
  id BIGSERIAL PRIMARY KEY,
  run_id UUID NOT NULL REFERENCES simulation_runs(id),
  sequence_number INT NOT NULL,  -- Order within run
  timestamp TIMESTAMPTZ DEFAULT NOW(),

  event_type TEXT NOT NULL,  -- 'page_read', 'index_lookup', 'lock_acquired'
  block_id TEXT NOT NULL,    -- Which block emitted this
  payload JSONB NOT NULL,    -- Event-specific data

  UNIQUE(run_id, sequence_number)
);

CREATE INDEX idx_events_run ON run_events(run_id);
CREATE INDEX idx_events_type ON run_events(event_type);
```

**Example Events**:
```typescript
// Page read event
{
  event_type: 'page_read',
  block_id: 'heap-storage-1',
  payload: {
    page_id: 42,
    from_cache: false,
    latency_us: 1500  // Disk read
  }
}

// B-tree lookup event
{
  event_type: 'btree_lookup',
  block_id: 'btree-index-1',
  payload: {
    key: 12345,
    tree_height: 3,
    pages_traversed: 3,
    found: true
  }
}
```

**Why event sourcing?**
- ✅ Full audit trail (replay simulation step-by-step)
- ✅ Debugging ("why did this query take 10ms?")
- ✅ Generate visualizations (animate data flow)
- ❌ Storage cost (mitigate: compress old runs, archive to S3)

---

#### 6. Run Metrics (Time-Series)

```sql
-- Use TimescaleDB for time-series metrics
CREATE TABLE run_metrics (
  time TIMESTAMPTZ NOT NULL,
  run_id UUID NOT NULL REFERENCES simulation_runs(id),
  metric_name TEXT NOT NULL,
  metric_value DECIMAL NOT NULL,
  tags JSONB  -- {block_id, operation_type, etc.}
);

-- Convert to hypertable (TimescaleDB)
SELECT create_hypertable('run_metrics', 'time');

CREATE INDEX idx_metrics_run ON run_metrics(run_id, time DESC);
CREATE INDEX idx_metrics_name ON run_metrics(metric_name);
```

**Example Metrics**:
```typescript
// Throughput over time (sampled every 100ms)
{
  time: '2026-01-20T10:30:00.000Z',
  run_id: 'uuid',
  metric_name: 'throughput_qps',
  metric_value: 1234.5,
  tags: {}
}

// Block-specific metric
{
  time: '2026-01-20T10:30:00.100Z',
  run_id: 'uuid',
  metric_name: 'cache_hit_ratio',
  metric_value: 0.87,
  tags: { block_id: 'lru-buffer-1' }
}
```

**Why TimescaleDB?**
- ✅ Efficient time-range queries ("show metrics from 10:30:00 to 10:31:00")
- ✅ Automatic partitioning (old data compressed)
- ✅ Built-in aggregations (avg, p95, etc.)

---

## Part 4: API Design

### API Style: REST + GraphQL Hybrid

**REST** for commands (CRUD, actions):
- `POST /api/designs` → Create design
- `POST /api/runs` → Save simulation result

**GraphQL** for complex queries:
- Fetch design + all runs + metrics in one query
- Avoid N+1 queries

**Why hybrid?**
- ✅ REST is simple for mutations (idempotent, cacheable)
- ✅ GraphQL is powerful for reads (flexible, efficient)
- ❌ Adds complexity (two protocols)

**Decision**: Start with REST (simpler), add GraphQL in Phase 2 if needed.

---

### Canvas Service APIs

#### Create Design

```
POST /api/designs
Content-Type: application/json
Authorization: Bearer <token>

{
  "name": "OLTP with LSM Tree",
  "description": "High-throughput write design",
  "canvas_state": {
    "blocks": [...],
    "connections": [...]
  }
}

Response 201:
{
  "id": "uuid",
  "name": "OLTP with LSM Tree",
  "created_at": "2026-01-20T10:30:00Z",
  "version": 1
}
```

**Idempotency**: Use `Idempotency-Key` header to prevent duplicates.

---

#### Update Design (Optimistic Locking)

```
PUT /api/designs/:id
Content-Type: application/json
If-Match: "v2"  ← ETag from previous GET

{
  "canvas_state": {...}
}

Response 200:
{
  "id": "uuid",
  "version": 3,
  "updated_at": "2026-01-20T10:35:00Z"
}

Response 412 (Precondition Failed):
{
  "error": "conflict",
  "message": "Design was modified by another user",
  "current_version": 4
}
```

**Why ETags?**
- ✅ Prevents lost updates (concurrent edits)
- ✅ Standard HTTP mechanism
- ❌ Requires clients to handle conflicts

---

#### Fetch Design with Versions

```
GET /api/designs/:id?include=versions

Response 200:
{
  "id": "uuid",
  "name": "OLTP with LSM Tree",
  "canvas_state": {...},
  "version": 5,
  "versions": [
    {
      "version": 5,
      "created_at": "2026-01-20T11:00:00Z",
      "commit_message": "Added buffer pool"
    },
    {
      "version": 4,
      "created_at": "2026-01-20T10:45:00Z",
      "commit_message": "Swapped heap for LSM"
    }
  ]
}
```

---

### Simulation Results APIs

#### Save Simulation Result

```
POST /api/runs
Content-Type: application/json

{
  "design_id": "uuid",
  "workload_id": "uuid",
  "metrics": {
    "throughput_qps": 1234,
    "latency_p99_ms": 8.1,
    "cache_hit_ratio": 0.87,
    ...
  },
  "events": [
    {
      "sequence_number": 1,
      "event_type": "page_read",
      "block_id": "heap-1",
      "payload": {...}
    },
    ...
  ]
}

Response 201:
{
  "id": "uuid",
  "status": "completed",
  "created_at": "2026-01-20T10:30:00Z"
}
```

**Design Considerations**:
- Events can be huge (10K operations = 100K+ events). Options:
  1. **Batch POST**: Send events in chunks (1000 at a time)
  2. **Stream**: Use Server-Sent Events (SSE) or WebSocket
  3. **Compress**: gzip request body

**Decision**: Use gzip + batch POST (simplest).

---

#### Compare Runs

```
GET /api/runs/compare?ids=uuid1,uuid2,uuid3

Response 200:
{
  "runs": [
    {
      "id": "uuid1",
      "design_name": "Heap + B-tree",
      "throughput_qps": 1234,
      "latency_p99_ms": 8.1
    },
    {
      "id": "uuid2",
      "design_name": "LSM + Skip List",
      "throughput_qps": 1856,
      "latency_p99_ms": 12.3
    }
  ],
  "winner": {
    "by_throughput": "uuid2",
    "by_latency": "uuid1"
  },
  "summary": [
    "Design 2 has 50% higher throughput",
    "Design 1 has 35% lower latency"
  ]
}
```

**Why structured summaries?**
- ✅ Frontend can display without parsing
- ✅ AI-generated or rule-based insights

---

## Part 5: Simulation Engine Architecture

### Client-Side Engine (TypeScript/WASM)

**Why client-side?**
- ✅ Zero backend cost (scales to millions)
- ✅ Instant feedback (`<100ms`)
- ✅ Works offline
- ❌ Limited by browser memory

**Architecture**:

```
┌────────────────────────────────────────────┐
│          Simulation Orchestrator           │
│  (Compiles canvas → execution graph)       │
└───────────┬────────────────────────────────┘
            │
            ├── Initialize Blocks
            ├── Build Execution Plan
            ├── Execute Workload
            └── Collect Metrics
                    │
        ┌───────────┴────────────┐
        │                        │
        ▼                        ▼
┌───────────────┐     ┌──────────────────┐
│  Block Layer  │     │  Metrics Layer   │
│ (Heap, B-tree)│     │ (Counters, Histo)│
└───────────────┘     └──────────────────┘
```

### Block Execution Model

**Interface**:
```typescript
interface Block {
  id: string;
  type: string;

  // Lifecycle
  initialize(params: BlockParameters): void;
  execute(input: BlockInput): BlockOutput;
  getMetrics(): BlockMetrics;

  // Event emission
  emitEvent(event: SimulationEvent): void;
}
```

**Example: Heap File Storage Block**

```typescript
class HeapFileStorage implements Block {
  private pages: Page[] = [];
  private pageSize: number;
  private fillFactor: number;
  private metricsCollector: MetricsCollector;

  initialize(params: BlockParameters): void {
    this.pageSize = params.page_size || 8192;
    this.fillFactor = params.fill_factor || 0.9;
  }

  execute(input: BlockInput): BlockOutput {
    if (input.operation === 'INSERT') {
      return this.insert(input.record);
    } else if (input.operation === 'SCAN') {
      return this.scan();
    }
  }

  private insert(record: Record): BlockOutput {
    // Find page with space
    let targetPage = this.findPageWithSpace(record.size);

    if (!targetPage) {
      // Allocate new page
      targetPage = this.allocatePage();
      this.emitEvent({
        type: 'page_allocated',
        payload: { page_id: targetPage.id }
      });
    }

    // Write record
    targetPage.writeRecord(record);
    this.metricsCollector.increment('pages_written', 1);

    this.emitEvent({
      type: 'record_inserted',
      payload: {
        page_id: targetPage.id,
        record_id: record.id
      }
    });

    return { success: true, page_id: targetPage.id };
  }

  private scan(): BlockOutput {
    const records: Record[] = [];

    for (const page of this.pages) {
      // Simulate disk read latency
      this.simulateDiskRead();

      this.emitEvent({
        type: 'page_read',
        payload: {
          page_id: page.id,
          from_cache: false,
          latency_us: 1500
        }
      });

      records.push(...page.records);
    }

    this.metricsCollector.increment('pages_read', this.pages.length);
    return { records };
  }

  getMetrics(): BlockMetrics {
    return {
      pages_allocated: this.pages.length,
      pages_written: this.metricsCollector.get('pages_written'),
      pages_read: this.metricsCollector.get('pages_read')
    };
  }
}
```

---

### Storage Layer Simulation

**Page Structure** (conceptual):

```typescript
interface Page {
  id: number;
  size: number;          // e.g., 8192 bytes
  used: number;          // Bytes used
  records: Record[];     // Simplified (not byte-accurate)
}

interface Record {
  id: number;
  data: any;             // Simplified (not actual bytes)
  size: number;          // Estimated size in bytes
}
```

**Heap vs Clustered Storage**:

```typescript
// Heap: Unordered, insert anywhere
class HeapStorage {
  insert(record: Record): void {
    const page = this.findPageWithSpace(record.size);
    page.append(record);  // No ordering
  }

  scan(): Record[] {
    // Full table scan (all pages)
    return this.pages.flatMap(p => p.records);
  }
}

// Clustered: Ordered by primary key
class ClusteredStorage {
  insert(record: Record): void {
    const page = this.findPageForKey(record.key);
    page.insertSorted(record);  // Maintain order

    if (page.isFull()) {
      this.splitPage(page);  // B-tree split logic
    }
  }

  rangeScan(min: Key, max: Key): Record[] {
    // Efficient range scan (sorted order)
    return this.pages
      .filter(p => p.minKey >= min && p.maxKey <= max)
      .flatMap(p => p.records);
  }
}
```

---

### Index Layer Simulation

**B-tree Index**:

```typescript
class BTreeIndex {
  private root: BTreeNode;
  private fanout: number;

  lookup(key: Key): RecordID | null {
    let node = this.root;
    let depth = 0;

    while (!node.isLeaf) {
      this.emitEvent({
        type: 'btree_traverse',
        payload: { depth, node_id: node.id }
      });

      node = node.findChild(key);
      depth++;
    }

    this.metricsCollector.increment('btree_lookups', 1);
    this.metricsCollector.recordHistogram('btree_depth', depth);

    return node.findRecord(key);
  }

  insert(key: Key, recordId: RecordID): void {
    const leaf = this.findLeaf(key);
    leaf.insert(key, recordId);

    if (leaf.isFull()) {
      this.split(leaf);  // May propagate up to root
      this.metricsCollector.increment('btree_splits', 1);
    }
  }
}
```

**LSM Tree Index**:

```typescript
class LSMTreeIndex {
  private memtable: Map<Key, RecordID>;
  private sstables: SSTable[];
  private memtableThreshold: number;

  insert(key: Key, recordId: RecordID): void {
    this.memtable.set(key, recordId);

    if (this.memtable.size >= this.memtableThreshold) {
      this.flush();
    }
  }

  lookup(key: Key): RecordID | null {
    // Check memtable first
    if (this.memtable.has(key)) {
      return this.memtable.get(key);
    }

    // Check SSTables (newest to oldest)
    for (const sstable of this.sstables) {
      const result = sstable.lookup(key);
      if (result) return result;

      this.metricsCollector.increment('sstable_lookups', 1);
    }

    return null;
  }

  private flush(): void {
    // Convert memtable to SSTable
    const sstable = new SSTable(this.memtable);
    this.sstables.unshift(sstable);
    this.memtable.clear();

    this.emitEvent({
      type: 'lsm_flush',
      payload: { sstable_id: sstable.id, keys: sstable.size }
    });

    // Trigger compaction if needed
    if (this.sstables.length > 5) {
      this.compact();
    }
  }

  private compact(): void {
    // Merge oldest SSTables
    const toMerge = this.sstables.slice(-3);
    const merged = this.mergeSSTables(toMerge);

    this.sstables = this.sstables.slice(0, -3).concat(merged);

    this.emitEvent({
      type: 'lsm_compaction',
      payload: { merged_count: 3 }
    });
  }
}
```

---

### Buffer Pool Simulation

**LRU Buffer Pool**:

```typescript
class LRUBufferPool {
  private cache: Map<PageID, Page>;
  private lruList: PageID[];
  private capacity: number;

  getPage(pageId: PageID, storage: Storage): Page {
    if (this.cache.has(pageId)) {
      // Cache hit
      this.moveToFront(pageId);
      this.metricsCollector.increment('cache_hits', 1);

      return this.cache.get(pageId);
    }

    // Cache miss - load from storage
    this.metricsCollector.increment('cache_misses', 1);
    const page = storage.readPage(pageId);

    this.emitEvent({
      type: 'page_read_from_disk',
      payload: { page_id: pageId, latency_us: 1500 }
    });

    // Evict if full
    if (this.cache.size >= this.capacity) {
      const evicted = this.evict();
      this.emitEvent({
        type: 'page_evicted',
        payload: { page_id: evicted }
      });
    }

    this.cache.set(pageId, page);
    this.lruList.unshift(pageId);

    return page;
  }

  private evict(): PageID {
    const evicted = this.lruList.pop();
    this.cache.delete(evicted);
    return evicted;
  }
}
```

---

### Query Execution Simulation

**Sequential Scan**:

```typescript
class SequentialScan {
  execute(input: { storage: Storage }): Record[] {
    const records: Record[] = [];
    const pageCount = input.storage.getPageCount();

    for (let i = 0; i < pageCount; i++) {
      const page = input.storage.readPage(i);

      this.emitEvent({
        type: 'page_scan',
        payload: { page_id: i }
      });

      records.push(...page.records);
    }

    this.metricsCollector.increment('pages_scanned', pageCount);
    return records;
  }
}
```

**Index Scan**:

```typescript
class IndexScan {
  execute(input: { index: Index, predicate: Predicate }): Record[] {
    const recordIds = input.index.lookup(input.predicate.key);

    // For each record ID, fetch from storage
    const records = recordIds.map(id => {
      return input.storage.getRecord(id);
    });

    this.metricsCollector.increment('index_lookups', recordIds.length);
    return records;
  }
}
```

**Hash Join**:

```typescript
class HashJoin {
  execute(input: { left: Record[], right: Record[], joinKey: string }): Record[] {
    // Build phase: Hash smaller table
    const buildTable = input.left.length < input.right.length ? input.left : input.right;
    const probeTable = buildTable === input.left ? input.right : input.left;

    const hashTable = new Map<any, Record[]>();
    for (const record of buildTable) {
      const key = record[input.joinKey];
      if (!hashTable.has(key)) {
        hashTable.set(key, []);
      }
      hashTable.get(key).push(record);
    }

    this.emitEvent({
      type: 'hash_join_build',
      payload: { build_size: buildTable.length }
    });

    // Probe phase
    const results: Record[] = [];
    for (const record of probeTable) {
      const key = record[input.joinKey];
      const matches = hashTable.get(key) || [];

      for (const match of matches) {
        results.push({ ...record, ...match });
      }
    }

    this.emitEvent({
      type: 'hash_join_probe',
      payload: { probe_size: probeTable.length, matches: results.length }
    });

    return results;
  }
}
```

---

### Transaction & Concurrency Simulation

**Two-Phase Locking**:

```typescript
class TwoPhaseLocker {
  private locks: Map<RecordID, { holder: TxnID, mode: 'shared' | 'exclusive' }>;
  private waitGraph: Map<TxnID, Set<TxnID>>;

  acquireLock(txnId: TxnID, recordId: RecordID, mode: LockMode): boolean {
    const currentLock = this.locks.get(recordId);

    if (!currentLock) {
      // No lock held, grant immediately
      this.locks.set(recordId, { holder: txnId, mode });
      return true;
    }

    if (currentLock.holder === txnId) {
      // Already holding lock
      return true;
    }

    // Conflict - add to wait graph
    this.waitGraph.get(txnId).add(currentLock.holder);

    this.emitEvent({
      type: 'lock_wait',
      payload: { txn_id: txnId, waiting_for: currentLock.holder }
    });

    // Check for deadlock
    if (this.detectDeadlock(txnId)) {
      this.emitEvent({
        type: 'deadlock_detected',
        payload: { victim_txn: txnId }
      });

      throw new Error('Deadlock detected, aborting transaction');
    }

    return false;  // Must wait
  }

  releaseLock(txnId: TxnID, recordId: RecordID): void {
    this.locks.delete(recordId);

    // Wake waiting transactions
    for (const [waitingTxn, waitingFor] of this.waitGraph) {
      if (waitingFor.has(txnId)) {
        waitingFor.delete(txnId);
      }
    }
  }

  private detectDeadlock(txnId: TxnID): boolean {
    // DFS to find cycle in wait graph
    const visited = new Set<TxnID>();
    const stack = new Set<TxnID>();

    const hasCycle = (node: TxnID): boolean => {
      if (stack.has(node)) return true;
      if (visited.has(node)) return false;

      visited.add(node);
      stack.add(node);

      for (const neighbor of this.waitGraph.get(node) || []) {
        if (hasCycle(neighbor)) return true;
      }

      stack.delete(node);
      return false;
    };

    return hasCycle(txnId);
  }
}
```

**MVCC (Simplified)**:

```typescript
class MVCCManager {
  private versions: Map<RecordID, RecordVersion[]>;
  private activeTransactions: Map<TxnID, Snapshot>;

  beginTransaction(): TxnID {
    const txnId = this.generateTxnId();
    const snapshot = this.createSnapshot();

    this.activeTransactions.set(txnId, snapshot);
    return txnId;
  }

  read(txnId: TxnID, recordId: RecordID): Record | null {
    const snapshot = this.activeTransactions.get(txnId);
    const versions = this.versions.get(recordId) || [];

    // Find version visible to this transaction
    for (const version of versions) {
      if (version.xmin <= snapshot.xmax && !snapshot.inProgress.has(version.xmin)) {
        if (version.xmax === null || version.xmax > snapshot.xmax) {
          this.emitEvent({
            type: 'mvcc_read',
            payload: { txn_id: txnId, version_id: version.xmin }
          });

          return version.data;
        }
      }
    }

    return null;  // No visible version
  }

  write(txnId: TxnID, recordId: RecordID, data: any): void {
    const versions = this.versions.get(recordId) || [];

    // Create new version
    versions.push({
      xmin: txnId,
      xmax: null,
      data: data
    });

    this.versions.set(recordId, versions);

    this.emitEvent({
      type: 'mvcc_write',
      payload: { txn_id: txnId, record_id: recordId }
    });
  }

  commit(txnId: TxnID): void {
    // Mark old versions as expired
    for (const [recordId, versions] of this.versions) {
      for (const version of versions) {
        if (version.xmin === txnId) {
          // New version created by this txn
        }
        if (version.xmax === null && this.wasUpdatedBy(recordId, txnId)) {
          version.xmax = txnId;  // Mark as deleted
        }
      }
    }

    this.activeTransactions.delete(txnId);
  }
}
```

---

## Part 6: Performance & Scalability

### Client-Side Performance

**Constraints**:
- Browser memory: ~2GB practical limit
- Single-threaded (unless Web Workers)
- Simulation must complete in `<5` seconds (UX requirement)

**Optimizations**:

**1. Sampling**:
```typescript
// Don't simulate 1M records, simulate 10K and extrapolate
const SAMPLE_SIZE = 10000;
const TOTAL_RECORDS = 1000000;
const SCALE_FACTOR = TOTAL_RECORDS / SAMPLE_SIZE;

// Simulate with sample
const metrics = runSimulation(SAMPLE_SIZE);

// Extrapolate
metrics.throughput_qps *= SCALE_FACTOR;
metrics.total_io_pages *= SCALE_FACTOR;
```

**2. Coarse-Grained Simulation**:
```typescript
// Don't simulate byte-level page layout, use simplified model
class SimplifiedPage {
  capacity: number = 8192;  // Bytes
  used: number = 0;
  recordCount: number = 0;

  canFit(recordSize: number): boolean {
    return this.used + recordSize <= this.capacity * FILL_FACTOR;
  }
}
```

**3. Web Workers** (Phase 2):
```typescript
// Offload simulation to background thread
const worker = new Worker('simulation-worker.js');

worker.postMessage({
  design: canvasState,
  workload: workloadConfig
});

worker.onmessage = (e) => {
  const results = e.data;
  displayMetrics(results);
};
```

---

### Backend Scalability

**Read-Heavy** (99% reads: fetch designs, compare runs):
- **Caching**: Redis for hot designs, CDN for templates
- **Read Replicas**: PostgreSQL streaming replication
- **Indexed Queries**: Ensure all queries use indexes

**Write-Heavy** (1% writes: save results):
- **Batch Inserts**: Insert 1000 events at once (single transaction)
- **Connection Pooling**: PgBouncer (1000 concurrent users → 20 DB connections)
- **Async Jobs**: Background workers for heavy processing

**Metrics**:
- **Target**: 10,000 concurrent users
- **Design fetch**: `<50ms` (cached), `<200ms` (DB)
- **Run save**: `<500ms` (batch insert)
- **Compare query**: `<1s` (indexed)

**Scaling Strategy**:
```
Phase 1 (MVP):  Single region, single DB, no caching
                Supports: 100 concurrent users

Phase 2:        Add Redis, read replicas
                Supports: 1,000 concurrent users

Phase 3:        Multi-region, CDN, sharding
                Supports: 10,000+ concurrent users
```

---

## Part 7: Observability & Debugging

### Logging Strategy

**Structured Logging** (JSON format):
```json
{
  "timestamp": "2026-01-20T10:30:00.123Z",
  "level": "INFO",
  "service": "canvas-service",
  "user_id": "uuid",
  "design_id": "uuid",
  "action": "design_updated",
  "duration_ms": 45,
  "metadata": {
    "blocks_added": 2,
    "connections_added": 3
  }
}
```

**Log Levels**:
- **DEBUG**: Detailed internal state (not in production)
- **INFO**: User actions (design created, simulation started)
- **WARN**: Degraded performance (slow queries, cache misses)
- **ERROR**: Failures (DB errors, validation failures)

---

### Event Tracing

**OpenTelemetry** for distributed tracing:
```
User Request → Canvas API → DB Query → Response
     ├─ fetch_design (50ms)
     │   ├─ query_designs_table (30ms)
     │   └─ query_versions_table (20ms)
     └─ serialize_response (5ms)
```

**Key Traces**:
- `GET /api/designs/:id` (measure DB query time)
- `POST /api/runs` (measure event insertion time)
- `GET /api/runs/compare` (measure aggregation time)

---

### Metrics

**Application Metrics** (Prometheus):
```
# Request rate
http_requests_total{method="GET", endpoint="/api/designs", status="200"}

# Latency
http_request_duration_seconds{endpoint="/api/designs", quantile="0.99"}

# Error rate
http_requests_total{method="POST", endpoint="/api/runs", status="500"}

# Database
db_connections_active
db_query_duration_seconds{query="fetch_design"}

# Business metrics
designs_created_total
simulation_runs_completed_total
simulation_runs_failed_total
```

**Dashboards**:
- **Operational**: Request rate, latency, error rate, DB health
- **Business**: Designs created/day, runs executed/day, popular blocks

---

### Debugging Simulation Runs

**Replay Simulation from Events**:
```typescript
// Load events from database
const events = await db.query(
  'SELECT * FROM run_events WHERE run_id = ? ORDER BY sequence_number',
  [runId]
);

// Replay step-by-step
for (const event of events) {
  const block = blocks.get(event.block_id);
  block.applyEvent(event);

  console.log(`Step ${event.sequence_number}: ${event.event_type}`);
  console.log(JSON.stringify(event.payload, null, 2));
}
```

**Visualization**:
- **Timeline**: Show events on a timeline (X-axis: time, Y-axis: blocks)
- **Gantt Chart**: Show which blocks were active when
- **Flamegraph**: Show where time was spent

---

## Part 8: Implementation Phases

### Phase 1: MVP (8 weeks)

**Week 1-2**: Backend scaffolding
- [ ] Set up Node.js + TypeScript project
- [ ] PostgreSQL schema (designs, workloads, runs tables)
- [ ] REST API skeleton (Express or Fastify)
- [ ] Authentication (JWT-based)

**Week 3-4**: Canvas APIs
- [ ] POST/GET/PUT/DELETE /api/designs
- [ ] Versioning (design_versions table)
- [ ] Templates (seed data)

**Week 5-6**: Results APIs
- [ ] POST /api/runs (save simulation results)
- [ ] GET /api/runs/compare
- [ ] Event storage (run_events table)

**Week 7-8**: Polish
- [ ] Error handling (proper HTTP status codes)
- [ ] Validation (JSON schema)
- [ ] Logging (Winston + structured logs)
- [ ] Deployment (Docker + Kubernetes)

**Deliverable**: Working backend that supports MVP frontend.

---

### Phase 2: Optimization (4 weeks)

**Week 9-10**: Performance
- [ ] Add Redis caching
- [ ] DB query optimization (EXPLAIN ANALYZE all queries)
- [ ] Connection pooling (PgBouncer)
- [ ] Load testing (k6 or Locust)

**Week 11-12**: Advanced features
- [ ] Backend simulation execution (async jobs)
- [ ] GraphQL API (optional)
- [ ] Analytics endpoints (popular blocks, benchmarks)

---

### Phase 3: Scale (8 weeks)

**Week 13-16**: Scalability
- [ ] Read replicas
- [ ] Multi-region deployment
- [ ] CDN for static assets

**Week 17-20**: Advanced observability
- [ ] OpenTelemetry integration
- [ ] Custom dashboards (Grafana)
- [ ] Alerting (PagerDuty)

---

## Part 9: Technology Stack

### Backend

**Language**: Node.js + TypeScript
- ✅ Same language as client-side (code reuse)
- ✅ Great ecosystem (libraries for everything)
- ✅ Fast iteration
- ❌ Single-threaded (mitigate: clustering, workers)

**Alternative**: Go
- ✅ Better performance (10x faster than Node)
- ✅ Built-in concurrency
- ❌ Separate language (no code reuse)

**Decision**: Node.js for MVP (faster development), consider Go for Phase 3 if performance becomes an issue.

---

**Framework**: Fastify
- ✅ Fastest Node.js framework
- ✅ Schema validation built-in (JSON Schema)
- ✅ Plugin ecosystem

**Alternative**: Express
- ✅ Most popular (more resources)
- ❌ Slower than Fastify

**Decision**: Fastify (performance matters).

---

### Database

**Primary**: PostgreSQL 15+
- ✅ Robust, battle-tested
- ✅ JSONB for flexible schemas
- ✅ TimescaleDB extension for time-series
- ✅ Full-text search (for block library)

**Why not NoSQL?**
- ❌ Need ACID transactions (compare runs atomically)
- ❌ Need complex queries (JOIN designs + runs + metrics)
- ❌ JSONB gives us flexibility where needed

---

**Caching**: Redis
- ✅ Fast (sub-millisecond reads)
- ✅ TTL support (auto-expire stale data)
- ✅ Simple (key-value store)

**Cache Strategy**:
- **Designs**: Cache for 5 minutes
- **Templates**: Cache indefinitely (rarely change)
- **Run results**: Don't cache (always fresh)

---

### Job Queue (Phase 2)

**BullMQ** (Redis-based):
- ✅ Reliable (retry, exponential backoff)
- ✅ Priority queues
- ✅ Rate limiting

**Use Cases**:
- Long-running simulations (backend execution)
- Report generation (PDF exports)
- Analytics aggregation (nightly jobs)

---

### Observability

**Logging**: Winston + ELK Stack
- Winston: Structured JSON logs
- Elasticsearch: Store and search logs
- Kibana: Visualize logs

**Metrics**: Prometheus + Grafana
- Prometheus: Scrape metrics
- Grafana: Visualize dashboards

**Tracing**: OpenTelemetry + Jaeger
- OpenTelemetry: Instrument code
- Jaeger: Visualize traces

---

## Part 10: Security Considerations

### Authentication & Authorization

**JWT-based auth**:
```
1. User logs in → Backend returns JWT
2. Client stores JWT (localStorage or httpOnly cookie)
3. All requests include: Authorization: Bearer <jwt>
4. Backend validates JWT signature
```

**Authorization Levels**:
- **Anonymous**: View public designs, templates
- **Authenticated**: Create designs, run simulations, save results
- **Premium**: Backend execution, priority queue, unlimited storage

---

### Data Privacy

**User Data**:
- Designs are private by default (user_id FK)
- Users can make designs public (share link)
- Soft deletes (never hard-delete user data)

**Compliance**:
- GDPR: Support "right to be forgotten" (DELETE /api/users/:id)
- Data export: GET /api/users/:id/export (JSON dump)

---

### Input Validation

**JSON Schema** for all API requests:
```typescript
const createDesignSchema = {
  type: 'object',
  required: ['name', 'canvas_state'],
  properties: {
    name: { type: 'string', minLength: 1, maxLength: 255 },
    description: { type: 'string', maxLength: 5000 },
    canvas_state: {
      type: 'object',
      required: ['blocks', 'connections'],
      properties: {
        blocks: { type: 'array', maxItems: 100 },
        connections: { type: 'array', maxItems: 200 }
      }
    }
  }
};
```

**Rate Limiting**:
```typescript
// Per user: 100 requests/minute
fastify.register(require('@fastify/rate-limit'), {
  max: 100,
  timeWindow: '1 minute'
});
```

---

## Part 11: Open Questions & Future Work

### Open Questions

1. **WebAssembly for simulation?**
   - Pros: 10x faster than TypeScript
   - Cons: Harder to debug, larger bundle size
   - **Decision**: Start with TypeScript, profile, move hot paths to WASM if needed

2. **Real-time collaboration?**
   - Multiple users editing same design simultaneously
   - Requires: WebSocket, OT/CRDT for conflict resolution
   - **Decision**: Phase 3 feature (complex, not MVP-critical)

3. **Backend simulation execution?**
   - Run simulations server-side (no browser limits)
   - Requires: Job queue, worker pool, resource isolation
   - **Decision**: Phase 2 (nice-to-have, not MVP)

---

### Future Work

**Phase 4: Machine Learning**
- Train models on user data (which designs are fastest)
- Auto-suggest optimizations (AI co-pilot)
- Predict performance without running simulation

**Phase 5: Distributed Database Simulation**
- Sharding, replication, consistency protocols
- Simulate network latency, partitions
- CAP theorem trade-offs

**Phase 6: Production Code Generation**
- Export design → Generate RocksDB config
- Export design → Generate SQL DDL
- One-click deploy to cloud (AWS RDS, GCP CloudSQL)

---

## Conclusion

This backend architecture is designed to:
1. **Support the canvas** (CRUD, versioning, sharing)
2. **Persist simulation results** (metrics, events, traces)
3. **Enable comparison** (side-by-side, analytics)
4. **Scale to millions** (caching, read replicas, async jobs)
5. **Be observable** (logging, metrics, tracing)

**Key Trade-offs**:
- **Client-side simulation**: Fast, free, but limited by browser
- **Event sourcing**: Full replay capability, but storage cost
- **PostgreSQL**: Flexible (JSONB), but not as fast as specialized DBs
- **Node.js**: Fast development, but not as fast as Go/Rust

**Recommendation**: Start with this architecture for MVP, measure, optimize based on real user data.

---

**Next Steps**:
1. Review and approve architecture
2. Set up backend repo (Node.js + TypeScript + Fastify)
3. Create PostgreSQL schema (migrations)
4. Implement Canvas Service APIs (Week 1-2)
5. Integrate with frontend (Week 3-4)

**End of Backend Implementation Plan**
