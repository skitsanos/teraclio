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
- 🔧 **Custom Filters** - Base64 encoding/decoding, bytes conversion
- 📊 **JSON Integration** - Direct JSON data access in templates
- 🌍 **Cross-Platform** - Linux, macOS, Windows (Intel & ARM)
- ⚡ **Stdout Support** - Output to file or stdout

## Documentation

- **[Installation Guide](docs/installation.md)** - Download binaries or build from source
- **[Usage Guide](docs/usage.md)** - Command-line interface and examples
- **[Custom Filters](docs/custom-filters.md)** - Base64 and bytes conversion filters
- **[Tera Template Basics](docs/tera-basics.md)** - Template syntax and features
- **[Examples](docs/examples.md)** - Real-world usage scenarios

## Basic Usage

```bash
teraclio --source <json-file> --template <template-file> [--dest <output-file>]
```

| Argument | Required | Description |
|----------|----------|-------------|
| `--source, -s` | ✅ | JSON data file |
| `--template, -t` | ✅ | Tera template file |
| `--dest, -d` | ❌ | Output file (stdout if omitted) |

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

## Custom Filters

Teraclio extends Tera with additional filters:

- `base64_encode` / `base64_decode` - Base64 operations
- `bytes_to_str` / `str_to_bytes` - Bytes/string conversion

See [Custom Filters Documentation](docs/custom-filters.md) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
