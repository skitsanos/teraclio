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
- 🔧 **Extensive Filter Library** - 16+ custom filters for data transformation
- 📊 **Multi-Format Input** - JSON, YAML, TOML with auto-detection
- 🌍 **Cross-Platform** - Linux, macOS, Windows (Intel & ARM)
- ⚡ **Flexible output** - File output or stdout, with input format detection
- 🔒 **Security Filters** - Hash generation (MD5, SHA1, SHA256)
- 🌐 **Web-Ready** - URL encoding, HTML/XML escaping
- 🔤 **Case Conversion** - snake_case, kebab-case, camelCase, PascalCase
- 🌍 **Environment Integration** - Access environment variables in templates

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
| `--source, -s` | ✅ | Data file path (JSON, YAML, TOML) or `-` to read from stdin |
| `--template, -t` | ✅ | Tera template file |
| `--dest, -d` | ❌ | Output file (stdout if omitted) |
| `--format, -f` | ❌ | Input format (auto-detected from file extension; required for stdin and unknown extensions) |
| `--env-vars` | ❌ | Include environment variables as `data.env` |

Notes:
- Files without a known extension are treated as JSON.
- `--source -` reads JSON/YAML/TOML data from stdin; `--format` is required in this mode.
- `--env-vars` requires the input data root to be an object; non-object inputs now return a clear validation error.

## Quick Examples

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

Teraclio extends Tera with 16+ custom filters organized by category:

**Hash & Security**
- `md5`, `sha1`, `sha256` - Generate cryptographic hashes
- `base64_encode` / `base64_decode` - Base64 operations

**Web & URL**
- `url_encode` / `url_decode` - URL encoding/decoding
- `html_escape` / `xml_escape` - HTML/XML entity escaping

**String Transformation**
- `snake_case`, `kebab_case`, `camel_case`, `pascal_case` - Case conversions
- `bytes_to_str` / `str_to_bytes` - Bytes/string conversion

See [Custom Filters Documentation](docs/custom-filters.md) for complete details and examples.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
