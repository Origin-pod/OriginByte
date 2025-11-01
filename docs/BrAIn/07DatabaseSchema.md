# Brain AI - Database Schema

## Overview
This document outlines the PostgreSQL database schema for the Brain AI application, designed to support voice-first interactions, task management, and AI-powered insights.

## Tables

### 1. Users
Stores user account information and preferences.

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  subscription_tier TEXT DEFAULT 'free' CHECK (subscription_tier IN ('free', 'pro', 'enterprise')),
  settings JSONB DEFAULT '{"theme": "light", "notifications": true}',
  last_active_at TIMESTAMP WITH TIME ZONE
);
```

### 2. Voice Entries
Core table for storing all voice recordings and their transcriptions.

```sql
CREATE TABLE voice_entries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  original_audio_url TEXT,
  transcript TEXT NOT NULL,
  processed_content JSONB,
  embedding VECTOR(1536),
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  
  -- Add constraints
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 3. Tasks
Manages tasks extracted from voice entries.

```sql
CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  voice_entry_id UUID REFERENCES voice_entries(id) ON DELETE SET NULL,
  title TEXT NOT NULL,
  description TEXT,
  due_date TIMESTAMP WITH TIME ZONE,
  status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
  priority INTEGER DEFAULT 1 CHECK (priority BETWEEN 1 AND 5),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  
  -- Indexes for common queries
  CONSTRAINT fk_user_task FOREIGN KEY (user_id) REFERENCES users(id),
  CONSTRAINT fk_voice_entry FOREIGN KEY (voice_entry_id) REFERENCES voice_entries(id)
);
```

### 4. Tags
Organizes content with user-defined tags.

```sql
CREATE TABLE tags (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  color TEXT DEFAULT '#3B82F6' CHECK (color ~ '^#[0-9a-fA-F]{6}$'),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  
  -- Ensure tag names are unique per user
  CONSTRAINT unique_user_tag UNIQUE (user_id, name)
);
```

### 5. Entry Tags
Many-to-many relationship between voice entries and tags.

```sql
CREATE TABLE entry_tags (
  entry_id UUID NOT NULL REFERENCES voice_entries(id) ON DELETE CASCADE,
  tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  
  PRIMARY KEY (entry_id, tag_id)
);
```

### 6. Reflections
Stores AI-generated insights and summaries.

```sql
CREATE TABLE reflections (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  period_type TEXT NOT NULL CHECK (period_type IN ('day', 'week', 'month')),
  period_start TIMESTAMP WITH TIME ZONE NOT NULL,
  period_end TIMESTAMP WITH TIME ZONE NOT NULL,
  insights JSONB NOT NULL DEFAULT '{}',
  summary TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  
  -- One reflection per user per period
  CONSTRAINT unique_reflection_period UNIQUE (user_id, period_type, period_start)
);
```

## Indexes

```sql
-- Performance optimization for common queries
CREATE INDEX idx_voice_entries_user_created ON voice_entries(user_id, created_at DESC);
CREATE INDEX idx_tasks_user_status ON tasks(user_id, status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date) WHERE status = 'pending';
CREATE INDEX idx_voice_entries_embedding ON voice_entries USING hnsw (embedding vector_cosine_ops);
CREATE INDEX idx_tags_user ON tags(user_id);
CREATE INDEX idx_reflections_user_period ON reflections(user_id, period_type, period_start DESC);
```

## Data Retention Policy

```sql
-- Example: Automatically archive voice entries older than 1 year
CREATE OR REPLACE FUNCTION archive_old_entries()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO archived_voice_entries
  SELECT * FROM voice_entries 
  WHERE created_at < NOW() - INTERVAL '1 year';
  
  DELETE FROM voice_entries 
  WHERE created_at < NOW() - INTERVAL '1 year';
  
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Schedule the archiving job (runs daily at 2 AM)
SELECT cron.schedule(
  'archive-old-entries',
  '0 2 * * *',
  $$DELETE FROM voice_entries WHERE created_at < NOW() - INTERVAL '1 year'$$
);
```

## Security Considerations

1. Row-level security (RLS) is enabled on all tables
2. Sensitive data is encrypted at rest
3. Regular backups are performed
4. Access is restricted to application users with least privilege

## Performance Considerations

1. Vector indexes for semantic search
2. Appropriate indexing on frequently queried columns
3. Partitioning for large tables (e.g., voice_entries by date)
4. Regular VACUUM and ANALYZE operations

## Schema Migrations

All schema changes are managed through version-controlled migration files using a migration tool like Flyway or Node-PG-Migrate.
