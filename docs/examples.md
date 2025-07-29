# Examples

This page provides practical examples of using Teraclio for various template rendering scenarios.

## Basic Examples

### Simple Text Replacement

**Data** (`user.json`):
```json
{
  "name": "Alice",
  "role": "Developer",
  "team": "Backend"
}
```

**Template** (`greeting.txt`):
```jinja2
Hello {{ data.name }}!
You are a {{ data.role }} on the {{ data.team }} team.
```

**Command**:
```bash
teraclio --source user.json --template greeting.txt
```

**Output**:
```
Hello Alice!
You are a Developer on the Backend team.
```

### Working with Arrays

**Data** (`inventory.json`):
```json
{
  "store": "Tech Shop",
  "items": [
    {"name": "Laptop", "price": 999.99, "stock": 5},
    {"name": "Mouse", "price": 29.99, "stock": 15},
    {"name": "Keyboard", "price": 79.99, "stock": 8}
  ]
}
```

**Template** (`inventory-report.txt`):
```jinja2
{{ data.store }} Inventory Report
{{ "=" * (data.store | length + 17) }}

{% for item in data.items %}
{{ loop.index }}. {{ item.name }}
   Price: ${{ item.price }}
   Stock: {{ item.stock }} units
   Value: ${{ (item.price * item.stock) | round(precision=2) }}

{% endfor %}
Total Items: {{ data.items | length }}
Total Value: ${{ data.items | map(attribute="price") | sum | round(precision=2) }}
```

## Advanced Examples

### Configuration File Generation

**Data** (`config-data.json`):
```json
{
  "app": {
    "name": "MyApp",
    "version": "1.2.0",
    "port": 8080
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "myapp_db"
  },
  "features": {
    "auth": true,
    "logging": true,
    "metrics": false
  }
}
```

**Template** (`app.conf`):
```jinja2
# {{ data.app.name }} Configuration
# Generated on {{ now() | date(format="%Y-%m-%d %H:%M:%S") }}

[application]
name = {{ data.app.name }}
version = {{ data.app.version }}
port = {{ data.app.port }}

[database]
host = {{ data.database.host }}
port = {{ data.database.port }}
database = {{ data.database.name }}

[features]
{% for feature, enabled in data.features %}
{{ feature }} = {{ enabled | lower }}
{% endfor %}

# Environment-specific settings (use --env-vars flag)
debug = {{ data.env.DEBUG | default(value="false") }}
log_level = {{ data.env.LOG_LEVEL | default(value="info") }}
```

### HTML Report Generation

**Data** (`test-results.json`):
```json
{
  "project": "Web Application",
  "timestamp": "2024-01-15T10:30:00Z",
  "summary": {
    "total": 150,
    "passed": 142,
    "failed": 8,
    "skipped": 0
  },
  "failures": [
    {
      "test": "user_authentication_test",
      "error": "Connection timeout",
      "file": "tests/auth.py",
      "line": 45
    },
    {
      "test": "payment_processing_test", 
      "error": "Invalid API key",
      "file": "tests/payment.py",
      "line": 78
    }
  ]
}
```

**Template** (`test-report.html`):
```jinja2
<!DOCTYPE html>
<html>
<head>
    <title>{{ data.project }} - Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .summary { background: #f5f5f5; padding: 20px; margin: 20px 0; }
        .passed { color: green; }
        .failed { color: red; }
        .failure { background: #ffe6e6; padding: 10px; margin: 10px 0; border-left: 4px solid red; }
    </style>
</head>
<body>
    <h1>{{ data.project }} Test Report</h1>
    <p><strong>Generated:</strong> {{ data.timestamp | date(format="%B %d, %Y at %H:%M") }}</p>
    
    <div class="summary">
        <h2>Summary</h2>
        <p><strong>Total Tests:</strong> {{ data.summary.total }}</p>
        <p class="passed"><strong>Passed:</strong> {{ data.summary.passed }}</p>
        <p class="failed"><strong>Failed:</strong> {{ data.summary.failed }}</p>
        <p><strong>Success Rate:</strong> {{ ((data.summary.passed / data.summary.total) * 100) | round }}%</p>
    </div>

    {% if data.failures %}
    <h2>Failures</h2>
    {% for failure in data.failures %}
    <div class="failure">
        <h3>{{ failure.test }}</h3>
        <p><strong>Error:</strong> {{ failure.error }}</p>
        <p><strong>Location:</strong> {{ failure.file }}:{{ failure.line }}</p>
    </div>
    {% endfor %}
    {% endif %}
</body>
</html>
```

