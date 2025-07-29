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

# Environment-specific settings
debug = {{ get_env(name="DEBUG", default="false") }}
log_level = {{ get_env(name="LOG_LEVEL", default="info") }}
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

### Data Encoding/Decoding

**Data** (`encoded-data.json`):
```json
{
  "messages": [
    {
      "id": 1,
      "encoded": "SGVsbG8gV29ybGQh",
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

**Template** (`decoded-messages.txt`):
```jinja2
Decoded Messages
================

{% for msg in data.messages %}
Message {{ msg.id }} from {{ msg.sender }}:
Encoded: {{ msg.encoded }}
Decoded: {{ msg.encoded | base64_decode | bytes_to_str }}

{% endfor %}
```

### Binary Data Processing

**Data** (`binary-data.json`):
```json
{
  "filename": "document.pdf",
  "content": "Hello Binary World",
  "metadata": {
    "type": "text",
    "encoding": "utf-8"
  }
}
```

**Template** (`binary-analysis.txt`):
```jinja2
File Analysis: {{ data.filename }}
{{ "=" * (data.filename | length + 15) }}

Content: {{ data.content }}
Byte representation: {{ data.content | str_to_bytes }}
Byte count: {{ data.content | str_to_bytes | length }}

Encoded versions:
- Base64: {{ data.content | base64_encode }}
- Hex representation: {% for byte in data.content | str_to_bytes %}{{ "%02x" | format(byte) }}{% endfor %}

Round-trip test:
Original: {{ data.content }}
Processed: {{ data.content | str_to_bytes | bytes_to_str }}
Match: {{ data.content == (data.content | str_to_bytes | bytes_to_str) }}
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
          value: "{{ get_env(name="ENVIRONMENT", default="production") }}"
```

## Tips and Best Practices

1. **Use meaningful variable names** in your JSON data
2. **Provide defaults** for optional fields: `{{ data.optional | default(value="N/A") }}`
3. **Validate your JSON** before running Teraclio
4. **Test templates** with sample data during development
5. **Use comments** in templates to document complex logic
6. **Break down large templates** into smaller, reusable components when possible