# Tera Template Basics

Teraclio uses [Tera](https://keats.github.io/tera/) 2, a template engine inspired by Jinja2 and Django templates. This guide covers the essential syntax and features. Tera 2 is stricter than Tera 1 and Jinja2. If you are porting old templates, read [Coming from Jinja2 or Tera 1](#coming-from-jinja2-or-tera-1) first.

## Template Syntax

Tera uses three types of delimiters:

- `{{ }}` for **expressions** (variables, function calls)
- `{% %}` for **statements** (control flow, assignments)
- `{# #}` for **comments**

## Variables and Expressions

### Basic Variables
```jinja2
Hello {{ data.name }}!
User ID: {{ data.user.id }}
```

### Dot Notation
```jinja2
{{ product.name }}
{{ user.profile.email }}
```

Array elements need square brackets: `{{ items[0] }}`. The Tera 1 form `{{ items.0 }}` is a syntax error in Tera 2.

### Square Bracket Notation
```jinja2
{{ product["name"] }}
{{ user["profile"]["email"] }}
{{ items[0] }}

{# Dynamic access #}
{% set field = "name" %}
{{ product[field] }}
```

## Data Types and Literals

### Basic Types
```jinja2
{# String literals #}
{{ "Hello World" }}
{{ 'Single quotes' }}
{{ `Backticks` }}

{# Numbers #}
{{ 42 }}
{{ 3.14 }}

{# Booleans #}
{{ true }}
{{ false }}

{# Arrays #}
{{ [1, 2, 3] }}
{{ ["apple", "banana", "cherry"] }}
```

### Math Operations
```jinja2
{{ 5 + 3 }}        {# Addition: 8 #}
{{ 10 - 4 }}       {# Subtraction: 6 #}
{{ 6 * 7 }}        {# Multiplication: 42 #}
{{ 15 / 3 }}       {# Division always returns a float: 5.0 #}
{{ 17 % 5 }}       {# Modulo: 2 #}
```

### Comparisons
```jinja2
{{ age >= 18 }}
{{ name == "admin" }}
{{ count != 0 }}
{{ score > 85 }}
```

### String Concatenation
```jinja2
{{ "Hello " ~ name ~ "!" }}
{{ first_name ~ " " ~ last_name }}
```

Use `~`. The `+` operator only works on numbers: `"a" + "b"` is a render error.

## Control Flow

### Conditionals
```jinja2
{% if user.role == "admin" %}
  Welcome, Administrator!
{% elif user.role == "moderator" %}
  Welcome, Moderator!
{% else %}
  Welcome, User!
{% endif %}

{# Check if variable exists #}
{% if data.optional_field %}
  Field exists: {{ data.optional_field }}
{% else %}
  Field not provided
{% endif %}
```

### Loops
```jinja2
{# Basic loop #}
{% for item in data.items %}
  {{ loop.index }}. {{ item.name }}
{% endfor %}

{# Loop with else (when array is empty) #}
{% for product in data.products %}
  - {{ product.name }}: ${{ product.price }}
{% else %}
  No products available
{% endfor %}

{# Loop variables #}
{% for item in data.items %}
  Index: {{ loop.index }}      {# 1-based #}
  Index0: {{ loop.index0 }}    {# 0-based #}
  First: {{ loop.first }}      {# boolean #}
  Last: {{ loop.last }}        {# boolean #}
  Content: {{ item }}
{% endfor %}
```

### Loop Controls
```jinja2
{# Break and continue #}
{% for item in data.items %}
  {% if item.skip %}{% continue %}{% endif %}
  {% if item.id == target_id %}{% break %}{% endif %}
  Processing: {{ item.name }}
{% endfor %}
```

## Variables and Assignments

### Setting Variables
```jinja2
{% set user_name = data.user.name %}
{% set full_name = data.first_name ~ " " ~ data.last_name %}
{% set items_count = data.items | length %}

Hello {{ user_name }}!
Full name: {{ full_name }}
You have {{ items_count }} items.
```

### Global Variables (in loops)

A plain `{% set %}` inside a `for` loop is scoped to that loop iteration, so accumulators need `set_global`:

```jinja2
{% set_global total = 0 %}
{% for item in data.items %}
  {% set_global total = total + item.price %}
{% endfor %}
Total: ${{ total }}
```

## Filters

Filters transform variables and are chained with the `|` operator:

```jinja2
{{ name | upper }}
{{ content | truncate(length=100) }}
{{ items | length }}
{{ date_string | date_format(format="%Y-%m-%d") }}  {# Teraclio filter #}

{# Chain multiple filters #}
{{ data.description | trim | truncate(length=50) | title }}
```

Filter arguments are always named (`truncate(length=50)`), never positional (`truncate(50)`).

### Built-in Filters

These are the complete set of filters Tera 2 ships with. Teraclio adds 24 more (see [Custom Filters](custom-filters.md)).

```jinja2
{# Strings #}
{{ text | upper }}  {{ text | lower }}  {{ text | title }}  {{ text | capitalize }}
{{ text | trim }}  {{ text | trim_start(pat="x") }}  {{ text | trim_end }}
{{ text | replace(from="_", to=" ") }}
{{ text | truncate(length=20) }}           {# appends "…"; use end="" to disable #}
{{ text | split(pat=",") }}
{{ text | wordcount }}
{{ text | indent(width=4) }}
{{ text | newlines_to_br }}
{{ count | pluralize(singular="", plural="s") }}

{# Escaping and safety #}
{{ text | escape }}  {{ text | escape_html }}  {{ text | escape_xml }}
{{ markup | safe }}

{# Arrays and maps #}
{{ items | length }}  {{ items | first }}  {{ items | last }}  {{ items | nth(n=2) }}
{{ items | reverse }}  {{ items | unique }}
{{ items | sort }}  {{ people | sort(attribute="age") }}
{{ names | join(sep=", ") }}
{{ people | group_by(attribute="team") }}
{{ config | keys }}  {{ config | values }}  {{ config | pairs }}
{{ config | get(key="port", default=8080) }}

{# Numbers and conversion #}
{{ price | round }}  {{ price | round(method="floor", precision=2) }}  {{ delta | abs }}
{{ "42" | int }}  {{ "3.14" | float }}  {{ 42 | str }}

{# Fallbacks #}
{{ data.optional | default(value="N/A") }}
```

Tera 1 / Jinja2 filters that **do not exist** in Tera 2 include `date`, `striptags`, `filesizeformat`, `map`, `sum`, `filter`, `slice`, `concat`, `string`, `tojson`, and `urlencode`. Teraclio covers several of these: `date_format`, `json_encode`, and `url_encode`.

## Functions

### Built-in Functions

Tera 2 has only two built-in functions:

```jinja2
{# range(end=N), optionally with start= and step_by= #}
{% for i in range(end=5) %}
  Number: {{ i }}
{% endfor %}

{# throw(message=...) aborts rendering with an error #}
{% if not data.version %}{{ throw(message="data.version is required") }}{% endif %}
```

Tera 1's `now()`, `get_env()`, and `get_random()` are gone from Tera itself. Teraclio provides these replacements:

| Need | Teraclio approach |
|------|-------------------|
| Current time | `{{ now() }}` (local, RFC 3339), `{{ now(utc=true) }}`, or `{{ now(timestamp=true) }}` (Unix seconds); format with `{{ now() \| date_format(format="%Y-%m-%d") }}` |
| Environment variable | `--env-vars` then `{{ data.env.PROJECT_NAME \| default(value="Unknown") }}` |
| Random identifier | `{{ "" \| uuid }}` |

## Comments and Raw Content

### Comments
```jinja2
{# This is a comment and won't appear in output #}
{# 
   Multi-line
   comment
#}
```

### Raw Content
```jinja2
{% raw %}
This content won't be processed: {{ variable }}
{% endraw %}
```

## Whitespace Control

Control whitespace around template tags:

```jinja2
{# Remove whitespace before #}
{%- if condition %}
  Content
{%- endif %}

{# Remove whitespace after #}
{% if condition -%}
  Content
{% endif -%}

{# Remove whitespace both sides #}
{%- if condition -%}
  Content
{%- endif -%}
```

## Template Inheritance

Teraclio registers every file in the template's directory under its bare file name. That means `{% extends "base.html" %}` and `{% include "header.html" %}` work when those files sit next to the template being rendered.

### Base Template (`base.html`)
```jinja2
<!DOCTYPE html>
<html>
<head>
    {% block head %}
    <title>{% block title %}Default Title{% endblock %}</title>
    {% endblock %}
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```

### Child Template
```jinja2
{% extends "base.html" %}

{% block title %}Custom Page Title{% endblock %}

{% block content %}
<h1>Page Content</h1>
<p>This content replaces the block in the base template.</p>
{% endblock %}
```

## Autoescaping

Templates whose file name ends in `.html`, `.htm`, or `.xml` are auto-escaped: every `{{ ... }}` output has `&`, `<`, `>`, `"`, and `'` escaped. The same applies when one of those is followed by a template suffix (`page.html.tera`, `feed.xml.j2`). Other templates (`.txt`, `.md`, `.yaml`, `notes.tera`, ...) are not.

```jinja2
{# page.html: data.bio = "<b>Hi</b>" #}
{{ data.bio }}          {# &lt;b&gt;Hi&lt;/b&gt; #}
{{ data.bio | safe }}   {# <b>Hi</b>: only for trusted content #}
```

In auto-escaped templates, do not also apply Teraclio's `html_escape` or `xml_escape`, or the output is escaped twice (`&amp;lt;`).

## Coming from Jinja2 or Tera 1

| Jinja2 / Tera 1 | Tera 2 |
|-----------------|--------|
| `{{ "a" + "b" }}` | `{{ "a" ~ "b" }}` |
| `{{ items.0 }}` | `{{ items[0] }}` |
| `{{ list \| join(", ") }}` | `{{ list \| join(sep=", ") }}` (named arguments only) |
| `{{ "=" * 20 }}` | `{% for i in range(end=20) %}={% endfor %}` |
| `{% for k, v in obj.items() %}` | `{% for k, v in obj %}` (no method calls) |
| `{{ x \| string }}` | `{{ x \| str }}` |
| `{{ x \| tojson }}` | `{{ x \| json_encode }}` (Teraclio filter) |
| `{{ list \| map(attribute="p") \| sum }}` | accumulate with `set_global` in a loop |
| `{{ now() \| date(format=...) }}` | `{{ now() \| date_format(format=...) }}` (Teraclio function and filter) |
| `{{ flag \| lower }}` on a boolean | `{{ flag }}` (filters such as `lower` need strings) |
| `{% macro %}` / `{% import %}` | not supported; use `{% include %}` |

## Best Practices

### Variable Safety
```jinja2
{# Always provide defaults for optional data #}
{{ data.optional_field | default(value="Not provided") }}

{# Check existence before use #}
{% if data.user %}
  Welcome {{ data.user.name }}!
{% endif %}
```

### Performance Tips
```jinja2
{# Store expensive operations in variables #}
{% set items_count = data.items | length %}
{% if items_count > 0 %}
  Found {{ items_count }} items
{% endif %}
```

### Debugging
```jinja2
{# Use the special context variable to debug #}
<pre>{{ __tera_context }}</pre>
```