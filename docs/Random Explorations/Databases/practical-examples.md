# Database Practical Examples

This document provides hands-on examples and code snippets for various database operations and scenarios.

## SQL Examples

### Basic CRUD Operations

#### Create Table with Constraints

```sql
-- Create a comprehensive employees table
CREATE TABLE employees (
    id INT PRIMARY KEY AUTO_INCREMENT,
    employee_id VARCHAR(10) UNIQUE NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(20),
    hire_date DATE NOT NULL,
    salary DECIMAL(10,2) CHECK (salary > 0),
    department_id INT,
    manager_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (department_id) REFERENCES departments(id),
    FOREIGN KEY (manager_id) REFERENCES employees(id)
);

-- Create departments table
CREATE TABLE departments (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    location VARCHAR(100),
    budget DECIMAL(12,2),
    manager_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### Advanced Queries

```sql
-- Complex join with aggregation
SELECT
    d.name AS department_name,
    COUNT(e.id) AS employee_count,
    AVG(e.salary) AS average_salary,
    MAX(e.salary) AS highest_salary,
    MIN(e.salary) AS lowest_salary
FROM departments d
LEFT JOIN employees e ON d.id = e.department_id
WHERE d.budget > 100000
GROUP BY d.id, d.name
HAVING COUNT(e.id) > 5
ORDER BY average_salary DESC;

-- Window functions for ranking
SELECT
    first_name,
    last_name,
    salary,
    department_id,
    ROW_NUMBER() OVER (PARTITION BY department_id ORDER BY salary DESC) AS dept_rank,
    RANK() OVER (ORDER BY salary DESC) AS overall_rank,
    DENSE_RANK() OVER (ORDER BY salary DESC) AS dense_rank,
    LAG(salary, 1) OVER (ORDER BY salary DESC) AS prev_salary,
    LEAD(salary, 1) OVER (ORDER BY salary DESC) AS next_salary
FROM employees
ORDER BY salary DESC;

-- Recursive CTE for hierarchical data
WITH RECURSIVE employee_hierarchy AS (
    -- Base case: top-level managers
    SELECT id, first_name, last_name, manager_id, 0 AS level
    FROM employees
    WHERE manager_id IS NULL

    UNION ALL

    -- Recursive case: subordinates
    SELECT e.id, e.first_name, e.last_name, e.manager_id, eh.level + 1
    FROM employees e
    JOIN employee_hierarchy eh ON e.manager_id = eh.id
)
SELECT
    CONCAT(REPEAT('  ', level), first_name, ' ', last_name) AS hierarchy,
    level
FROM employee_hierarchy
ORDER BY level, first_name;
```

### Indexing Strategies

```sql
-- Create composite index for common query patterns
CREATE INDEX idx_employee_dept_salary ON employees(department_id, salary DESC);

-- Create covering index to avoid table lookups
CREATE INDEX idx_employee_covering ON employees(department_id, salary, first_name, last_name);

-- Create partial index for specific conditions
CREATE INDEX idx_high_salary_employees ON employees(salary) WHERE salary > 100000;

-- Create functional index
CREATE INDEX idx_employee_email_domain ON employees(SUBSTRING(email, LOCATE('@', email) + 1));

-- Analyze index usage
EXPLAIN FORMAT=JSON
SELECT first_name, last_name, salary
FROM employees
WHERE department_id = 1 AND salary > 50000;
```

## NoSQL Examples

### MongoDB Operations

#### Document Structure and Queries

```javascript
// Insert documents with embedded data
db.employees.insertMany([
  {
    _id: ObjectId(),
    employee_id: "EMP001",
    personal_info: {
      first_name: "John",
      last_name: "Doe",
      email: "john.doe@company.com",
      phone: "+1-555-0123",
    },
    employment: {
      hire_date: ISODate("2023-01-15"),
      salary: 75000,
      department: "Engineering",
      position: "Software Engineer",
    },
    skills: ["JavaScript", "Python", "React", "Node.js"],
    projects: [
      {
        name: "E-commerce Platform",
        role: "Lead Developer",
        start_date: ISODate("2023-02-01"),
        end_date: ISODate("2023-08-31"),
      },
    ],
    performance: {
      rating: 4.5,
      goals_met: 8,
      goals_total: 10,
    },
  },
]);

