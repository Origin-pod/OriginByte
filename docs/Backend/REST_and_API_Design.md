# REST and API Design Guide

## Table of Contents

- [What is REST?](#what-is-rest)
- [Core REST Principles](#core-rest-principles)
- [REST API Design Best Practices](#rest-api-design-best-practices)
- [Backend System Design Roadmap](#backend-system-design-roadmap)
- [Learning Resources](#learning-resources)

## What is REST?

REST (Representational State Transfer) is an architectural style for distributed hypermedia systems, defined by Roy Fielding in 2000. It's not a protocol but a set of constraints for designing networked applications.

## Core REST Principles

### 1. Client-Server Architecture

- **Separation of concerns**: Client handles UI, server handles data/storage
- **Independence**: Client and server can evolve independently
- **Scalability**: Server can be scaled without affecting clients

### 2. Statelessness

- **No session state**: Every request must contain all information needed
- **Self-contained requests**: Server doesn't store client context
- **Benefits**: Simplified scaling, reliability, and visibility

### 3. Cacheability

- **Explicit cache control**: Responses define cache policies
- **Performance**: Reduces server load and latency
- **Cache headers**: `Cache-Control`, `ETag`, `Last-Modified`

### 4. Uniform Interface

- **Resource identification**: URIs identify resources
- **Multiple representations**: JSON, XML, etc.
- **Self-descriptive messages**: Includes metadata in HTTP headers
- **HATEOAS**: Responses contain links to possible next actions

### 5. Layered System

- **Hierarchical architecture**: Load balancers, proxies, gateways
- **Transparency**: Client doesn't know about intermediaries
- **Scalability**: Each layer can be scaled independently

### 6. Code on Demand (Optional)

- **Server executable code**: JavaScript, applets
- **Rarely used** in modern systems

## REST API Design Best Practices

### Resource Naming Conventions

```http
# Good: Use nouns, not verbs
GET /users          # Get all users
GET /users/123      # Get specific user
GET /users/123/orders  # Get user's orders

# Bad: Avoid verbs
GET /getAllUsers
GET /getUserById/123
```

### HTTP Methods

| Method | Description                      |
| ------ | -------------------------------- |
| GET    | Retrieve resource representation |
| POST   | Create new resource              |
| PUT    | Replace resource entirely        |
| PATCH  | Partial resource update          |
| DELETE | Remove resource                  |

### Status Codes

#### Success (2xx)

- `200 OK` - Request succeeded
- `201 Created` - Resource created
- `204 No Content` - Success, no response body

#### Redirection (3xx)

- `301 Moved Permanently`
- `304 Not Modified`

#### Client Errors (4xx)

- `400 Bad Request`
- `401 Unauthorized`
- `403 Forbidden`
- `404 Not Found`
- `409 Conflict`
- `422 Unprocessable Entity`

#### Server Errors (5xx)

- `500 Internal Server Error`
- `502 Bad Gateway`
- `503 Service Unavailable`

### URL Design Patterns

#### Collections

```http
GET    /api/v1/products
POST   /api/v1/products
```

#### Specific Resources

```http
GET    /api/v1/products/123
PUT    /api/v1/products/123
DELETE /api/v1/products/123
```

#### Nested Resources

```http
GET    /api/v1/users/456/orders
POST   /api/v1/users/456/orders
GET    /api/v1/users/456/orders/789
```

#### Query Parameters

```http
# Filtering and sorting
GET /api/v1/products?category=electronics&price_min=100&sort=price_asc

# Pagination
GET /api/v1/products?page=2&limit=20&offset=20

# Field selection
GET /api/v1/products?fields=id,name,price
```

### Request/Response Examples

#### Request Body

```json
// POST /api/v1/users
{
  "name": "John Doe",
  "email": "john@example.com",
  "age": 30
}
```

#### Response Structure

```json
{
  "data": {
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com"
  },
  "meta": {
    "timestamp": "2024-01-15T10:30:00Z",
    "version": "v1"
  }
}
```

#### Collection Response

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "hasNext": true
  }
}
```

#### Error Response

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": {
      "field": "email",
      "value": "invalid-email"
    }
  }
}
```

### Security Best Practices

#### Authentication Methods

- **JWT (JSON Web Tokens)**: Stateless authentication
- **OAuth 2.0**: Third-party authorization
- **API Keys**: Simple authentication for public APIs
- **Basic Auth**: Simple username/password (HTTPS required)

#### Security Headers

```http
Content-Security-Policy: default-src 'self'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Strict-Transport-Security: max-age=31536000
```

### API Versioning

#### Path-based

```
/api/v1/users
/api/v2/users
```

#### Header-based

```http
GET /users
Accept: application/vnd.api+json;version=1
```

#### Query Parameter

```
GET /users?version=v1
```

## Backend System Design Roadmap

### Phase 1: Fundamentals

- [x] REST API Design
- [x] HTTP Protocol (HTTP/1.1, HTTP/2, HTTP/3)
- [x] Data Formats (JSON, XML, Protocol Buffers, MessagePack)

### Phase 2: Core Backend Concepts

1. **Database Design**

   - Relational (PostgreSQL, MySQL)
   - NoSQL (MongoDB, Redis, Cassandra)
   - Indexing and query optimization
   - Transactions and ACID properties

2. **Authentication & Authorization**

   - JWT, OAuth 2.0, OpenID Connect
   - Session management
   - Role-based access control (RBAC)

3. **Caching Strategies**

   - In-memory caching (Redis, Memcached)
   - CDN caching
   - Cache invalidation patterns
   - Cache-aside, write-through, write-behind

4. **Message Queues**
   - Asynchronous processing
   - RabbitMQ, Apache Kafka, AWS SQS
   - Publisher-subscriber patterns
   - Event-driven architecture

### Phase 3: Scalability & Performance

1. **Load Balancing**

   - Horizontal vs vertical scaling
   - Load balancing algorithms
   - Health checks and failover

2. **Microservices Architecture**

   - Service decomposition
   - Inter-service communication
   - Service discovery
   - API gateways

3. **Database Scaling**

   - Read replicas
   - Sharding
   - Connection pooling
   - Database clustering

4. **Performance Optimization**
   - Query optimization
   - Connection management
   - Resource pooling
   - Async programming

### Phase 4: Advanced Topics

1. **System Reliability**

   - Circuit breakers
   - Retry patterns
   - Rate limiting
   - Backpressure

2. **Monitoring & Observability**

   - Logging, metrics, tracing
   - Application performance monitoring (APM)
   - Distributed tracing
   - Alerting systems

3. **Security**

   - API security (OWASP API Security)
   - DDoS protection
   - Input validation and sanitization
   - Encryption (TLS/SSL)

4. **DevOps & Deployment**
   - Containerization (Docker, Kubernetes)
   - CI/CD pipelines
   - Infrastructure as Code
   - Cloud platforms (AWS, GCP, Azure)

### Phase 5: Specialized Topics

1. GraphQL vs REST
2. WebSocket APIs
3. Serverless Architecture
4. Edge Computing
5. API Management Platforms

## Learning Resources

### Books

- "Designing Data-Intensive Applications" by Martin Kleppmann
- "Building Microservices" by Sam Newman
- "RESTful Web APIs" by Leonard Richardson
- "System Design Interview" by Alex Xu

### Practical Projects

1. Build a REST API with proper error handling and validation
2. Implement caching with Redis
3. Create a microservices-based application
4. Design a system with message queues
5. Build a system with real-time notifications

### Next Steps

Start with database design and authentication patterns as they're fundamental to most backend systems. Then move on to caching and message queues for understanding how to build scalable systems.
