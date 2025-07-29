# Tera Template Basics

Teraclio uses the [Tera](https://keats.github.io/tera/) template engine, which is inspired by Jinja2 and Django templates. This guide covers the essential syntax and features.

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
{{ items.0 }}  {# First item in array #}
```

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
{{ 15 / 3 }}       {# Division: 5 #}
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
{{ date_string | date(format="%Y-%m-%d") }}

{# Chain multiple filters #}
{{ data.description | striptags | truncate(length=50) | title }}
```

### Common Built-in Filters
```jinja2
{# String filters #}
{{ text | upper }}
{{ text | lower }}
{{ text | title }}
{{ text | capitalize }}
{{ html_content | striptags }}

{# Array filters #}
{{ items | length }}
{{ items | first }}
{{ items | last }}
{{ items | reverse }}
{{ items | sort }}
{{ names | join(sep=", ") }}

{# Number filters #}
{{ price | round }}
{{ size | filesizeformat }}
```

## Functions

### Built-in Functions
```jinja2
{# Range function #}
{% for i in range(end=5) %}
  Number: {{ i }}
{% endfor %}

{# Current time #}
Generated: {{ now() | date(format="%Y-%m-%d %H:%M") }}

{# Environment variables #}
Project: {{ get_env(name="PROJECT_NAME", default="Unknown") }}

{# Random numbers #}
Random: {{ get_random(start=1, end=100) }}
```

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