// Complex aggregation pipeline
db.employees.aggregate([
  // Match employees in Engineering department
  { $match: { "employment.department": "Engineering" } },

  // Unwind skills array to analyze individual skills
  { $unwind: "$skills" },

  // Group by skill and calculate statistics
  {
    $group: {
      _id: "$skills",
      count: { $sum: 1 },
      avg_salary: { $avg: "$employment.salary" },
      max_salary: { $max: "$employment.salary" },
      min_salary: { $min: "$employment.salary" },
    },
  },

  // Sort by count descending
  { $sort: { count: -1 } },

  // Limit to top 10 skills
  { $limit: 10 },
]);

// Text search with scoring
db.employees
  .find(
    { $text: { $search: "JavaScript React Node.js" } },
    {
      score: { $meta: "textScore" },
      "personal_info.first_name": 1,
      "personal_info.last_name": 1,
      "employment.position": 1,
      skills: 1,
    }
  )
  .sort({ score: { $meta: "textScore" } });

// Geospatial queries (if location data exists)
db.employees.find({
  location: {
    $near: {
      $geometry: {
        type: "Point",
        coordinates: [-74.0059, 40.7128], // New York coordinates
      },
      $maxDistance: 10000, // 10km radius
    },
  },
});
```

### Redis Operations

#### Caching Patterns

```python
import redis
import json
import time

# Connect to Redis
r = redis.Redis(host='localhost', port=6379, db=0)

# Cache-aside pattern
def get_employee(employee_id):
    # Try to get from cache first
    cached_data = r.get(f"employee:{employee_id}")
    if cached_data:
        return json.loads(cached_data)

    # If not in cache, get from database
    employee_data = database.get_employee(employee_id)

    # Store in cache with expiration
    r.setex(f"employee:{employee_id}", 3600, json.dumps(employee_data))

    return employee_data

# Write-through pattern
def update_employee(employee_id, data):
    # Update database
    database.update_employee(employee_id, data)

    # Update cache
    r.setex(f"employee:{employee_id}", 3600, json.dumps(data))

# Cache invalidation
def delete_employee(employee_id):
    # Delete from database
    database.delete_employee(employee_id)

    # Remove from cache
    r.delete(f"employee:{employee_id}")

# Session storage
def create_user_session(user_id, session_data):
    session_id = f"session:{user_id}:{int(time.time())}"
    r.setex(session_id, 1800, json.dumps(session_data))  # 30 minutes
    return session_id

# Rate limiting
def check_rate_limit(user_id, limit=100, window=3600):
    key = f"rate_limit:{user_id}"
    current = r.incr(key)

    if current == 1:
        r.expire(key, window)

    return current <= limit
```

## Database Design Patterns

### Repository Pattern Implementation

#### Python Example

```python
from abc import ABC, abstractmethod
from typing import List, Optional, Dict, Any
import sqlite3

class EmployeeRepository(ABC):
    @abstractmethod
    def find_by_id(self, employee_id: int) -> Optional[Dict[str, Any]]:
        pass

    @abstractmethod
    def find_by_department(self, department_id: int) -> List[Dict[str, Any]]:
        pass

    @abstractmethod
    def save(self, employee: Dict[str, Any]) -> int:
        pass

    @abstractmethod
    def update(self, employee_id: int, employee: Dict[str, Any]) -> bool:
        pass

    @abstractmethod
    def delete(self, employee_id: int) -> bool:
        pass

