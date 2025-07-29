# Custom Filters

Teraclio extends Tera's built-in filters with additional functionality for data transformation. These custom filters are particularly useful for encoding/decoding operations and data format conversions.

## Base64 Filters

### `base64_encode`

Encodes a string to Base64 format using standard Base64 encoding.

**Syntax**: `{{ value | base64_encode }}`

**Example**:
```jinja2
Input: {{ data.content }}
Encoded: {{ data.content | base64_encode }}
```

**Input JSON**:
```json
{
  "content": "Hello, World!"
}
```

**Output**:
```
Input: Hello, World!
Encoded: SGVsbG8sIFdvcmxkIQ==
```

### `base64_decode`

Decodes a Base64 string back to its original format.

**Syntax**: `{{ value | base64_decode }}`

**Example**:
```jinja2
Encoded: {{ data.encoded }}
Decoded: {{ data.encoded | base64_decode | bytes_to_str }}
```

**Input JSON**:
```json
{
  "encoded": "SGVsbG8sIFdvcmxkIQ=="
}
```

**Output**:
```
Encoded: SGVsbG8sIFdvcmxkIQ==
Decoded: Hello, World!
```

**Note**: Base64 decode returns bytes, often requiring `bytes_to_str` for text display.

## Bytes Conversion Filters

### `bytes_to_str`

Converts an array of bytes (numbers) to a UTF-8 string.

**Syntax**: `{{ value | bytes_to_str }}`

**Example**:
```jinja2
Bytes: {{ data.byte_array }}
String: {{ data.byte_array | bytes_to_str }}
```

**Input JSON**:
```json
{
  "byte_array": [72, 101, 108, 108, 111]
}
```

**Output**:
```
Bytes: [72, 101, 108, 108, 111]
String: Hello
```

### `str_to_bytes`

Converts a string to an array of bytes (UTF-8 encoding).

**Syntax**: `{{ value | str_to_bytes }}`

**Example**:
```jinja2
Text: {{ data.text }}
Bytes: {{ data.text | str_to_bytes }}
```

**Input JSON**:
```json
{
  "text": "Hi"
}
```

**Output**:
```
Text: Hi
Bytes: [72, 105]
```

## Combined Filter Examples

### Round-trip Base64 Encoding
```jinja2
Original: {{ data.message }}
Encoded: {{ data.message | base64_encode }}
Decoded: {{ data.message | base64_encode | base64_decode | bytes_to_str }}
```

### Binary Data Processing
```jinja2
{% set binary_data = data.text | str_to_bytes %}
Byte count: {{ binary_data | length }}
First byte: {{ binary_data | first }}
Last byte: {{ binary_data | last }}
Back to string: {{ binary_data | bytes_to_str }}
```

### Data Validation Template
```jinja2
{% for item in data.items %}
Item: {{ item.name }}
{% if item.encoded_data %}
  Encoded Size: {{ item.encoded_data | length }} chars
  Decoded Size: {{ item.encoded_data | base64_decode | length }} bytes
  Content: {{ item.encoded_data | base64_decode | bytes_to_str | truncate(length=50) }}
{% endif %}

{% endfor %}
```

## Error Handling

Custom filters include error handling for invalid inputs:

- **base64_decode**: Returns error for invalid Base64 strings
- **bytes_to_str**: Handles invalid UTF-8 sequences gracefully  
- **str_to_bytes**: Always succeeds for valid strings

**Example error case**:
```jinja2
Invalid Base64: {{ "invalid-base64!" | base64_decode }}
```

This would result in a template error: `Failed to decode Base64: invalid input`

## Integration with Tera Built-ins

Custom filters work seamlessly with Tera's built-in filters:

```jinja2
{# Combine with built-in filters #}
{{ data.message | upper | base64_encode }}
{{ data.encoded | base64_decode | bytes_to_str | title }}

{# Use in conditionals #}
{% if data.payload | base64_decode | bytes_to_str | length > 100 %}
Large payload detected
{% endif %}

{# Use in loops #}
{% for item in data.items %}
{{ loop.index }}. {{ item | base64_decode | bytes_to_str }}
{% endfor %}
```