## Custom Filter Examples

### Hash & Security Filters

**Data** (`user-data.json`):
```json
{
  "users": [
    {
      "username": "alice",
      "email": "alice@example.com",
      "password": "secret123",
      "api_key": "myapikey123"
    },
    {
      "username": "bob", 
      "email": "bob@example.com",
      "password": "password456",
      "api_key": "bobsecret456"
    }
  ]
}
```

**Template** (`user-security.txt`):
```jinja2
User Security Report
===================

{% for user in data.users %}
User: {{ user.username }}
Email Hash (MD5): {{ user.email | md5 }}
Password Hash (SHA256): {{ user.password | sha256 }}
API Key (Base64): {{ user.api_key | base64_encode }}

{% endfor %}
```

### Web & URL Filters

**Data** (`web-data.json`):
```json
{
  "search_queries": [
    "hello world",
    "rust programming",
    "template engine"
  ],
  "user_content": "<script>alert('xss')</script>",
  "api_endpoints": [
    "/users/search?q=john doe",
    "/products/search?category=electronics & computers"
  ]
}
```

**Template** (`web-safe-output.html`):
```jinja2
<!DOCTYPE html>
<html>
<head><title>Search Results</title></head>
<body>
    <h1>Search Queries</h1>
    <ul>
    {% for query in data.search_queries %}
        <li><a href="/search?q={{ query | url_encode }}">{{ query }}</a></li>
    {% endfor %}
    </ul>
    
    <h2>User Content (Escaped)</h2>
    <p>{{ data.user_content | html_escape }}</p>
    
    <h2>API Endpoints</h2>
    {% for endpoint in data.api_endpoints %}
    <code>{{ endpoint | url_encode }}</code><br>
    {% endfor %}
</body>
</html>
```

### String Case Conversion

**Data** (`api-spec.json`):
```json
{
  "service_name": "UserAuthenticationService",
  "endpoints": [
    {"name": "getUserProfile", "method": "GET"},
    {"name": "updateUserSettings", "method": "POST"},
    {"name": "deleteUserAccount", "method": "DELETE"}
  ],
  "database_tables": ["user_profiles", "authentication_tokens"]
}
```

**Template** (`api-documentation.md`):
```jinja2
# {{ data.service_name | kebab_case }} API

Service: {{ data.service_name | pascal_case }}  
Database: {{ data.service_name | snake_case }}_db

## Endpoints

{% for endpoint in data.endpoints %}
### {{ endpoint.name | pascal_case }}
- **Method:** {{ endpoint.method }}
- **Function:** `{{ endpoint.name | camel_case }}`
- **Route:** `/{{ endpoint.name | kebab_case }}`
- **Handler:** `{{ endpoint.name | snake_case }}_handler`

{% endfor %}

## Database Tables
{% for table in data.database_tables %}
- {{ table | pascal_case }} (`{{ table }}`)
{% endfor %}
```

### Data Encoding & Processing

**Data** (`encoded-messages.json`):
```json
{
  "messages": [
    {
      "id": 1,
      "content": "Hello World!",
      "sender": "system"
    },
    {
      "id": 2,
      "encoded": "V2VsY29tZSB0byBUZXJhY2xpbyE=",
      "sender": "admin"
    }
  ]
}
```