class SQLiteEmployeeRepository(EmployeeRepository):
    def __init__(self, db_path: str):
        self.db_path = db_path

    def _get_connection(self):
        return sqlite3.connect(self.db_path)

    def find_by_id(self, employee_id: int) -> Optional[Dict[str, Any]]:
        with self._get_connection() as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.cursor()
            cursor.execute(
                "SELECT * FROM employees WHERE id = ?",
                (employee_id,)
            )
            row = cursor.fetchone()
            return dict(row) if row else None

    def find_by_department(self, department_id: int) -> List[Dict[str, Any]]:
        with self._get_connection() as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.cursor()
            cursor.execute(
                "SELECT * FROM employees WHERE department_id = ?",
                (department_id,)
            )
            rows = cursor.fetchall()
            return [dict(row) for row in rows]

    def save(self, employee: Dict[str, Any]) -> int:
        with self._get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute("""
                INSERT INTO employees (first_name, last_name, email, department_id, salary)
                VALUES (?, ?, ?, ?, ?)
            """, (
                employee['first_name'],
                employee['last_name'],
                employee['email'],
                employee['department_id'],
                employee['salary']
            ))
            return cursor.lastrowid

    def update(self, employee_id: int, employee: Dict[str, Any]) -> bool:
        with self._get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute("""
                UPDATE employees
                SET first_name = ?, last_name = ?, email = ?,
                    department_id = ?, salary = ?
                WHERE id = ?
            """, (
                employee['first_name'],
                employee['last_name'],
                employee['email'],
                employee['department_id'],
                employee['salary'],
                employee_id
            ))
            return cursor.rowcount > 0

    def delete(self, employee_id: int) -> bool:
        with self._get_connection() as conn:
            cursor = conn.cursor()
            cursor.execute("DELETE FROM employees WHERE id = ?", (employee_id,))
            return cursor.rowcount > 0

# Usage example
repo = SQLiteEmployeeRepository("company.db")
employee = repo.find_by_id(1)
employees = repo.find_by_department(1)
```

### Unit of Work Pattern

```python
class UnitOfWork:
    def __init__(self, repository: EmployeeRepository):
        self.repository = repository
        self.new_objects = []
        self.dirty_objects = []
        self.removed_objects = []

    def register_new(self, employee: Dict[str, Any]):
        self.new_objects.append(employee)

    def register_dirty(self, employee: Dict[str, Any]):
        if employee not in self.dirty_objects:
            self.dirty_objects.append(employee)

    def register_removed(self, employee_id: int):
        self.removed_objects.append(employee_id)

    def commit(self):
        try:
            # Insert new objects
            for employee in self.new_objects:
                self.repository.save(employee)

            # Update dirty objects
            for employee in self.dirty_objects:
                self.repository.update(employee['id'], employee)

            # Delete removed objects
            for employee_id in self.removed_objects:
                self.repository.delete(employee_id)

            # Clear tracking
            self.new_objects.clear()
            self.dirty_objects.clear()
            self.removed_objects.clear()

        except Exception as e:
            # Rollback logic would go here
            raise e
```

## Database Migration Examples

### SQL Migration Scripts

```sql
-- Migration: Add employee skills table
-- Version: 1.1.0
-- Date: 2024-01-15

