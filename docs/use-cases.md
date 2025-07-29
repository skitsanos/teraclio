# Enterprise Use Cases

This document outlines practical applications of Teraclio in enterprise environments, demonstrating how the tool can streamline operations, improve consistency, and automate complex workflows across different business domains.

## 1. CI/CD Pipeline Configuration Management

### Problem
Organizations often need to deploy the same application across multiple environments (development, staging, production) with different configurations, resource allocations, and security settings. Manually maintaining separate configuration files leads to inconsistencies and deployment errors.

### Solution
Use Teraclio to generate environment-specific pipeline configurations from a single data source, ensuring consistency while allowing environment-specific customization.

**Data Source** (`pipeline-config.yaml`):
```yaml
application:
  name: "customer-api"
  version: "2.1.0"
  repository: "github.com/company/customer-api"

environments:
  development:
    replicas: 1
    cpu_limit: "500m"
    memory_limit: "1Gi"
    database_url: "postgresql://dev-db:5432/customers"
    debug_enabled: true
    log_level: "debug"
  
  staging:
    replicas: 2
    cpu_limit: "1000m"
    memory_limit: "2Gi"
    database_url: "postgresql://staging-db:5432/customers"
    debug_enabled: false
    log_level: "info"
  
  production:
    replicas: 5
    cpu_limit: "2000m"
    memory_limit: "4Gi"
    database_url: "postgresql://prod-db:5432/customers"
    debug_enabled: false
    log_level: "warn"

security:
  secrets:
    - database_password
    - api_key
    - jwt_secret
```

**Template** (`kubernetes-deployment.yaml.template`):
```jinja2
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ data.application.name | kebab_case }}-{{ data.env.ENVIRONMENT | default(value="dev") }}
  namespace: {{ data.env.ENVIRONMENT | default(value="dev") }}
  labels:
    app: {{ data.application.name | kebab_case }}
    version: {{ data.application.version }}
    environment: {{ data.env.ENVIRONMENT | default(value="dev") }}
spec:
  replicas: {{ data.environments[data.env.ENVIRONMENT | default(value="development")].replicas }}
  selector:
    matchLabels:
      app: {{ data.application.name | kebab_case }}
  template:
    metadata:
      labels:
        app: {{ data.application.name | kebab_case }}
        version: {{ data.application.version }}
    spec:
      containers:
      - name: {{ data.application.name | kebab_case }}
        image: {{ data.application.repository }}:{{ data.application.version }}
        resources:
          limits:
            cpu: {{ data.environments[data.env.ENVIRONMENT | default(value="development")].cpu_limit }}
            memory: {{ data.environments[data.env.ENVIRONMENT | default(value="development")].memory_limit }}
        env:
        - name: DATABASE_URL
          value: "{{ data.environments[data.env.ENVIRONMENT | default(value="development")].database_url }}"
        - name: DEBUG_ENABLED
          value: "{{ data.environments[data.env.ENVIRONMENT | default(value="development")].debug_enabled }}"
        - name: LOG_LEVEL
          value: "{{ data.environments[data.env.ENVIRONMENT | default(value="development")].log_level | upper }}"
        - name: APP_VERSION
          value: "{{ data.application.version }}"
        - name: DEPLOYMENT_HASH
          value: "{{ (data.application.name + data.application.version + data.env.ENVIRONMENT | default(value="dev")) | sha256 }}"
        {% for secret in data.security.secrets %}
        - name: {{ secret | snake_case | upper }}
          valueFrom:
            secretKeyRef:
              name: {{ data.application.name | kebab_case }}-secrets
              key: {{ secret | kebab_case }}
        {% endfor %}
---
apiVersion: v1
kind: Service
metadata:
  name: {{ data.application.name | kebab_case }}-service
  namespace: {{ data.env.ENVIRONMENT | default(value="dev") }}
spec:
  selector:
    app: {{ data.application.name | kebab_case }}
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

**Usage in CI/CD Pipeline**:
```bash
# Development deployment
ENVIRONMENT=development teraclio -s pipeline-config.yaml -t kubernetes-deployment.yaml.template --env-vars | kubectl apply -f -

