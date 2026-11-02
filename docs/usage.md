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
| `--format` | `-f` | No | Input format (json, yaml, toml). Auto-detected from file extension; required for stdin and for unrecognised extensions. Applies to every `--source` |
| `--env-vars` | - | No | Include environment variables as `data.env` object |
| `--set KEY=VALUE` | - | No | Set `data.KEY` to a string, overriding merged sources. Dotted keys nest. Repeatable |
| `--set-json KEY=JSON` | - | No | Like `--set`, but the value is parsed as JSON (number, bool, array, object). Applied after `--set`. Repeatable |
| `--check` | - | No | Render the template to validate it, but write nothing |
| `--diff` | - | No | Print a unified diff against the `--dest` file instead of writing it (`--dest` required). Exits 1 if anything differs |
| `--watch` | `-w` | No | Watch source/template for changes and re-render automatically |
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
# They are available as {{ data.version }} and {{ data.stage }}
teraclio -s data.json -t template.txt --set version=2.0.1 --set stage=production

# Dotted keys nest into the data, keeping sibling keys from the sources:
# sets data.app.db.host and leaves data.app.name alone
teraclio -s app.yaml -t template.txt --set app.db.host=db.internal

# --set values are always strings: --set build_number=42 yields "42".
# Use --set-json for numbers, booleans, arrays, and objects:
teraclio -s app.yaml -t template.txt --set-json replicas=3 --set-json debug=true \
  --set-json 'tags=["web","prod"]'

# A --source is still required. --set-json is applied after --set, so it wins
# when both name the same key. Setting a key below a non-object
# (app.name.x when app.name is a string) is an error.
```

#### Template Validation with --check
```bash
# Render the template against real data and fail on any error, without writing output
teraclio -s data.json -t template.txt --check

# Useful in CI to catch template errors early
teraclio -s config.yaml -t deployment.yaml --check && echo "Template OK"
```

#### Diff Mode
```bash
# Preview changes before overwriting an existing file
teraclio -s data.json -t template.txt -d output.txt --diff

# Exit status: 0 = identical, 1 = differs (or error). Fail CI when committed
# output has drifted from its template:
teraclio -s config.yaml -t k8s-deployment.yaml.tera -d deployment.yaml --diff -q

# Works in directory mode too: exits 1 if any file differs
teraclio -s data.json -t templates/ -d output/ -r --diff
```

#### Watch Mode
```bash
# Automatically re-render when source or template files change.
# Watch mode follows editors that save via rename, ignores its own output and
# editor scratch files (.swp, ~), and renders once per burst of changes.
# A render error is reported and watching continues.
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

Every non-hidden file in the directory is a template. Output names drop a
`.tera`, `.j2`, `.jinja`, or `.jinja2` suffix, so `templates/config.yaml.tera`
renders to `output/config.yaml`. If two templates would produce the same output
name (`config.yaml` and `config.yaml.tera`), the run fails rather than letting
one overwrite the other. Templates in the same directory can include each other.

#### Output Files

Files written with `--dest` are replaced atomically: Teraclio writes a hidden
temporary file next to the destination and renames it into place, so a dev
server or watcher never reads a half-written file. Missing parent directories
are created. If the destination is a symlink, its target is updated and the
link is kept, and an existing file's permissions are preserved.

#### Output Format Validation
```bash
# Ensure the rendered output is valid JSON
teraclio -s data.yaml -t api-response.json -d output.json --output-format json

# Validate YAML output (multi-document `---` streams are accepted)
teraclio -s config.json -t k8s-manifest.yaml.tera --output-format yaml
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
# Templates can include (or extend) other templates using Tera's include tag
# Given a template that contains: {% include "header.html" %}
# Teraclio registers every file in the template's own directory under its bare
# file name (except the --source files and the --dest file), so header.html must
# sit next to page.html. Subdirectory paths such as
# {% include "partials/header.html" %} are not resolved.
teraclio -s data.json -t templates/page.html -d output.html
```

#### Environment Variables Integration
```bash
# Include environment variables as data.env
teraclio -s data.json -t template.txt --env-vars

# Template can access: {{ data.env.HOME }}, {{ data.env.USER }}, etc.
# Missing variables fail the render; guard optional ones with default:
#   {{ data.env.BUILD_ID | default(value="local") }}
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
Generated: {{ now() | date_format(format="%Y-%m-%d %H:%M") }}

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
# Top-level keys must come before the first [table]
items = ["apple", "banana", "cherry"]
count = 3

[user]
name = "John Doe"
email = "john@example.com"
```

TOML dates and datetimes (`created = 2024-01-15T10:30:00Z`) reach templates as strings in RFC 3339 / ISO 8601 form, ready for `date_format`.

### Template Access
```jinja2
User: {{ data.user.name }} ({{ data.user.email }})
Items: {{ data.items | join(sep=", ") }}
Count: {{ data.count }}

# Environment variables (when --env-vars is used)
Home: {{ data.env.HOME }}
User: {{ data.env.USER }}

# Ad-hoc variables (when --set is used): also under data
Version: {{ data.version }}
```

## Error Handling

Teraclio prints errors to stderr as `Error: ...` and exits with code 1. Validation errors carry an `Invalid input:` prefix. Common messages:

- **File not found**: `Data source file does not exist: /path/to/file.json`
- **Empty file**: `Data source file is empty`
- **Invalid format**: `YAML parsing error: expected a node but found Value at line 2 column 4` (JSON: `JSON parsing error: ...`, TOML: `TOML parsing error: ...`)
- **Undefined variable**: ``Template error: error: Field `missing_var` is not defined. Available fields: ...`` (Tera 2 points at the exact line and column)
- **Invalid template**: `Template file does not exist: /path/to/template.txt`
- **Unsupported format**: `Unsupported input format for file '...' Supported formats: json, yaml, toml`
- **Invalid env-vars input**: `Cannot include environment variables: data source must be a JSON object when --env-vars is used.`
- **Stdin without format**: `When reading from stdin, --format must be specified (json, yaml, or toml)`
- **Output format validation**: `Output is not valid JSON: ...` (or YAML / TOML)
- **Unknown filter or function**: ``Template error: error: Unknown filter `date` `` (caught before rendering)
- **Diff without destination**: `--dest is required when using --diff`
- **Directory mode without destination**: `--dest is required when using directory mode`
- **Destination overwrites the template**: `--dest must not be the template file itself` (or `template directory itself`)

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
teraclio -s data.json -t report.html.tera --env-vars > report.html

# Docker builds
teraclio -s build-config.yaml -t Dockerfile.template > Dockerfile

# Validate templates in CI before deploying
teraclio -s config.yaml -t deployment.yaml --check

# Preview changes before applying
teraclio -s config.yaml -t deployment.yaml.tera -d deployment.yaml --diff
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

- `0`: Success (with `--diff`: no differences)
- `1`: Error (file not found, parsing error, template error, etc.), or `--diff` found differences

`--strict` is still accepted for backward compatibility but does nothing (undefined variables always fail) and is hidden from `--help`.