-- Create skills table
CREATE TABLE skills (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL UNIQUE,
    category VARCHAR(50),
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create employee_skills junction table
CREATE TABLE employee_skills (
    employee_id INT NOT NULL,
    skill_id INT NOT NULL,
    proficiency_level ENUM('Beginner', 'Intermediate', 'Advanced', 'Expert') DEFAULT 'Beginner',
    years_experience INT DEFAULT 0,
    certified BOOLEAN DEFAULT FALSE,
    certified_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (employee_id, skill_id),
    FOREIGN KEY (employee_id) REFERENCES employees(id) ON DELETE CASCADE,
    FOREIGN KEY (skill_id) REFERENCES skills(id) ON DELETE CASCADE
);

-- Insert initial skills data
INSERT INTO skills (name, category, description) VALUES
('JavaScript', 'Programming', 'High-level programming language'),
('Python', 'Programming', 'General-purpose programming language'),
('React', 'Frontend', 'JavaScript library for building user interfaces'),
('Node.js', 'Backend', 'JavaScript runtime for server-side development'),
('SQL', 'Database', 'Structured Query Language for database management'),
('MongoDB', 'Database', 'NoSQL document database'),
('Docker', 'DevOps', 'Containerization platform'),
('AWS', 'Cloud', 'Amazon Web Services cloud platform');

-- Add indexes for performance
CREATE INDEX idx_employee_skills_employee ON employee_skills(employee_id);
CREATE INDEX idx_employee_skills_skill ON employee_skills(skill_id);
CREATE INDEX idx_employee_skills_proficiency ON employee_skills(proficiency_level);
```

### Database Schema Versioning

```python
class DatabaseMigrator:
    def __init__(self, connection):
        self.connection = connection
        self.migrations = []

    def add_migration(self, version, description, up_sql, down_sql):
        self.migrations.append({
            'version': version,
            'description': description,
            'up_sql': up_sql,
            'down_sql': down_sql
        })

    def get_current_version(self):
        cursor = self.connection.cursor()
        cursor.execute("SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1")
        result = cursor.fetchone()
        return result[0] if result else 0

    def migrate_up(self, target_version=None):
        current_version = self.get_current_version()
        migrations_to_run = [
            m for m in self.migrations
            if m['version'] > current_version and
            (target_version is None or m['version'] <= target_version)
        ]

        for migration in sorted(migrations_to_run, key=lambda x: x['version']):
            print(f"Running migration {migration['version']}: {migration['description']}")
            cursor = self.connection.cursor()
            cursor.execute(migration['up_sql'])
            cursor.execute(
                "INSERT INTO schema_migrations (version, description) VALUES (?, ?)",
                (migration['version'], migration['description'])
            )
            self.connection.commit()

    def migrate_down(self, target_version):
        current_version = self.get_current_version()
        migrations_to_rollback = [
            m for m in self.migrations
            if m['version'] > target_version and m['version'] <= current_version
        ]

        for migration in sorted(migrations_to_rollback, key=lambda x: x['version'], reverse=True):
            print(f"Rolling back migration {migration['version']}: {migration['description']}")
            cursor = self.connection.cursor()
            cursor.execute(migration['down_sql'])
            cursor.execute(
                "DELETE FROM schema_migrations WHERE version = ?",
                (migration['version'],)
            )
            self.connection.commit()

# Usage
migrator = DatabaseMigrator(connection)

# Add migrations
migrator.add_migration(
    version=1,
    description="Create employees table",
    up_sql="CREATE TABLE employees (id INT PRIMARY KEY, name VARCHAR(100))",
    down_sql="DROP TABLE employees"
)

migrator.add_migration(
    version=2,
    description="Add email column to employees",
    up_sql="ALTER TABLE employees ADD COLUMN email VARCHAR(100)",
    down_sql="ALTER TABLE employees DROP COLUMN email"
)

# Run migrations
migrator.migrate_up()
```

## Performance Optimization Examples

### Query Optimization

```sql
-- Before: Inefficient query
SELECT e.first_name, e.last_name, d.name as department_name
FROM employees e
JOIN departments d ON e.department_id = d.id
WHERE e.salary > 50000
ORDER BY e.salary DESC;

-- After: Optimized with proper indexing
-- Create covering index
CREATE INDEX idx_employee_salary_dept ON employees(salary DESC, department_id, first_name, last_name);

-- Optimized query
SELECT e.first_name, e.last_name, d.name as department_name
FROM employees e
JOIN departments d ON e.department_id = d.id
WHERE e.salary > 50000
ORDER BY e.salary DESC
LIMIT 100;

-- Use EXPLAIN to analyze query plan
EXPLAIN FORMAT=JSON
SELECT e.first_name, e.last_name, d.name as department_name
FROM employees e
JOIN departments d ON e.department_id = d.id
WHERE e.salary > 50000
ORDER BY e.salary DESC;
```

### Connection Pooling

```python
import psycopg2
from psycopg2 import pool
import threading
import time

class DatabaseConnectionPool:
    def __init__(self, min_connections=5, max_connections=20, **kwargs):
        self.connection_pool = psycopg2.pool.ThreadedConnectionPool(
            min_connections, max_connections, **kwargs
        )
        self.lock = threading.Lock()

    def get_connection(self):
        return self.connection_pool.getconn()

    def return_connection(self, connection):
        self.connection_pool.putconn(connection)

    def close_all(self):
        self.connection_pool.closeall()

# Usage with context manager
class DatabaseContext:
    def __init__(self, pool):
        self.pool = pool
        self.connection = None

    def __enter__(self):
        self.connection = self.pool.get_connection()
        return self.connection

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.connection:
            self.pool.return_connection(self.connection)

# Example usage
pool = DatabaseConnectionPool(
    min_connections=5,
    max_connections=20,
    host='localhost',
    database='company',
    user='admin',
    password='password'
)

def execute_query(query, params=None):
    with DatabaseContext(pool) as conn:
        cursor = conn.cursor()
        cursor.execute(query, params)
        return cursor.fetchall()
```

## Database Security Examples

### SQL Injection Prevention

```python
import sqlite3
import hashlib
import secrets

class SecureDatabase:
    def __init__(self, db_path):
        self.connection = sqlite3.connect(db_path)
        self.connection.row_factory = sqlite3.Row

    def authenticate_user(self, username, password):
        # Use parameterized queries to prevent SQL injection
        cursor = self.connection.cursor()
        cursor.execute(
            "SELECT id, username, password_hash, salt FROM users WHERE username = ?",
            (username,)
        )
        user = cursor.fetchone()

        if user:
            # Verify password with salt
            password_hash = self._hash_password(password, user['salt'])
            if password_hash == user['password_hash']:
                return user
        return None

    def create_user(self, username, password):
        # Generate random salt
        salt = secrets.token_hex(16)
        password_hash = self._hash_password(password, salt)

        cursor = self.connection.cursor()
        cursor.execute(
            "INSERT INTO users (username, password_hash, salt) VALUES (?, ?, ?)",
            (username, password_hash, salt)
        )
        self.connection.commit()
        return cursor.lastrowid

    def _hash_password(self, password, salt):
        # Use PBKDF2 for password hashing
        return hashlib.pbkdf2_hmac('sha256', password.encode(), salt.encode(), 100000).hex()

    def search_employees_safe(self, search_term):
        # Safe search with parameterized query
        cursor = self.connection.cursor()
        cursor.execute(
            "SELECT * FROM employees WHERE first_name LIKE ? OR last_name LIKE ?",
            (f"%{search_term}%", f"%{search_term}%")
        )
        return cursor.fetchall()

# Example of what NOT to do (vulnerable to SQL injection)
def search_employees_unsafe(connection, search_term):
    cursor = connection.cursor()
    # This is vulnerable to SQL injection!
    query = f"SELECT * FROM employees WHERE first_name LIKE '%{search_term}%'"
    cursor.execute(query)
    return cursor.fetchall()
```

### Data Encryption

```python
from cryptography.fernet import Fernet
import base64
import os

class DatabaseEncryption:
    def __init__(self):
        # Generate or load encryption key
        self.key = self._get_or_create_key()
        self.cipher = Fernet(self.key)

    def _get_or_create_key(self):
        key_file = 'encryption.key'
        if os.path.exists(key_file):
            with open(key_file, 'rb') as f:
                return f.read()
        else:
            key = Fernet.generate_key()
            with open(key_file, 'wb') as f:
                f.write(key)
            return key

    def encrypt_sensitive_data(self, data):
        """Encrypt sensitive data before storing"""
        if isinstance(data, str):
            data = data.encode()
        return self.cipher.encrypt(data)

    def decrypt_sensitive_data(self, encrypted_data):
        """Decrypt sensitive data when retrieving"""
        decrypted = self.cipher.decrypt(encrypted_data)
        return decrypted.decode()

    def store_encrypted_employee(self, connection, employee_data):
        """Store employee with encrypted sensitive fields"""
        cursor = connection.cursor()

        # Encrypt sensitive fields
        encrypted_ssn = self.encrypt_sensitive_data(employee_data['ssn'])
        encrypted_salary = self.encrypt_sensitive_data(str(employee_data['salary']))

        cursor.execute("""
            INSERT INTO employees (first_name, last_name, email, encrypted_ssn, encrypted_salary)
            VALUES (?, ?, ?, ?, ?)
        """, (
            employee_data['first_name'],
            employee_data['last_name'],
            employee_data['email'],
            encrypted_ssn,
            encrypted_salary
        ))
        connection.commit()

    def retrieve_encrypted_employee(self, connection, employee_id):
        """Retrieve and decrypt employee data"""
        cursor = connection.cursor()
        cursor.execute(
            "SELECT * FROM employees WHERE id = ?",
            (employee_id,)
        )
        row = cursor.fetchone()

        if row:
            # Decrypt sensitive fields
            decrypted_ssn = self.decrypt_sensitive_data(row['encrypted_ssn'])
            decrypted_salary = float(self.decrypt_sensitive_data(row['encrypted_salary']))

            return {
                'id': row['id'],
                'first_name': row['first_name'],
                'last_name': row['last_name'],
                'email': row['email'],
                'ssn': decrypted_ssn,
                'salary': decrypted_salary
            }
        return None
```

These examples demonstrate practical database operations, design patterns, and security measures that are commonly used in real-world applications.
