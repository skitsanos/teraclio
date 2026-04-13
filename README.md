# Teraclio

A powerful CLI tool for template rendering using the [Tera](https://keats.github.io/tera/) template engine. Transform JSON data into any text format with custom filters for encoding, decoding, and data manipulation.

## Quick Start

```bash
# Install from releases
wget https://github.com/skitsanos/teraclio/releases/latest/download/teraclio-linux-amd64.tar.gz
tar -xzf teraclio-linux-amd64.tar.gz
sudo mv teraclio /usr/local/bin/

# Basic usage
echo '{"name": "World"}' > data.json
echo 'Hello {{ data.name }}!' > template.txt
teraclio --source data.json --template template.txt
# Output: Hello World!
```

## Features

- 🚀 **Fast & Lightweight** - Single binary with no dependencies
- 📝 **Powerful Templating** - Full Tera template engine support
- 🔧 **Extensive Filter Library** - 24 custom filters for data transformation
- 📊 **Multi-Format Input** - JSON, YAML, TOML with auto-detection
- 🌍 **Cross-Platform** - Linux, macOS, Windows (Intel & ARM)
- ⚡ **Flexible output** - File output or stdout, with input format detection
- 🔒 **Security Filters** - Hash generation (MD5, SHA1, SHA256)
- 🌐 **Web-Ready** - URL encoding, HTML/XML escaping
- 🔤 **Case Conversion** - snake_case, kebab-case, camelCase, PascalCase
- 🌍 **Environment Integration** - Access environment variables in templates
- 📦 **Template Includes** - `{% include %}` support for reusable partials
- 📂 **Multiple Data Sources** - Merge multiple JSON/YAML/TOML files
- 👀 **Watch Mode** - Auto re-render on file changes
- 📁 **Directory Mode** - Batch process template folders
- 🐚 **Shell Completions** - bash, zsh, fish, powershell

## Documentation

- **[Installation Guide](docs/installation.md)** - Download binaries or build from source
- **[Usage Guide](docs/usage.md)** - Command-line interface and examples
- **[Custom Filters](docs/custom-filters.md)** - Base64 and bytes conversion filters
- **[Tera Template Basics](docs/tera-basics.md)** - Template syntax and features
- **[Examples](docs/examples.md)** - Real-world usage scenarios

## Basic Usage

```bash
teraclio --source <data-file> --template <template-file> [OPTIONS]
```

| Argument | Required | Description |
|----------|----------|-------------|
| `--source, -s` | ✅ | Data file path(s) (JSON, YAML, TOML) or `-` for stdin. Can be specified multiple times |
| `--template, -t` | ✅ | Template file or directory path |
| `--dest, -d` | ❌ | Output file or directory (stdout if omitted) |
| `--format, -f` | ❌ | Input format (auto-detected; required for stdin) |
| `--env-vars` | ❌ | Include environment variables as `data.env` |
| `--set KEY=VALUE` | ❌ | Set template variables from CLI (repeatable) |
| `--check` | ❌ | Render-check templates without writing output |
| `--diff` | ❌ | Show diff vs destination file instead of writing |
| `--watch, -w` | ❌ | Watch files and re-render on changes |
| `--strict` | ❌ | Compatibility flag; undefined template variables already fail by default |
| `--output-format` | ❌ | Validate output is well-formed (json, yaml, toml) |
| `--recursive, -r` | ❌ | Process template directories recursively |
| `--quiet, -q` | ❌ | Suppress informational messages |
| `--list-filters` | ❌ | List all available filters and exit |
| `--completions` | ❌ | Generate shell completions (bash, zsh, fish, elvish, powershell) |

Notes:
- Files without a known extension are treated as JSON.
- `--source -` reads JSON/YAML/TOML data from stdin; `--format` is required in this mode.
- `--env-vars` requires the input data root to be an object; non-object inputs now return a clear validation error.
- Multiple `--source` flags merge data objects, with later sources overriding earlier ones.

## Quick Examples

### Multiple Sources
```bash
teraclio -s base.json -s overrides.json -t template.txt
```

### Set Variables from CLI
```bash
teraclio -s data.json -t template.txt --set version=2.0
```

### Watch Mode
```bash
teraclio -s data.json -t template.txt -d out.txt --watch
```

### Directory Mode
```bash
teraclio -s data.json -t templates/ -d output/ -r
```

### Template Includes
```html
{# In your template, include reusable partials #}
{% include "header.html" %}
<main>{{ data.content }}</main>
{% include "footer.html" %}
```

### Diff Mode
```bash
teraclio -s data.json -t template.txt -d existing.txt --diff
```

### Output Validation
```bash
teraclio -s data.json -t config.tpl --output-format json
```

### Simple Report Generation
```bash
# Create data
echo '{"user": "Alice", "items": ["laptop", "mouse"]}' > data.json

# Create template  
cat > report.txt << 'EOF'
User Report: {{ data.user }}
Items: {{ data.items | join(sep=", ") }}
Count: {{ data.items | length }}
EOF

# Generate report
teraclio -s data.json -t report.txt
```

### Base64 Operations
```bash
# Encode data
echo '{"message": "Hello World"}' > data.json
echo '{{ data.message | base64_encode }}' > encode.txt
teraclio -s data.json -t encode.txt
# Output: SGVsbG8gV29ybGQ=
```

### Piping data into Teraclio
```bash
curl -s https://api.example.com/data.json | teraclio --source - --format json -t template.txt
```

## Custom Filters

Teraclio extends Tera with 24 custom filters organized by category:

**Hash & Security**
- `md5`, `sha1`, `sha256`, `hmac_sha256` - Cryptographic hashes and HMAC
- `base64_encode` / `base64_decode` - Base64 operations

**Web & URL**
- `url_encode` / `url_decode` - URL encoding/decoding
- `html_escape` / `html_unescape` - HTML entity escaping/unescaping
- `xml_escape` - XML entity escaping

**Serialization**
- `json_encode` - Serialize values to JSON strings
- `yaml_encode` - Serialize values to YAML strings

**Text**
- `truncate_words` - Truncate text to a specified number of words
- `regex_replace` - Replace text matching a regular expression

**String Case**
- `snake_case`, `kebab_case`, `camel_case`, `pascal_case` - Case conversions
- `slug` - URL-friendly slug generation

**Date**
- `date_format` - Format date strings with custom patterns

**UUID**
- `uuid` - Generate unique identifiers

**Data Conversion**
- `bytes_to_str` / `str_to_bytes` - Bytes/string conversion

See [Custom Filters Documentation](docs/custom-filters.md) for complete details and examples.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