**Template** (`message-processing.txt`):
```jinja2
Message Processing Report
========================

{% for msg in data.messages %}
Message {{ msg.id }} from {{ msg.sender }}:
{% if msg.content %}
  Content: {{ msg.content }}
  SHA256: {{ msg.content | sha256 }}
  Base64: {{ msg.content | base64_encode }}
  Bytes: {{ msg.content | str_to_bytes | length }} bytes
{% elif msg.encoded %}
  Encoded: {{ msg.encoded }}
  Decoded: {{ msg.encoded | base64_decode | bytes_to_str }}
  Hash: {{ msg.encoded | base64_decode | bytes_to_str | sha256 }}
{% endif %}

{% endfor %}
```

## Integration Examples

### CI/CD Pipeline Report

**Data** (`pipeline-status.json`):
```json
{
  "pipeline": {
    "id": "build-123",
    "branch": "main",
    "commit": "a1b2c3d",
    "status": "success"
  },
  "stages": [
    {"name": "build", "status": "success", "duration": 120},
    {"name": "test", "status": "success", "duration": 45},
    {"name": "deploy", "status": "success", "duration": 30}
  ],
  "metrics": {
    "total_duration": 195,
    "tests_run": 156,
    "coverage": 87.5
  }
}
```

**Template** (`pipeline-slack.md`):
```jinja2
## 🚀 Pipeline {{ data.pipeline.id }} - {{ data.pipeline.status | title }}

**Branch:** `{{ data.pipeline.branch }}`  
**Commit:** `{{ data.pipeline.commit }}`  
**Duration:** {{ data.metrics.total_duration }}s

### Stages
{% for stage in data.stages -%}
{% if stage.status == "success" -%}
✅ {{ stage.name | title }}: {{ stage.duration }}s
{% else -%}
❌ {{ stage.name | title }}: {{ stage.duration }}s
{% endif -%}
{% endfor %}

### Metrics
- **Tests:** {{ data.metrics.tests_run }} run
- **Coverage:** {{ data.metrics.coverage }}%
- **Total Time:** {{ data.metrics.total_duration }}s
```

### Environment-Specific Deployment

**Data** (`deployment.json`):
```json
{
  "service": "api-server",
  "version": "v2.1.0",
  "replicas": 3,
  "resources": {
    "cpu": "500m",
    "memory": "1Gi"
  },
  "config": {
    "database_url": "postgresql://localhost:5432/prod",
    "redis_url": "redis://localhost:6379"
  }
}
```

**Template** (`kubernetes-deployment.yaml`):
```jinja2
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ data.service }}
  labels:
    app: {{ data.service }}
    version: {{ data.version }}
spec:
  replicas: {{ data.replicas }}
  selector:
    matchLabels:
      app: {{ data.service }}
  template:
    metadata:
      labels:
        app: {{ data.service }}
        version: {{ data.version }}
    spec:
      containers:
      - name: {{ data.service }}
        image: {{ data.service }}:{{ data.version }}
        resources:
          requests:
            cpu: {{ data.resources.cpu }}
            memory: {{ data.resources.memory }}
          limits:
            cpu: {{ data.resources.cpu }}
            memory: {{ data.resources.memory }}
        env:
        {% for key, value in data.config %}
        - name: {{ key | upper }}
          value: "{{ value }}"
        {% endfor %}
        - name: ENVIRONMENT
          value: "{{ data.env.ENVIRONMENT | default(value="production") }}"
```

## Multi-Format & Environment Examples

### YAML Configuration Processing

**Data** (`app-config.yaml`):
```yaml
app:
  name: MyWebApp
  version: 2.1.0
database:
  host: localhost
  port: 5432
services:
  - name: auth_service
    port: 8001
  - name: user_service  
    port: 8002
```

