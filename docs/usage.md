# Usage Guide

## Command Line Interface

Teraclio provides a simple command-line interface for template rendering:

```bash
teraclio --source <data-file> --template <template-file> [OPTIONS]
```

### Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--source` | `-s` | ✅ Yes | Path to data file (JSON, YAML, TOML) |
| `--template` | `-t` | ✅ Yes | Path to Tera template file |
| `--dest` | `-d` | ❌ No | Output file path (stdout if omitted) |
| `--format` | `-f` | ❌ No | Input format (json, yaml, toml) - auto-detected from extension |
| `--env-vars` | - | ❌ No | Include environment variables as `data.env` object |

### Examples

#### Basic Usage
```bash
# Output to stdout
teraclio --source data.json --template template.txt

# Output to file
teraclio --source data.json --template template.txt --dest output.txt
```

#### Multi-Format Input Support
```bash
# JSON (auto-detected)
teraclio -s config.json -t template.txt

# YAML (auto-detected)
teraclio -s config.yaml -t template.txt

# TOML (auto-detected)  
teraclio -s config.toml -t template.txt

# Explicit format specification
teraclio -s data.txt -t template.txt --format yaml
```

#### Environment Variables Integration
```bash
# Include environment variables as data.env
teraclio -s data.json -t template.txt --env-vars

# Template can access: {{ data.env.HOME }}, {{ data.env.USER }}, etc.
```

#### Using Short Arguments
```bash
# Equivalent combinations
teraclio -s data.json -t template.txt -d output.txt
teraclio -s config.yaml -t template.txt -f yaml --env-vars
```

## Real-World Example: SonarQube Report

Here's a practical example generating a report from SonarQube API data:

### 1. Fetch Data
```bash
curl -H "Authorization: Bearer $SONARQUBE_TOKEN" \
  "$SONARQUBE_URL/api/issues/search?componentKeys=$PROJECT_KEY&resolved=no" \
  | jq '.issues[] | {key, message, severity, component, textRange}' \
  | jq --arg REPO_NAME "$REPO_NAME" --arg PROJECT_KEY "$PROJECT_KEY" \
    -s '{issues: ., repo: $REPO_NAME, project: $PROJECT_KEY}' > report.json
```

### 2. Create Template
Create `template.txt`:
```jinja2
Issues Report for {{ data.repo }}
Project: {{ data.project }}
Generated: {{ now() | date(format="%Y-%m-%d %H:%M") }}

{% for issue in data.issues %}
{{ loop.index }}. {{ issue.component }}
   Severity: {{ issue.severity }}
   Message: {{ issue.message }}
   
{% endfor %}

Total Issues: {{ data.issues | length }}
```

### 3. Generate Report
```bash
teraclio --source report.json --template template.txt --dest issues-report.txt
```

## Data Access

All input data (JSON, YAML, TOML) is accessible through the `data` root element:

### Input Examples

#### JSON Format
```json
{
  "user": {
    "name": "John Doe",
    "email": "john@example.com"
  },
  "items": ["apple", "banana", "cherry"],
  "count": 3
}
```

#### YAML Format  
```yaml
user:
  name: John Doe
  email: john@example.com
items:
  - apple
  - banana
  - cherry
count: 3
```

#### TOML Format
```toml
[user]
name = "John Doe"
email = "john@example.com"

items = ["apple", "banana", "cherry"]
count = 3
```

### Template Access
```jinja2
User: {{ data.user.name }} ({{ data.user.email }})
Items: {{ data.items | join(", ") }}
Count: {{ data.count }}

# Environment variables (when --env-vars is used)
Home: {{ data.env.HOME }}
User: {{ data.env.USER }}
```

## Error Handling

Teraclio provides clear error messages for common issues:

- **File not found**: `Data source file does not exist: /path/to/file.json`
- **Empty file**: `Data source file is empty`
- **Invalid format**: `YAML parsing error: expected value at line 1 column 1`  
- **Template error**: `Template error: Variable 'missing_var' not found`
- **Invalid template**: `Template file does not exist: /path/to/template.txt`
- **Unsupported format**: `Unsupported format: xml (supported: json, yaml, toml)`

## Advanced Usage Patterns

### Configuration File Processing
```bash
# Process different config formats
teraclio -s app.yaml -t k8s-deployment.yaml --env-vars
teraclio -s database.toml -t sql-migration.sql 
teraclio -s api.json -t openapi-spec.yaml
```

### Pipeline Integration
```bash
# In CI/CD pipelines
curl -s $API_ENDPOINT | jq '.' > data.json
teraclio -s data.json -t report.html --env-vars > report.html

# Docker builds
teraclio -s build-config.yaml -t Dockerfile.template > Dockerfile
```

### Environment-Specific Templating
```bash
# Development environment
teraclio -s config.dev.yaml -t app-config.yaml --env-vars

# Production environment  
teraclio -s config.prod.yaml -t app-config.yaml --env-vars
```

## Exit Codes

- `0`: Success
- `1`: Error (file not found, parsing error, template error, etc.)