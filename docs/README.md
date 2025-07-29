# Teraclio Documentation

Welcome to the Teraclio documentation! This directory contains comprehensive guides for using Teraclio effectively.

## Getting Started

1. **[Installation Guide](installation.md)** - Download binaries or build from source
2. **[Usage Guide](usage.md)** - Command-line interface and basic usage

## Template Development

3. **[Tera Template Basics](tera-basics.md)** - Complete guide to Tera template syntax
4. **[Custom Filters](custom-filters.md)** - Teraclio's additional filters for data transformation
5. **[Examples](examples.md)** - Real-world usage scenarios and templates

## Quick Reference

### Basic Syntax
- `{{ variable }}` - Display variables
- `{% if condition %}` - Conditionals  
- `{% for item in items %}` - Loops
- `{{ value | filter }}` - Apply filters

### Custom Filters
- `base64_encode` / `base64_decode` - Base64 operations
- `bytes_to_str` / `str_to_bytes` - Bytes/string conversion

### Command Line
```bash
teraclio --source data.json --template template.txt [--dest output.txt]
```

## External Resources

- [Tera Template Engine](https://keats.github.io/tera/) - Official documentation
- [Jinja2 Documentation](https://jinja.palletsprojects.com/) - Similar syntax reference
- [JSON Specification](https://www.json.org/) - Data format specification

## Contributing to Documentation

Found an error or want to improve the documentation? Please:

1. Open an issue on [GitHub](https://github.com/skitsanos/teraclio/issues)
2. Submit a pull request with improvements
3. Suggest new examples or use cases

All documentation is written in Markdown and located in the `docs/` directory.