# Production deployment  
ENVIRONMENT=production teraclio -s pipeline-config.yaml -t kubernetes-deployment.yaml.template --env-vars | kubectl apply -f -
```

**Benefits**:
- **Consistency**: Single source of truth for all environments
- **Maintainability**: Changes propagate across all environments automatically
- **Security**: Environment-specific secrets and configurations
- **Auditability**: Clear tracking of configuration changes

---

## 2. Security Compliance Reporting

### Problem
Security teams need to generate compliance reports from various security scanning tools (SAST, DAST, container scans) in standardized formats for different stakeholders (executives, auditors, development teams).

### Solution
Transform security scan results into professional reports with risk assessments, remediation guidance, and executive summaries.

**Data Source** (`security-scan-results.json`):
```json
{
  "scan_metadata": {
    "project": "E-Commerce Platform",
    "scan_date": "2024-01-15T10:30:00Z",
    "scan_types": ["SAST", "DAST", "Container", "Dependencies"],
    "total_files_scanned": 1247,
    "scan_duration": "45 minutes"
  },
  "vulnerabilities": [
    {
      "id": "CVE-2023-12345",
      "severity": "critical",
      "category": "injection",
      "title": "SQL Injection in User Authentication",
      "description": "Unsanitized user input in login endpoint",
      "file": "src/auth/login.py",
      "line": 45,
      "cwe": "CWE-89",
      "remediation": "Use parameterized queries"
    },
    {
      "id": "CVE-2023-67890", 
      "severity": "high",
      "category": "xss",
      "title": "Cross-Site Scripting in User Profile",
      "description": "Unescaped user input in profile display",
      "file": "src/profile/display.js",
      "line": 123,
      "cwe": "CWE-79",
      "remediation": "Implement proper output encoding"
    },
    {
      "id": "DEP-2023-001",
      "severity": "medium", 
      "category": "dependency",
      "title": "Outdated jQuery Version",
      "description": "jQuery 3.5.1 has known security vulnerabilities",
      "file": "package.json",
      "line": 15,
      "remediation": "Update to jQuery 3.7.1 or later"
    }
  ],
  "summary": {
    "critical": 1,
    "high": 1, 
    "medium": 1,
    "low": 0,
    "total": 3
  },
  "compliance_frameworks": ["SOC2", "GDPR", "OWASP Top 10"]
}
```

**Template** (`security-report.md`):
```jinja2
# Security Compliance Report: {{ data.scan_metadata.project }}

**Generated**: {{ data.scan_metadata.scan_date | date(format="%B %d, %Y at %H:%M UTC") }}  
**Report ID**: {{ (data.scan_metadata.project + data.scan_metadata.scan_date) | sha256 | truncate(length=12) }}  
**Compliance Frameworks**: {{ data.compliance_frameworks | join(sep=", ") }}

---

## Executive Summary

This security assessment identified **{{ data.summary.total }}** vulnerabilities across **{{ data.scan_metadata.total_files_scanned }}** files in the {{ data.scan_metadata.project }}.

### Risk Profile
- 🔴 **Critical**: {{ data.summary.critical }} (Immediate action required)
- 🟠 **High**: {{ data.summary.high }} (Fix within 7 days)
- 🟡 **Medium**: {{ data.summary.medium }} (Fix within 30 days)
- 🟢 **Low**: {{ data.summary.low }} (Fix within 90 days)

**Overall Risk Score**: {{ ((data.summary.critical * 10) + (data.summary.high * 7) + (data.summary.medium * 4) + (data.summary.low * 1)) }}/100

---

## Detailed Findings

{% for vuln in data.vulnerabilities %}
### {{ loop.index }}. {{ vuln.title }}

**Severity**: {% if vuln.severity == "critical" %}🔴 CRITICAL{% elif vuln.severity == "high" %}🟠 HIGH{% elif vuln.severity == "medium" %}🟡 MEDIUM{% else %}🟢 LOW{% endif %}  
**Category**: {{ vuln.category | upper }}  
**CVE/ID**: `{{ vuln.id }}`  
**CWE**: {{ vuln.cwe | default(value="N/A") }}

**Description**: {{ vuln.description }}

**Location**: `{{ vuln.file }}:{{ vuln.line }}`

**Remediation**: {{ vuln.remediation }}

**Risk Hash**: `{{ (vuln.id + vuln.file + vuln.line|string) | md5 }}`

---
{% endfor %}

## Compliance Status

{% for framework in data.compliance_frameworks %}
### {{ framework }}
{% if data.summary.critical > 0 or data.summary.high > 0 %}
❌ **Non-Compliant** - Critical/High severity issues present
{% else %}
✅ **Compliant** - No critical or high severity issues
{% endif %}
{% endfor %}

## Remediation Plan

### Immediate Actions (Critical/High Priority)
{% for vuln in data.vulnerabilities %}{% if vuln.severity in ["critical", "high"] %}
- [ ] **{{ vuln.title }}** - {{ vuln.file }}:{{ vuln.line }}
{% endif %}{% endfor %}

### Scheduled Actions (Medium/Low Priority)  
{% for vuln in data.vulnerabilities %}{% if vuln.severity in ["medium", "low"] %}
- [ ] **{{ vuln.title }}** - {{ vuln.file }}:{{ vuln.line }}
{% endif %}{% endfor %}

