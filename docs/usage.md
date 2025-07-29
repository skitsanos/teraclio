# Usage Guide

## Command Line Interface

Teraclio provides a simple command-line interface for template rendering:

```bash
teraclio --source <json-file> --template <template-file> [--dest <output-file>]
```

### Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--source` | `-s` | ✅ Yes | Path to JSON data file |
| `--template` | `-t` | ✅ Yes | Path to Tera template file |
| `--dest` | `-d` | ❌ No | Output file path (stdout if omitted) |

### Examples

#### Basic Usage
```bash
# Output to stdout
teraclio --source data.json --template template.txt

# Output to file
teraclio --source data.json --template template.txt --dest output.txt
```

#### Using Short Arguments
```bash
# Equivalent to the above
teraclio -s data.json -t template.txt -d output.txt
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

JSON data is accessible in templates through the `data` root element:

### Input JSON
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

### Template Access
```jinja2
User: {{ data.user.name }} ({{ data.user.email }})
Items: {{ data.items | join(", ") }}
Count: {{ data.count }}
```

## Error Handling

Teraclio provides clear error messages for common issues:

- **File not found**: `JSON source file does not exist: /path/to/file.json`
- **Empty file**: `JSON source file is empty`
- **Invalid JSON**: `JSON parsing error: expected value at line 1 column 1`
- **Template error**: `Template error: Variable 'missing_var' not found`
- **Invalid template**: `Template file does not exist: /path/to/template.txt`

## Environment Variables

You can use environment variables in templates with the `get_env` function:

```jinja2
Project: {{ get_env(name="PROJECT_NAME", default="Unknown") }}
Build: {{ get_env(name="BUILD_NUMBER", default="dev") }}
```

## Exit Codes

- `0`: Success
- `1`: Error (file not found, parsing error, template error, etc.)