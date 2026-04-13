# Usage Guide

## Command Line Interface

Teraclio provides a simple command-line interface for template rendering:

```bash
teraclio --source <data-file> --template <template-file> [OPTIONS]
```

### Arguments

| Argument | Short | Required | Description |
|----------|-------|----------|-------------|
| `--source` | `-s` | Yes | Data file path(s), repeatable, or `-` for stdin |
| `--template` | `-t` | Yes | Template file or directory path |
| `--dest` | `-d` | No | Output file or directory |
| `--format` | `-f` | No | Input format (json, yaml, toml) - auto-detected from file extension, required when reading from stdin or files with unknown extension |
| `--env-vars` | - | No | Include environment variables as `data.env` object |
| `--set KEY=VALUE` | - | No | Set template variables directly (repeatable) |
| `--check` | - | No | Validate template without rendering |
| `--diff` | - | No | Show diff against existing destination file |
| `--watch` | `-w` | No | Watch source/template for changes and re-render automatically |
| `--strict` | - | No | Enable strict mode (undefined variables cause errors) |
| `--output-format` | - | No | Validate rendered output format (json, yaml, toml) |
| `--recursive` | `-r` | No | Recurse into subdirectories in directory mode |
| `--quiet` | `-q` | No | Suppress stderr informational messages |
| `--list-filters` | - | No | List all available Tera filters and exit |
| `--completions` | - | No | Generate shell completions and exit |

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

# Files without an extension default to JSON
teraclio -s config -t template.txt

# YAML (auto-detected)
teraclio -s config.yaml -t template.txt

# TOML (auto-detected)
teraclio -s config.toml -t template.txt

# Explicit format specification
# (required for unknown extensions)
teraclio -s data.txt -t template.txt --format yaml
```

#### Reading from stdin
```bash
curl -s https://example.com/data.json | teraclio --source - --format json -t template.txt
```

#### Multiple Sources with Deep Merge
```bash
# Merge multiple data files into a single context
# Later files override earlier ones for conflicting keys
teraclio -s defaults.yaml -s overrides.yaml -t template.txt

# Combine base config with environment-specific values
teraclio -s base.json -s production.json -t app-config.yaml -d config.yaml
```

#### Ad-Hoc Variables with --set
```bash
# Set individual template variables from the command line
teraclio -s data.json -t template.txt --set version=2.0.1 --set env=production

# Use --set without a source file for simple variable injection
teraclio -s defaults.yaml -t template.txt --set build_number=42 --set commit_sha=abc123
```

#### Template Validation with --check
```bash
# Validate that a template is syntactically correct without rendering
teraclio -s data.json -t template.txt --check

# Useful in CI to catch template errors early
teraclio -s config.yaml -t deployment.yaml --check && echo "Template OK"
```

#### Diff Mode
```bash
# Preview changes before overwriting an existing file
teraclio -s data.json -t template.txt -d output.txt --diff

# Useful in code review or dry-run workflows
teraclio -s config.yaml -t k8s-deployment.yaml -d deployment.yaml --diff
```

#### Watch Mode
```bash
# Automatically re-render when source or template files change
teraclio -s data.json -t template.txt -d output.txt --watch

# Combine with quiet mode for less noise during development
teraclio -s data.yaml -t page.html -d index.html --watch --quiet
```

#### Directory Mode with --recursive
```bash
# Render all templates in a directory
teraclio -s data.json -t templates/ -d output/

# Recurse into subdirectories
teraclio -s data.json -t templates/ -d output/ --recursive
```

#### Output Format Validation
```bash
# Ensure the rendered output is valid JSON
teraclio -s data.yaml -t api-response.json -d output.json --output-format json

# Validate YAML output
teraclio -s config.json -t k8s-manifest.yaml --output-format yaml
```

#### Quiet Mode
```bash
# Suppress informational messages on stderr
teraclio -s data.json -t template.txt --quiet

# Useful in scripts where only stdout output matters
teraclio -s data.json -t template.txt --quiet > output.txt
```

#### Listing Filters and Generating Completions
```bash
# List all available Tera filters
teraclio --list-filters

# Generate shell completions (bash, zsh, fish, etc.)
teraclio --completions bash > ~/.local/share/bash-completion/completions/teraclio
teraclio --completions zsh > ~/.zfunc/_teraclio
```

#### Template Includes
```bash
# Templates can include other templates using Tera's include tag
# Given a template that contains: {% include "header.html" %}
# Teraclio resolves includes relative to the template's directory
teraclio -s data.json -t templates/page.html -d output.html
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
teraclio -s data.json -t template.txt -d output.txt -w -q
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

All input data (JSON, YAML, TOML) is accessible through the `data` root element. When multiple sources are provided, their contents are deep-merged in order, with later files taking precedence for conflicting keys.

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

# Ad-hoc variables (when --set is used)
Version: {{ version }}
```

## Error Handling

Teraclio provides clear error messages for common issues:

- **File not found**: `Data source file does not exist: /path/to/file.json`
- **Empty file**: `Data source file is empty`
- **Invalid format**: `YAML parsing error: expected value at line 1 column 1`
- **Template error**: `Template error: Variable 'missing_var' not found`
- **Invalid template**: `Template file does not exist: /path/to/template.txt`
- **Unsupported format**: `Unsupported input format for file '...' Supported formats: json, yaml, toml`
- **Invalid env-vars input**: `Cannot include environment variables: data source must be a JSON object when --env-vars is used.`
- **Stdin without format**: `When reading from stdin, --format must be specified (json, yaml, or toml)`
- **Output format validation**: `Rendered output is not valid JSON/YAML/TOML`
- **Strict mode**: `Variable 'undefined_var' not found in strict mode`

## Advanced Usage Patterns

### Configuration File Processing
```bash
# Process different config formats
teraclio -s app.yaml -t k8s-deployment.yaml --env-vars
teraclio -s database.toml -t sql-migration.sql
teraclio -s api.json -t openapi-spec.yaml

# Merge base and override configs
teraclio -s defaults.yaml -s app.yaml -s secrets.yaml -t k8s-deployment.yaml
```

### Pipeline Integration
```bash
# In CI/CD pipelines
curl -s $API_ENDPOINT | jq '.' > data.json
teraclio -s data.json -t report.html --env-vars > report.html

# Docker builds
teraclio -s build-config.yaml -t Dockerfile.template > Dockerfile

# Validate templates in CI before deploying
teraclio -s config.yaml -t deployment.yaml --check

# Preview changes before applying
teraclio -s config.yaml -t deployment.yaml -d deployment.yaml --diff
```

### Environment-Specific Templating
```bash
# Development environment
teraclio -s config.dev.yaml -t app-config.yaml --env-vars

# Production environment
teraclio -s config.prod.yaml -t app-config.yaml --env-vars

# Layer common defaults with environment overrides
teraclio -s config.base.yaml -s config.prod.yaml -t app-config.yaml --set deploy_time="$(date -u)"
```

### Watch Mode for Development
```bash
# Live-reload templates during development
teraclio -s data.json -t index.html -d dist/index.html --watch

# Watch a directory of templates
teraclio -s data.json -t templates/ -d dist/ --recursive --watch
```

## Exit Codes

- `0`: Success
- `1`: Error (file not found, parsing error, template error, etc.)