---

**Report Generated by**: Security Automation Team  
**Next Scan Scheduled**: {{ data.env.NEXT_SCAN_DATE | default(value="Not scheduled") }}  
**Contact**: security@company.com
```

**Usage**:
```bash
# Generate executive report
NEXT_SCAN_DATE="2024-02-01" teraclio -s security-scan-results.json -t security-report.md --env-vars -d security-report-$(date +%Y%m%d).md

# Generate PDF report
pandoc security-report-$(date +%Y%m%d).md -o security-report-$(date +%Y%m%d).pdf
```

**Benefits**:
- **Standardization**: Consistent report format across all projects
- **Automation**: Integration with CI/CD for automatic report generation
- **Traceability**: Unique identifiers and hashes for vulnerability tracking
- **Compliance**: Framework-specific status and remediation tracking

---

## 3. Infrastructure as Code (IaC) Generation

### Problem
DevOps teams need to provision similar infrastructure across multiple cloud providers and environments with consistent naming, tagging, and security policies while accommodating provider-specific differences.

### Solution
Use Teraclio to generate Terraform configurations, CloudFormation templates, and Kubernetes manifests from a unified infrastructure specification.

**Data Source** (`infrastructure-spec.toml`):
```toml
[project]
name = "ecommerce-platform"
environment = "production"
region = "us-east-1"
cost_center = "engineering"
owner = "platform-team"

[network]
vpc_cidr = "10.0.0.0/16"
availability_zones = ["us-east-1a", "us-east-1b", "us-east-1c"]
public_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
private_subnets = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]

[compute]
instance_type = "t3.large"
min_instances = 2
max_instances = 10
desired_instances = 3

[database]
engine = "postgresql"
version = "13.7"
instance_class = "db.t3.medium"
allocated_storage = 100
backup_retention = 7

[security]
ssl_certificate_arn = "arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012"
allowed_cidr_blocks = ["10.0.0.0/8", "172.16.0.0/12"]

[[applications]]
name = "api-server"
port = 8080
health_check_path = "/health"
memory = 2048
cpu = 1024

[[applications]]
name = "worker-service"
port = 8081
health_check_path = "/status"
memory = 1024
cpu = 512
```

**Template** (`terraform-infrastructure.tf.template`):
```jinja2
# {{ data.project.name | pascal_case }} Infrastructure
# Generated on {{ data.env.BUILD_DATE | default(value="unknown") }}
# Environment: {{ data.project.environment | upper }}

terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
  
  backend "s3" {
    bucket = "{{ data.project.name | kebab_case }}-terraform-state"
    key    = "{{ data.project.environment }}/terraform.tfstate"
    region = "{{ data.project.region }}"
  }
}

provider "aws" {
  region = "{{ data.project.region }}"
  
  default_tags {
    tags = {
      Project     = "{{ data.project.name | pascal_case }}"
      Environment = "{{ data.project.environment | title }}"
      CostCenter  = "{{ data.project.cost_center | title }}"
      Owner       = "{{ data.project.owner | kebab_case }}"
      ManagedBy   = "terraform"
      CreatedBy   = "teraclio"
      ConfigHash  = "{{ (data.project.name + data.project.environment) | sha256 | truncate(length=16) }}"
    }
  }
}

# VPC Configuration
resource "aws_vpc" "main" {
  cidr_block           = "{{ data.network.vpc_cidr }}"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "{{ data.project.name | kebab_case }}-vpc"
  }
}

# Internet Gateway
resource "aws_internet_gateway" "main" {
  vpc_id = aws_vpc.main.id

  tags = {
    Name = "{{ data.project.name | kebab_case }}-igw"
  }
}

# Public Subnets
{% for i in range(data.network.public_subnets | length) %}
resource "aws_subnet" "public_{{ loop.index }}" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "{{ data.network.public_subnets[i] }}"
  availability_zone       = "{{ data.network.availability_zones[i] }}"
  map_public_ip_on_launch = true

  tags = {
    Name = "{{ data.project.name | kebab_case }}-public-{{ loop.index }}"
    Type = "public"
  }
}
{% endfor %}

# Private Subnets
{% for i in range(data.network.private_subnets | length) %}
resource "aws_subnet" "private_{{ loop.index }}" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = "{{ data.network.private_subnets[i] }}"
  availability_zone = "{{ data.network.availability_zones[i] }}"

  tags = {
    Name = "{{ data.project.name | kebab_case }}-private-{{ loop.index }}"
    Type = "private"
  }
}
{% endfor %}

