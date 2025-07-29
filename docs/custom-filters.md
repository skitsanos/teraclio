# Custom Filters

Teraclio extends Tera's built-in filters with 16+ custom filters organized into four main categories: Hash & Security, Web & URL, String Transformation, and Data Conversion. These filters provide comprehensive data transformation capabilities for modern template processing.

## Hash & Security Filters

### Hash Generation

Generate cryptographic hashes for data integrity and security purposes.

#### `md5`
```jinja2
{{ data.password | md5 }}
```
Generates MD5 hash (32 hex characters). Note: MD5 is cryptographically broken, use for non-security purposes only.

#### `sha1` 
```jinja2
{{ data.content | sha1 }}
```
Generates SHA-1 hash (40 hex characters). More secure than MD5 but consider SHA-256 for new applications.

#### `sha256`
```jinja2
{{ data.sensitive_data | sha256 }}
```
Generates SHA-256 hash (64 hex characters). Recommended for security-critical applications.

### Base64 Encoding

#### `base64_encode`
```jinja2
Input: {{ data.content }}
Encoded: {{ data.content | base64_encode }}
```

#### `base64_decode` 
```jinja2
Decoded: {{ data.encoded | base64_decode | bytes_to_str }}
```
**Note**: Returns bytes, use `bytes_to_str` for text display.

## Web & URL Filters

Perfect for web development and API integrations.

### `url_encode`
```jinja2
Original: {{ data.query }}
Encoded: {{ data.query | url_encode }}
```
**Example**: `Hello World!` → `Hello%20World%21`

### `url_decode`
```jinja2
Encoded: {{ data.encoded_url }}
Decoded: {{ data.encoded_url | url_decode }}
```

### `html_escape`
```jinja2
Safe HTML: {{ data.user_content | html_escape }}
```
**Example**: `<script>alert('xss')</script>` → `&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;`

### `xml_escape`
```jinja2
Safe XML: {{ data.xml_content | xml_escape }}
```
Escapes XML special characters for safe embedding.

## String Transformation Filters

Convert between different naming conventions.

### `snake_case`
```jinja2
{{ "HelloWorld" | snake_case }}
```
**Output**: `hello_world`

### `kebab_case`
```jinja2
{{ "HelloWorld" | kebab_case }}
```
**Output**: `hello-world`

### `camel_case`
```jinja2
{{ "hello_world" | camel_case }}
```
**Output**: `helloWorld`

### `pascal_case`
```jinja2
{{ "hello_world" | pascal_case }}
```
**Output**: `HelloWorld`

## Data Conversion Filters

Low-level data type conversions for advanced use cases.

### `bytes_to_str`
```jinja2
String: {{ data.byte_array | bytes_to_str }}
```
Converts byte array `[72, 101, 108, 108, 111]` to `"Hello"`

### `str_to_bytes`
```jinja2
Bytes: {{ data.text | str_to_bytes }}
```
Converts `"Hi"` to `[72, 105]`

## Real-World Examples

### Security Token Generation
```jinja2
User: {{ data.username }}
Token: {{ (data.username + ":" + data.timestamp) | sha256 }}
API Key: {{ data.api_secret | base64_encode }}
```

### Web Form Processing
```jinja2
Search Query: {{ data.user_query | url_encode }}
Safe Display: {{ data.user_input | html_escape }}
CSS Class: {{ data.component_name | kebab_case }}
```

### Data Pipeline Template
```jinja2
{% for record in data.records %}
{{ loop.index }}. Processing {{ record.filename | pascal_case }}
   Hash: {{ record.content | sha256 }}
   Size: {{ record.content | length }} chars
   Encoded: {{ record.content | base64_encode | truncate(length=20) }}...
{% endfor %}
```

### API Response Template
```jinja2
{
  "user_id": "{{ data.user.id }}",
  "display_name": "{{ data.user.name | html_escape }}",
  "profile_url": "/users/{{ data.user.name | url_encode }}",
  "avatar_hash": "{{ data.user.email | md5 }}",
  "api_version": "{{ data.version | snake_case }}"
}
```

### Configuration Generator
```jinja2
# Generated configuration
{% for service in data.services %}
[service.{{ service.name | snake_case }}]
endpoint = "{{ service.url | url_encode }}"
api_key_hash = "{{ service.api_key | sha256 }}"
display_name = "{{ service.name | pascal_case }}"
{% endfor %}
```

## Filter Chaining & Integration

All custom filters integrate seamlessly with Tera's built-in filters:

```jinja2
{# Chain multiple transformations #}
{{ data.api_name | snake_case | upper }}
{{ data.user_input | html_escape | truncate(length=50) }}

{# Complex data processing #}
{% for item in data.items %}
{{ loop.index }}. {{ item.name | pascal_case }}
   Hash: {{ item.content | sha256 | truncate(length=16) }}...
   URL: /api/{{ item.name | kebab_case }}/{{ item.id }}
{% endfor %}

{# Conditional processing #}
{% if data.password | length > 8 %}
Strong password hash: {{ data.password | sha256 }}
{% else %}
Weak password detected
{% endif %}
```

## Error Handling

All custom filters include comprehensive error handling:

- **Hash filters**: Always succeed for string inputs
- **Base64 operations**: Validate input format and encoding
- **URL operations**: Handle special characters correctly
- **Case conversions**: Work with any Unicode string
- **Escape filters**: Prevent injection attacks

Invalid operations will result in clear template errors with descriptive messages.

## Performance Notes

- Hash operations are cryptographically secure but computationally expensive
- Case conversions are optimized for common ASCII strings
- URL and HTML escaping use efficient lookup tables
- Base64 operations use standard library implementations

For high-volume template rendering, consider caching computed hashes and avoiding repeated expensive operations in loops.