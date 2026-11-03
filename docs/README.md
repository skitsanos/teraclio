# Teraclio Documentation

Welcome to the Teraclio documentation! This directory contains comprehensive guides for using Teraclio effectively.

## Getting Started

1. **[Installation Guide](installation.md)** - Download binaries or build from source
2. **[Usage Guide](usage.md)** - Command-line interface and basic usage

## Template Development

3. **[Tera Template Basics](tera-basics.md)** - Tera 2 template syntax, built-ins, and gotchas
4. **[Custom Filters](custom-filters.md)** - Teraclio's 24 additional filters
5. **[Examples](examples.md)** - Real-world usage scenarios and templates
6. **[Use Cases](use-cases.md)** - End-to-end CI/CD, IaC, reporting, and docs generation

## Quick Reference

### Basic Syntax
- `{{ variable }}` - Display variables
- `{% if condition %}` - Conditionals  
- `{% for item in items %}` - Loops
- `{{ value | filter }}` - Apply filters

### Custom Filters
- Hashing: `md5`, `sha1`, `sha256`, `hmac_sha256(key=...)`
- Encoding: `base64_encode`, `base64_decode`, `url_encode`, `url_decode`
- Escaping: `html_escape`, `html_unescape`, `xml_escape`
- Serialization: `json_encode`, `yaml_encode`
- Text: `truncate_words(count=...)`, `regex_replace(pattern=..., replacement=...)`
- Case: `snake_case`, `kebab_case`, `camel_case`, `pascal_case`, `slug`
- Other: `date_format(format=...)`, `uuid`, `bytes_to_str`, `str_to_bytes`
- Function: `now()`, `now(utc=true)`, `now(timestamp=true)`

Run `teraclio --list-filters` for the same list from the binary.

### Command Line
```bash
teraclio --source data.json --template template.txt [--dest output.txt]
```

## External Resources

- [Tera Template Engine](https://keats.github.io/tera/) - Official documentation
- [Jinja2 Documentation](https://jinja.palletsprojects.com/) - Similar syntax, but Jinja2/Tera 1 idioms such as `+` string concatenation, positional filter arguments, `.items()`, and macros do not work in Tera 2
- [JSON Specification](https://www.json.org/) - Data format specification

## Contributing to Documentation

Found an error or want to improve the documentation? Please:

1. Open an issue on [GitHub](https://github.com/skitsanos/teraclio/issues)
2. Submit a pull request with improvements
3. Suggest new examples or use cases

All documentation is written in Markdown and located in the `docs/` directory.