# Security Groups
resource "aws_security_group" "application" {
  name_prefix = "{{ data.project.name | kebab_case }}-app-"
  vpc_id      = aws_vpc.main.id
  description = "Security group for {{ data.project.name | title }} applications"

  # Application ports
  {% for app in data.applications %}
  ingress {
    description = "{{ app.name | title }} application port"
    from_port   = {{ app.port }}
    to_port     = {{ app.port }}
    protocol    = "tcp"
    cidr_blocks = {{ data.security.allowed_cidr_blocks | tojson }}
  }
  {% endfor %}

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "{{ data.project.name | kebab_case }}-application-sg"
  }
}

# RDS Database
resource "aws_db_instance" "main" {
  identifier = "{{ data.project.name | kebab_case }}-{{ data.project.environment }}"
  
  engine         = "{{ data.database.engine }}"
  engine_version = "{{ data.database.version }}"
  instance_class = "{{ data.database.instance_class }}"
  
  allocated_storage     = {{ data.database.allocated_storage }}
  max_allocated_storage = {{ data.database.allocated_storage * 2 }}
  
  db_name  = "{{ data.project.name | snake_case }}"
  username = "{{ data.project.name | snake_case }}_admin"
  password = "{{ data.env.DB_PASSWORD | default(value="changeme123!") }}"
  
  backup_retention_period = {{ data.database.backup_retention }}
  backup_window          = "03:00-04:00"
  maintenance_window     = "sun:04:00-sun:05:00"
  
  vpc_security_group_ids = [aws_security_group.database.id]
  db_subnet_group_name   = aws_db_subnet_group.main.name
  
  skip_final_snapshot = {{ data.project.environment != "production" | lower }}
  deletion_protection = {{ data.project.environment == "production" | lower }}

  tags = {
    Name = "{{ data.project.name | kebab_case }}-database"
  }
}

# Auto Scaling Group
resource "aws_autoscaling_group" "main" {
  name                = "{{ data.project.name | kebab_case }}-asg"
  vpc_zone_identifier = [{% for i in range(data.network.private_subnets | length) %}aws_subnet.private_{{ loop.index }}.id{% if not loop.last %}, {% endif %}{% endfor %}]
  
  min_size         = {{ data.compute.min_instances }}
  max_size         = {{ data.compute.max_instances }}
  desired_capacity = {{ data.compute.desired_instances }}
  
  launch_template {
    id      = aws_launch_template.main.id
    version = "$Latest"
  }
  
  tag {
    key                 = "Name"
    value               = "{{ data.project.name | kebab_case }}-instance"
    propagate_at_launch = true
  }
}

# Outputs
output "vpc_id" {
  description = "ID of the VPC"
  value       = aws_vpc.main.id
}

output "database_endpoint" {
  description = "RDS instance endpoint"
  value       = aws_db_instance.main.endpoint
  sensitive   = true
}

output "infrastructure_hash" {
  description = "Hash of the infrastructure configuration"
  value       = "{{ data | tojson | sha256 }}"
}
```

**Usage**:
```bash
# Generate production infrastructure
BUILD_DATE=$(date -Iseconds) DB_PASSWORD="${PROD_DB_PASSWORD}" \
teraclio -s infrastructure-spec.toml -t terraform-infrastructure.tf.template --env-vars -d main.tf

# Apply infrastructure
terraform init && terraform plan && terraform apply
```

**Benefits**:
- **Consistency**: Standardized infrastructure across environments
- **Maintainability**: Single specification for multiple outputs
- **Security**: Environment-specific secrets and configurations
- **Traceability**: Configuration hashes for change tracking

---

## 4. API Documentation Generation

### Problem
Development teams maintain API specifications in OpenAPI/Swagger format but need to generate different documentation formats for different audiences (developers, product managers, QA teams) with varying levels of technical detail.

### Solution
Transform OpenAPI specifications into tailored documentation for different stakeholders while maintaining consistency with the actual API implementation.

**Data Source** (`api-specification.json`):
```json
{
  "api": {
    "name": "Customer Management API",
    "version": "2.1.0",
    "description": "RESTful API for managing customer data and interactions",
    "base_url": "https://api.company.com/v2",
    "authentication": "Bearer Token"
  },
  "endpoints": [
    {
      "path": "/customers",
      "method": "GET",
      "summary": "List customers",
      "description": "Retrieve a paginated list of customers",
      "parameters": [
        {"name": "page", "type": "integer", "required": false, "description": "Page number"},
        {"name": "limit", "type": "integer", "required": false, "description": "Items per page"},
        {"name": "status", "type": "string", "required": false, "description": "Filter by status"}
      ],
      "responses": {
        "200": "Success - Returns customer list",
        "400": "Bad Request - Invalid parameters",
        "401": "Unauthorized - Invalid token"
      },
      "example_response": {
        "customers": [
          {"id": 1, "name": "John Doe", "email": "john@example.com", "status": "active"}
        ],
        "pagination": {"page": 1, "limit": 10, "total": 150}
      }
    },
    {
      "path": "/customers/{id}",
      "method": "GET", 
      "summary": "Get customer details",
      "description": "Retrieve detailed information for a specific customer",
      "parameters": [
        {"name": "id", "type": "integer", "required": true, "description": "Customer ID"}
      ],
      "responses": {
        "200": "Success - Returns customer details",
        "404": "Not Found - Customer doesn't exist",
        "401": "Unauthorized - Invalid token"
      },
      "example_response": {
        "id": 1,
        "name": "John Doe",
        "email": "john@example.com", 
        "status": "active",
        "created_at": "2024-01-15T10:30:00Z"
      }
    },
    {
      "path": "/customers",
      "method": "POST",
      "summary": "Create customer",
      "description": "Create a new customer record",
      "request_body": {
        "name": "string (required)",
        "email": "string (required)",
        "phone": "string (optional)"
      },
      "responses": {
        "201": "Created - Customer created successfully",
        "400": "Bad Request - Invalid data",
        "409": "Conflict - Email already exists"
      }
    }
  ],
  "error_codes": [
    {"code": "INVALID_TOKEN", "message": "The provided authentication token is invalid"},
    {"code": "CUSTOMER_NOT_FOUND", "message": "The requested customer does not exist"},
    {"code": "EMAIL_ALREADY_EXISTS", "message": "A customer with this email already exists"}
  ]
}
```

**Template** (`api-docs-developer.md`):
```jinja2
# {{ data.api.name }} - Developer Documentation

