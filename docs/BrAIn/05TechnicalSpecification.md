# Brain AI - Technical Specification

## 1. Core User Flows

### 1.1 Primary Flow: Voice Capture → Understanding → Action
```
User → Clicks "Start Talking" → Browser mic activates → Real-time transcription
→ Speech processed for intent → Structured data created
→ Voice confirmation → Entry appears in timeline
```

### 1.2 Secondary Flow: Voice Search & Recall
- Voice query processing
- Semantic search across entries
- Returns relevant clips with text summaries
- Audio replay and entry editing

### 1.3 Tertiary Flow: Reflection & Planning
- Weekly/Monthly summaries
- Pattern analysis
- Task prioritization
- Voice-guided planning

## 2. System Architecture

### 2.1 High-Level Architecture
```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│    Browser      │     │    Backend API   │     │   External APIs │
│   (React App)   │◄──►│    (Node.js)     │◄──►│   OpenAI,       │
│                 │     │                  │     │   Deepgram,     │
│ • Voice Capture │     │ • Auth           │     │   ElevenLabs    │
│ • Real-time UI  │     │ • Processing     │     │                 │
│ • Audio Stream  │     │ • Storage        │     └─────────────────┘
└─────────────────┘     │ • Search         │     │
                        │ • AI Logic       │     ▼
                        └──────────────────┘     ┌─────────────────┐
                                                │    Database     │
                        ┌──────────────────┐     │   (Supabase)    │
                        │   File Storage   │     │                 │
                        │   (Cloudflare)   │     │ • Entries      │
                        │                  │     │ • Embeddings   │
                        └──────────────────┘     │ • Users        │
                                                └─────────────────┘
```

### 2.2 Tech Stack

#### Frontend:
- **Framework**: Next.js 14 (React + SSR/SSG)
- **Styling**: Tailwind CSS
- **Voice**: Web Speech API + Deepgram SDK
- **State**: Zustand
- **Animations**: Framer Motion
- **Real-time**: WebSocket

#### Backend:
- **Runtime**: Node.js + Express/Bun
- **Database**: Supabase (PostgreSQL)
- **AI/ML**:
  - OpenAI API (GPT-4)
  - Deepgram (STT)
  - ElevenLabs (TTS)

#### Infrastructure:
- **Hosting**: Vercel (frontend)
- **Edge**: Cloudflare Workers
- **Storage**: Cloudflare R2
- **Auth**: Supabase Auth
- **CI/CD**: GitHub Actions

## 3. Database Schema

### 3.1 Users
```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  subscription_tier TEXT DEFAULT 'free',
  settings JSONB DEFAULT '{}'
);
```

### 3.2 Voice Entries
```sql
CREATE TABLE voice_entries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  original_audio_url TEXT,
  transcript TEXT NOT NULL,
  processed_content JSONB,
  embedding VECTOR(1536),
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);
```

### 3.3 Tasks
```sql
CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  voice_entry_id UUID REFERENCES voice_entries(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  description TEXT,
  due_date TIMESTAMP,
  status TEXT DEFAULT 'pending',
  priority INTEGER DEFAULT 1,
  created_at TIMESTAMP DEFAULT NOW()
);
```

## 4. API Endpoints

### 4.1 Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `GET /api/auth/me` - Get current user
- `POST /api/auth/logout` - Logout

### 4.2 Voice Processing
- `POST /api/voice/transcribe` - Real-time transcription
- `POST /api/voice/process` - Process voice input
- `GET /api/voice/search` - Semantic search
- `POST /api/voice/speak` - Text-to-speech

### 4.3 Entries
- `GET /api/entries` - List entries
- `POST /api/entries` - Create entry
- `GET /api/entries/:id` - Get entry
- `PUT /api/entries/:id` - Update entry
- `DELETE /api/entries/:id` - Delete entry

### 4.4 Tasks
- `GET /api/tasks` - List tasks
- `POST /api/tasks` - Create task
- `GET /api/tasks/:id` - Get task
- `PUT /api/tasks/:id` - Update task
- `DELETE /api/tasks/:id` - Delete task

## 5. Development Roadmap

### 5.1 MVP (4-6 weeks)
1. Basic voice recording & transcription
2. Simple task creation
3. Basic search
4. User authentication
5. Simple dashboard

### 5.2 Phase 2 (2-3 months)
1. Advanced NLP processing
2. Multi-turn conversations
3. Rich media support
4. Mobile optimization
5. Basic analytics

### 5.3 Future Enhancements
1. Team collaboration
2. Advanced analytics
3. Third-party integrations
4. Mobile apps
5. Advanced AI features

## 6. Security & Compliance

- End-to-end encryption for voice data
- Secure API authentication (JWT)
- Rate limiting and DDoS protection
- Data retention policies
- Regular security audits
- GDPR/CCPA compliance

## 7. Performance Targets

- Voice-to-text latency: < 1s
- API response time: < 200ms
- Uptime: 99.9%
- Concurrent users: 10,000+
- Audio processing: < 2s for 30s recording

## 8. Monitoring & Analytics

### 8.1 Metrics
- API response times
- Error rates
- User engagement
- Voice recognition accuracy
- System resource usage

### 8.2 Logging
- Structured logging (JSON)
- Request/response logging
- Error tracking
- Performance metrics

## 9. Deployment

### 9.1 Environments
- Production
- Staging
- Development

### 9.2 CI/CD
- Automated testing
- Blue/green deployments
- Rollback procedures
- Database migrations

## 10. Testing Strategy

### 10.1 Unit Tests
- Core business logic
- Utility functions
- API endpoints

### 10.2 Integration Tests
- API flows
- Database operations
- Third-party services

### 10.3 E2E Tests
- User flows
- Voice interactions
- Cross-browser testing

## 11. Documentation

### 11.1 Technical Documentation
- API reference
- Architecture decisions
- Setup guides
- Deployment procedures

### 11.2 User Documentation
- Getting started
- Feature guides
- Troubleshooting
- FAQ