**Template** (`docker-compose.yml`):
```jinja2
version: '3.8'
services:
  {{ data.app.name | kebab_case }}:
    image: {{ data.app.name | kebab_case }}:{{ data.app.version }}
    environment:
      - DATABASE_URL={{ data.env.DATABASE_URL | default(value="postgresql://localhost:5432/app") }}
      - API_KEY={{ data.env.API_KEY | sha256 }}
    ports:
      - "8080:8080"
      
{% for service in data.services %}
  {{ service.name | kebab_case }}:
    image: {{ service.name | kebab_case }}:latest
    ports:
      - "{{ service.port }}:{{ service.port }}"
{% endfor %}
```

**Command**:
```bash
teraclio -s app-config.yaml -t docker-compose.yml --env-vars
```

### TOML Configuration Example

**Data** (`build-config.toml`):
```toml
[package]
name = "my-rust-app"
version = "1.0.0"

[build]
target = "x86_64-unknown-linux-gnu"
features = ["ssl", "database"]

[[dependencies]]
name = "serde"
version = "1.0"

[[dependencies]]
name = "tokio"
version = "1.0"
```

**Template** (`Cargo.toml.template`):
```jinja2
[package]
name = "{{ data.package.name | kebab_case }}"
version = "{{ data.package.version }}"
edition = "2021"

# Build configuration
# Target: {{ data.build.target }}
# Features: {{ data.build.features | join(sep=", ") }}

[dependencies]
{% for dep in data.dependencies %}
{{ dep.name }} = "{{ dep.version }}"
{% endfor %}

# Environment-specific dependencies
{% if data.env.ENVIRONMENT == "development" %}
tokio-test = "0.4"
{% endif %}
```

**Command**:
```bash
teraclio -s build-config.toml -t Cargo.toml.template --env-vars -d Cargo.toml
```

### Advanced Filter Combinations

**Data** (`complex-data.json`):
```json
{
  "project": "WebApp2024",
  "users": [
    {"email": "admin@example.com", "role": "Administrator"},
    {"email": "user@example.com", "role": "StandardUser"}
  ],
  "api_endpoints": [
    "getUserProfile",
    "updateUserSettings", 
    "deleteAccount"
  ]
}
```

**Template** (`security-report.md`):
```jinja2
# {{ data.project | pascal_case }} Security Report

Generated on: {{ data.env.BUILD_DATE | default(value="unknown") }}
Environment: {{ data.env.ENVIRONMENT | default(value="development") | upper }}

## User Analysis

{% for user in data.users %}
### {{ user.role | pascal_case }}
- **Email Hash**: `{{ user.email | md5 }}`
- **Role**: {{ user.role | snake_case | upper }}
- **Safe Display**: {{ user.email | html_escape }}
- **URL Param**: `email={{ user.email | url_encode }}`

{% endfor %}

## API Endpoints

| Endpoint | Route | Handler | CSS Class |
|----------|-------|---------|-----------|
{% for endpoint in data.api_endpoints -%}
| {{ endpoint | pascal_case }} | `/{{ endpoint | kebab_case }}` | `{{ endpoint | snake_case }}_handler` | `{{ endpoint | kebab_case }}-btn` |
{% endfor %}

## Security Tokens

Project Hash: `{{ data.project | sha256 }}`  
Session Token: `{{ (data.project + ":" + data.env.BUILD_ID | default(value="dev")) | base64_encode }}`
```

**Command**:
```bash
BUILD_DATE=$(date -Iseconds) BUILD_ID=12345 ENVIRONMENT=production \
teraclio -s complex-data.json -t security-report.md --env-vars
```

## Tips and Best Practices

1. **Use meaningful variable names** in your data files
2. **Leverage auto-detection** - let Teraclio detect JSON/YAML/TOML by file extension
3. **Combine filters** for powerful transformations: `{{ name | snake_case | upper }}`
4. **Use environment variables** with `--env-vars` for dynamic configuration
5. **Provide defaults** for optional fields: `{{ data.optional | default(value="N/A") }}`
6. **Validate your data** before running Teraclio  
7. **Test templates** with sample data during development
8. **Use security filters** (hash, escape) for sensitive data handling
9. **Chain case conversions** for consistent naming across systems
10. **Document complex template logic** with comments