**Version**: {{ data.api.version }}  
**Base URL**: `{{ data.api.base_url }}`  
**Authentication**: {{ data.api.authentication }}  
**Documentation Hash**: {{ data.api.name | sha256 | truncate(length=12) }}

{{ data.api.description }}

---

## Authentication

All API requests require authentication using a Bearer token:

```http
Authorization: Bearer YOUR_API_TOKEN
```

**Example**:
```bash
curl -H "Authorization: Bearer {{ data.env.SAMPLE_TOKEN | default(value="your-token-here") }}" \
     {{ data.api.base_url }}/customers
```

---

## Endpoints

{% for endpoint in data.endpoints %}
### {{ endpoint.method | upper }} {{ endpoint.path }}

**Summary**: {{ endpoint.summary }}

{{ endpoint.description }}

{% if endpoint.parameters %}
#### Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
{% for param in endpoint.parameters %}
| `{{ param.name }}` | {{ param.type }} | {{ "✅ Yes" if param.required else "❌ No" }} | {{ param.description }} |
{% endfor %}
{% endif %}

#### Response Codes

{% for code, description in endpoint.responses.items() %}
- **{{ code }}**: {{ description }}
{% endfor %}

{% if endpoint.request_body %}
#### Request Body

```json
{
{% for field, type in endpoint.request_body.items() %}
  "{{ field }}": "{{ type }}"{% if not loop.last %},{% endif %}
{% endfor %}
}
```
{% endif %}

{% if endpoint.example_response %}
#### Example Response

```json
{{ endpoint.example_response | tojson(indent=2) }}
```
{% endif %}

#### cURL Example

```bash
{% if endpoint.method == "GET" %}
curl -X {{ endpoint.method }} \
  -H "Authorization: Bearer YOUR_TOKEN" \
  "{{ data.api.base_url }}{{ endpoint.path }}"
{% elif endpoint.method == "POST" %}
curl -X {{ endpoint.method }} \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{{ endpoint.request_body | tojson }}' \
  "{{ data.api.base_url }}{{ endpoint.path }}"
{% endif %}
```

#### Endpoint Hash
`{{ (endpoint.method + endpoint.path) | md5 }}`

---
{% endfor %}

## Error Handling

The API uses standard HTTP status codes and returns error details in JSON format:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message",
    "details": "Additional context if available"
  }
}
```

### Common Error Codes

{% for error in data.error_codes %}
#### {{ error.code }}
{{ error.message }}

**Code**: `{{ error.code }}`  
**Hash**: `{{ error.code | md5 }}`
{% endfor %}

---

## SDK Examples

### JavaScript/Node.js

```javascript
const API_BASE = '{{ data.api.base_url }}';
const API_TOKEN = process.env.API_TOKEN;

async function getCustomers(page = 1, limit = 10) {
  const response = await fetch(`${API_BASE}/customers?page=${page}&limit=${limit}`, {
    headers: {
      'Authorization': `Bearer ${API_TOKEN}`,
      'Content-Type': 'application/json'
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${await response.text()}`);
  }
  
  return await response.json();
}
```

### Python

```python
import requests
import os

API_BASE = '{{ data.api.base_url }}'
API_TOKEN = os.getenv('API_TOKEN')

def get_customers(page=1, limit=10):
    headers = {
        'Authorization': f'Bearer {API_TOKEN}',
        'Content-Type': 'application/json'
    }
    
    response = requests.get(
        f'{API_BASE}/customers',
        params={'page': page, 'limit': limit},
        headers=headers
    )
    
    response.raise_for_status()
    return response.json()
```

---

**Generated**: {{ data.env.BUILD_DATE | default(value="Unknown") }}  
**Environment**: {{ data.env.ENVIRONMENT | default(value="development") | upper }}  
**Contact**: {{ data.env.API_CONTACT | default(value="api-team@company.com") }}
```

**Usage**:
```bash
# Generate developer documentation
BUILD_DATE=$(date -Iseconds) ENVIRONMENT=production API_CONTACT="developers@company.com" \
teraclio -s api-specification.json -t api-docs-developer.md --env-vars -d api-documentation.md

# Generate different formats
pandoc api-documentation.md -o api-documentation.pdf
pandoc api-documentation.md -o api-documentation.html
```

**Benefits**:
- **Consistency**: Documentation stays in sync with API specifications
- **Automation**: Generate docs as part of CI/CD pipeline
- **Customization**: Different templates for different audiences
- **Versioning**: Hash-based tracking of documentation versions

---

## 5. Business Intelligence Dashboard Configuration

### Problem
Business analysts need to create and maintain dashboard configurations for different departments, metrics, and time periods. Manual configuration of BI tools leads to inconsistencies and errors in metric calculations and visualizations.

### Solution
Generate dashboard configurations and reports from standardized business metric definitions, ensuring consistent KPI calculations across all departments.

**Data Source** (`business-metrics.yaml`):
```yaml
organization:
  name: "TechCorp Solutions"
  fiscal_year_start: "2024-01-01"
  currency: "USD"
  timezone: "America/New_York"

departments:
  sales:
    name: "Sales Department"
    manager: "Sarah Johnson"
    budget: 2500000
    targets:
      quarterly_revenue: 625000
      deals_closed: 150
      customer_acquisition: 50
  
  marketing:
    name: "Marketing Department" 
    manager: "Mike Chen"
    budget: 500000
    targets:
      leads_generated: 2000
      conversion_rate: 15.5
      campaign_roi: 4.2
  
  engineering:
    name: "Engineering Department"
    manager: "Alex Rodriguez"  
    budget: 1800000
    targets:
      sprint_velocity: 85
      bug_resolution_time: 2.5
      code_coverage: 90

metrics:
  revenue:
    name: "Monthly Recurring Revenue"
    formula: "SUM(subscription_fees) + SUM(one_time_fees)"
    target_variance: 5
    alert_threshold: 10
    
  customer_satisfaction:
    name: "Net Promoter Score"
    formula: "((promoters - detractors) / total_responses) * 100"
    target_value: 50
    measurement_frequency: "monthly"
    
  operational_efficiency:
    name: "Cost per Acquisition"
    formula: "total_marketing_spend / new_customers_acquired"
    target_value: 150
    trend: "decreasing"

dashboards:
  executive:
    title: "Executive Dashboard"
    refresh_interval: "1 hour"
    widgets:
      - type: "kpi_card"
        metric: "revenue"
        time_period: "current_month"
      - type: "trend_chart"
        metric: "customer_satisfaction"
        time_period: "last_12_months"
      - type: "department_summary"
        departments: ["sales", "marketing", "engineering"]
        
  departmental:
    title: "Department Performance"
    refresh_interval: "15 minutes"
    widgets:
      - type: "target_progress"
        metric: "department_targets"
        time_period: "current_quarter"
      - type: "team_metrics"
        department: "dynamic"
        time_period: "current_month"
```

**Template** (`grafana-dashboard.json.template`):
```jinja2
{
  "dashboard": {
    "id": null,
    "title": "{{ data.organization.name }} - Business Intelligence Dashboard",
    "description": "Generated on {{ data.env.BUILD_DATE | default(value="unknown") }}",
    "tags": [
      "business-intelligence",
      "{{ data.organization.name | kebab_case }}",
      "{{ data.env.ENVIRONMENT | default(value="production") }}"
    ],
    "timezone": "{{ data.organization.timezone }}",
    "refresh": "5m",
    "time": {
      "from": "now-30d",
      "to": "now"
    },
    "panels": [
      {
        "id": 1,
        "title": "Organization Overview",
        "type": "stat",
        "gridPos": {"h": 4, "w": 24, "x": 0, "y": 0},
        "fieldConfig": {
          "defaults": {
            "color": {"mode": "palette-classic"},
            "custom": {
              "displayMode": "list",
              "orientation": "horizontal"
            }
          }
        },
        "options": {
          "reduceOptions": {
            "values": false,
            "calcs": ["lastNotNull"],
            "fields": ""
          },
          "text": {
            "titleSize": 16,
            "valueSize": 24
          }
        },
        "targets": [
          {
            "expr": "organization_info",
            "legendFormat": "{{ data.organization.name }}",
            "refId": "A"
          }
        ]
      }
      {% for dept_name, dept_data in data.departments.items() %},
      {
        "id": {{ loop.index + 1 }},
        "title": "{{ dept_data.name }} - Performance Dashboard",
        "type": "graph",
        "gridPos": {"h": 8, "w": 12, "x": {{ (loop.index0 % 2) * 12 }}, "y": {{ 4 + (loop.index0 // 2) * 8 }}},
        "fieldConfig": {
          "defaults": {
            "color": {"mode": "palette-classic"},
            "custom": {
              "axisPlacement": "auto",
              "barAlignment": 0,
              "drawStyle": "line",
              "fillOpacity": 10,
              "gradientMode": "none",
              "hideFrom": {"tooltip": false, "vis": false, "legend": false},
              "lineInterpolation": "linear",
              "lineWidth": 2,
              "pointSize": 5,
              "scaleDistribution": {"log": 2, "type": "linear"},
              "showPoints": "never",
              "spanNulls": false,
              "stacking": {"group": "A", "mode": "none"},
              "thresholdsStyle": {"mode": "off"}
            },
            "unit": "{{ data.organization.currency | lower }}"
          }
        },
        "options": {
          "legend": {
            "calcs": [],
            "displayMode": "table",
            "placement": "bottom"
          },
          "tooltip": {"mode": "single"}
        },
        "targets": [
          {% for target_name, target_value in dept_data.targets.items() %}
          {
            "expr": "{{ dept_name | snake_case }}_{{ target_name | snake_case }}",
            "legendFormat": "{{ target_name | title | replace('_', ' ') }} (Target: {{ target_value }})",
            "refId": "{{ loop.index0 | string | upper }}"
          }{% if not loop.last %},{% endif %}
          {% endfor %}
        ],
        "alert": {
          "alertRuleTags": {
            "department": "{{ dept_name }}",
            "manager": "{{ dept_data.manager | url_encode }}",
            "budget_hash": "{{ dept_data.budget | string | md5 }}"
          },
          "conditions": [
            {
              "evaluator": {"params": [{{ dept_data.budget * 0.9 }}], "type": "lt"},
              "operator": {"type": "and"},
              "query": {"params": ["A", "5m", "now"]},
              "reducer": {"type": "avg"},
              "type": "query"
            }
          ],
          "executionErrorState": "alerting",
          "for": "5m",
          "frequency": "10s",
          "handler": 1,
          "name": "{{ dept_data.name }} - Budget Alert",
          "noDataState": "no_data",
          "notifications": []
        }
      }
      {% endfor %}
      {% for metric_name, metric_data in data.metrics.items() %},
      {
        "id": {{ loop.index + 10 }},
        "title": "{{ metric_data.name }}",
        "type": "singlestat",
        "gridPos": {"h": 6, "w": 8, "x": {{ (loop.index0 % 3) * 8 }}, "y": {{ 20 + (loop.index0 // 3) * 6 }}},
        "fieldConfig": {
          "defaults": {
            "color": {
              "mode": "thresholds"
            },
            "custom": {},
            "thresholds": {
              "mode": "absolute",
              "steps": [
                {"color": "red", "value": null},
                {"color": "yellow", "value": {{ metric_data.get('target_value', 0) * 0.8 }}},
                {"color": "green", "value": {{ metric_data.get('target_value', 0) * 0.95 }}}
              ]
            },
            "unit": "{% if 'revenue' in metric_name %}{{ data.organization.currency | lower }}{% elif 'rate' in metric_name or 'score' in metric_name %}percent{% else %}short{% endif %}"
          }
        },
        "options": {
          "colorMode": "background",
          "graphMode": "area",
          "justifyMode": "center",
          "orientation": "horizontal",
          "reduceOptions": {
            "calcs": ["lastNotNull"],
            "fields": "",
            "values": false
          },
          "text": {},
          "textMode": "auto"
        },
        "targets": [
          {
            "expr": "{{ metric_name | snake_case }}_current_value",
            "legendFormat": "Current",
            "refId": "A"
          },
          {
            "expr": "{{ metric_name | snake_case }}_target_value",
            "legendFormat": "Target",
            "refId": "B"
          }
        ],
        "transformations": [
          {
            "id": "calculateField",
            "options": {
              "alias": "Performance %",
              "binary": {
                "left": "Current",
                "operator": "/",
                "reducer": "sum",
                "right": "Target"
              },
              "mode": "binary",
              "reduce": {"include": ["Current", "Target"], "reducer": "sum"}
            }
          }
        ],
        "description": "{{ metric_data.get('formula', 'Custom business metric') }}\n\nMeasurement: {{ metric_data.get('measurement_frequency', 'daily') | title }}\nTarget: {{ metric_data.get('target_value', 'Variable') }}\nTrend: {{ metric_data.get('trend', 'stable') | title }}\n\nMetric Hash: {{ (metric_name + metric_data.name) | sha256 | truncate(length=12) }}"
      }
      {% endfor %}
    ],
    "templating": {
      "list": [
        {
          "allValue": null,
          "current": {"text": "All", "value": "$__all"},
          "datasource": "${DS_PROMETHEUS}",
          "definition": "label_values(department)",
          "hide": 0,
          "includeAll": true,
          "label": "Department",
          "multi": true,
          "name": "department",
          "options": [
            {% for dept_name in data.departments.keys() %}
            {"text": "{{ data.departments[dept_name].name }}", "value": "{{ dept_name }}"}{% if not loop.last %},{% endif %}
            {% endfor %}
          ],
          "query": "label_values(department)",
          "refresh": 1,
          "regex": "",
          "sort": 1,
          "tagValuesQuery": "",
          "tags": [],
          "tagsQuery": "",
          "type": "query",
          "useTags": false
        }
      ]
    },
    "annotations": {
      "list": [
        {
          "builtIn": 1,
          "datasource": "-- Grafana --",
          "enable": true,
          "hide": true,
          "iconColor": "rgba(0, 211, 255, 1)",
          "name": "Annotations & Alerts",
          "type": "dashboard"
        }
      ]
    },
    "editable": true,
    "gnetId": null,
    "graphTooltip": 0,
    "id": null,
    "links": [],
    "panels": [],
    "schemaVersion": 16,
    "style": "dark",
    "tags": ["business", "kpi", "{{ data.organization.name | kebab_case }}"],
    "time": {"from": "now-30d", "to": "now"},
    "timepicker": {
      "refresh_intervals": ["5s", "10s", "30s", "1m", "5m", "15m", "30m", "1h", "2h", "1d"],
      "time_options": ["5m", "15m", "1h", "6h", "12h", "24h", "2d", "7d", "30d"]
    },
    "timezone": "{{ data.organization.timezone }}",
    "title": "{{ data.organization.name }} Business Intelligence",
    "uid": "{{ data.organization.name | snake_case }}_bi_{{ data.env.DASHBOARD_VERSION | default(value="v1") }}",
    "version": {{ data.env.DASHBOARD_VERSION | default(value="1") }},
    "weekStart": ""
  }
}
```

**Usage**:
```bash
# Generate production dashboard
BUILD_DATE=$(date -Iseconds) ENVIRONMENT=production DASHBOARD_VERSION=2 \
teraclio -s business-metrics.yaml -t grafana-dashboard.json.template --env-vars -d business-dashboard.json

# Import into Grafana
curl -X POST \
  -H "Authorization: Bearer ${GRAFANA_API_KEY}" \
  -H "Content-Type: application/json" \
  -d @business-dashboard.json \
  ${GRAFANA_URL}/api/dashboards/db
```

**Benefits**:
- **Consistency**: Standardized metrics across all dashboards
- **Automation**: Dashboard updates deploy with configuration changes
- **Governance**: Version control and approval workflows for metric definitions
- **Flexibility**: Multiple dashboard templates for different audiences

---

## Summary

These use cases demonstrate Teraclio's versatility across enterprise scenarios:

1. **DevOps Automation**: Consistent infrastructure and deployment configurations
2. **Security Compliance**: Automated security reporting and audit trails
3. **Infrastructure Management**: Multi-cloud infrastructure provisioning
4. **Developer Experience**: Consistent API documentation and SDK generation
5. **Business Intelligence**: Standardized metrics and dashboard configurations

### Key Benefits Across All Use Cases:

- **Consistency**: Single source of truth eliminates configuration drift
- **Automation**: Integration with CI/CD pipelines reduces manual work
- **Security**: Built-in hashing and escaping prevent security issues
- **Auditability**: Version control and change tracking for compliance
- **Maintainability**: Template-based approach simplifies updates
- **Scalability**: Handles complex enterprise requirements efficiently

### Implementation Strategy:

1. **Start Small**: Begin with one use case and prove value
2. **Standardize**: Create organization-wide templates and conventions
3. **Automate**: Integrate into existing CI/CD and deployment pipelines
4. **Monitor**: Track usage and measure efficiency improvements
5. **Scale**: Expand to additional use cases and departments