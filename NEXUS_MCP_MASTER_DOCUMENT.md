# NexusMCP
## The Lightest, Fastest & Most Powerful Browser MCP Server for AI Agents

**Version**: 1.0.0 (Enterprise Edition)  
**Date**: July 2026  
**License**: Apache 2.0 (Core) + Enterprise License  
**Status**: Design & Planning Document

---

## Executive Summary

**NexusMCP** is a next-generation, self-hosted Model Context Protocol (MCP) server built on **Obscura** (Rust-based headless browser engine). It delivers enterprise-grade browser automation, stealth, and extraction capabilities while remaining extremely lightweight (~30-40MB RAM per session).

### Why NexusMCP?

Current MCP browser servers suffer from one or more of these problems:
- High memory usage (Playwright MCP)
- Weak stealth capabilities
- No parallel research support
- Poor extraction quality
- Cloud-only or expensive (Browserbase)
- No enterprise features (auditing, multi-tenancy, persistence)

**NexusMCP solves all of these** by combining the best hidden gems in the ecosystem into one unified, high-performance server.

### Key Differentiators

| Feature                    | NexusMCP              | Playwright MCP     | Firecrawl MCP      | Browserbase        |
|---------------------------|-----------------------|--------------------|--------------------|--------------------|
| Memory per session        | **30-40 MB**          | 200-300 MB         | 70-100 MB          | Cloud              |
| Startup time              | **Instant**           | 1-2s               | Fast               | Fast               |
| Built-in Stealth          | **Advanced**          | Basic              | Medium             | Good               |
| Parallel Research         | **Native**            | No                 | Limited            | No                 |
| Persistent Profiles       | **Full**              | No                 | No                 | Yes                |
| Markdown Quality          | **Excellent**         | Poor               | Excellent          | Average            |
| Enterprise Features       | **Full Suite**        | Basic              | Basic              | Good               |
| Self-hosted               | **Yes**               | Yes                | Yes                | No                 |

---

## 1. Project Vision & Goals

### Vision
To become the **default browser MCP server** for autonomous AI agents in 2026-2027 by providing unmatched performance, stealth, and intelligence.

### Core Goals
1. **Extreme Performance** — Sub-100ms page loads with minimal memory
2. **Maximum Stealth** — Bypass 95%+ of bot detection systems out of the box
3. **Agent-First Design** — Tools optimized for long-running autonomous tasks
4. **Enterprise Readiness** — Security, observability, auditing, and scalability
5. **Developer Experience** — Single binary, zero dependencies, easy configuration

---

## 2. Technical Architecture (Option A - Detailed)

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        NexusMCP Server                           │
├─────────────────────────────────────────────────────────────────┤
│                    MCP Protocol Layer                            │
│              (Rust MCP SDK + JSON-RPC over stdio/HTTP)           │
├─────────────────────────────────────────────────────────────────┤
│                    Intelligence & Routing Layer                  │
│  • Tool Router          • Engine Selector       • Fallback Logic │
│  • Context Manager      • Memory Layer          • Rate Limiter   │
├─────────────────────────────────────────────────────────────────┤
│                    Core Engine Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │   Obscura    │  │  Stealth     │  │   Extraction Engine  │   │
│  │  (Rust + V8) │  │  Engine      │  │   (Markdown + JSON)  │   │
│  └──────────────┘  └──────────────┘  └──────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Persistence & State Layer                     │
│  • SQLite (Profiles)  • Redis (optional)  • Vector Memory Store  │
├─────────────────────────────────────────────────────────────────┤
│                    Observability Layer                           │
│  • OpenTelemetry    • Metrics     • Structured Logging           │
└─────────────────────────────────────────────────────────────────┘
```

### Component Breakdown

| Layer                    | Technology                     | Responsibility                              |
|--------------------------|--------------------------------|---------------------------------------------|
| MCP Protocol             | Custom Rust implementation     | Tool exposure & communication               |
| Core Browser             | **Obscura** (Rust)             | Page loading, JS execution, CDP             |
| Stealth                  | Custom + Obscura stealth       | Fingerprinting, proxies, behavior           |
| Extraction               | Custom pipeline                | Markdown, structured JSON, tables           |
| Session Management       | SQLite + in-memory             | Profiles, cookies, persistence              |
| Parallel Execution       | Tokio + async runtime          | Multi-tab research                          |
| Observability            | OpenTelemetry + Prometheus     | Metrics, tracing, health checks             |

### Data Models

#### Profile
```rust
pub struct Profile {
    pub id: String,
    pub name: String,
    pub fingerprint: FingerprintConfig,
    pub proxy: Option<ProxyConfig>,
    pub cookies: Vec<Cookie>,
    pub storage: StorageState,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}

pub struct FingerprintConfig {
    pub user_agent: String,
    pub viewport: (u32, u32),
    pub timezone: String,
    pub locale: String,
    pub hardware_concurrency: u8,
    pub device_memory: u8,
}
```

#### Session
```rust
pub struct Session {
    pub id: String,
    pub profile_id: String,
    pub tabs: Vec<Tab>,
    pub metrics: SessionMetrics,
    pub stealth_level: StealthLevel,
    pub created_at: DateTime<Utc>,
}

pub struct Tab {
    pub id: String,
    pub url: String,
    pub title: String,
    pub status: TabStatus,
}
```

#### Extraction Result
```rust
pub struct ExtractionResult {
    pub markdown: String,
    pub structured_data: serde_json::Value,
    pub links: Vec<Link>,
    pub metadata: PageMetadata,
    pub performance: ExtractionMetrics,
}
```

---

## 3. Complete Tool Reference (Option B)

### 3.1 Navigation & Interaction Tools

| Tool Name                  | Description                                      | Parameters                                      | Returns                          | Example |
|---------------------------|--------------------------------------------------|-------------------------------------------------|----------------------------------|---------|
| `browser_navigate`        | Navigate to a URL with waiting options           | `url`, `wait_until`, `timeout`, `stealth_level` | `{url, title, status}`           | `browser_navigate` with `networkidle0` |
| `browser_click`           | Click an element using semantic or selector      | `selector` OR `role` + `name`, `timeout`        | `{success, element}`             | Click login button |
| `browser_type`            | Type text into an input field                    | `selector`, `text`, `delay_ms`                  | `{success}`                      | Fill email field |
| `browser_fill_form`       | Fill multiple form fields at once                | `form_data: object`                             | `{success, filled_fields}`       | Login form |
| `browser_evaluate`        | Execute arbitrary JavaScript                     | `script: string`                                | `result`                         | Extract data via JS |
| `browser_wait_for`        | Wait for element, text, or network condition     | `selector` OR `text`, `timeout`                 | `{found}`                        | Wait for results |

### 3.2 Extraction Tools (Firecrawl-level Quality)

| Tool Name                  | Description                                      | Parameters                                      | Returns                              |
|---------------------------|--------------------------------------------------|-------------------------------------------------|--------------------------------------|
| `browser_markdown`        | Convert page to clean, LLM-optimized Markdown    | `include_images`, `max_length`                  | `{markdown, metadata}`               |
| `browser_extract`         | Structured JSON extraction using LLM guidance    | `schema: object` OR `prompt`                    | `{data: object}`                     |
| `browser_extract_table`   | Extract all tables as structured data            | —                                               | `{tables: array}`                    |
| `browser_links`           | Extract all links with context                   | `filter: string`                                | `{links: array}`                     |
| `browser_pdf`             | Generate PDF of current page                     | `format`, `margin`                              | `pdf_bytes`                          |

### 3.3 Stealth & Anti-Detection Tools

| Tool Name                  | Description                                      | Parameters                                      | Returns                              |
|---------------------------|--------------------------------------------------|-------------------------------------------------|--------------------------------------|
| `browser_stealth_rotate`  | Rotate fingerprint and user-agent                | `level: low\|medium\|high`                      | `{new_fingerprint}`                  |
| `browser_proxy_rotate`    | Switch or rotate proxy from pool                 | `proxy_url` OR `pool_name`                      | `{active_proxy}`                     |
| `browser_human_behavior`  | Enable realistic human-like interaction          | `enabled: bool`, `intensity`                    | `{status}`                           |
| `browser_ghost_mode`      | Enable maximum stealth (blocks trackers)         | —                                               | `{status}`                           |

### 3.4 Research & Parallel Tools (Unique Innovation)

| Tool Name                  | Description                                      | Parameters                                      | Returns                              |
|---------------------------|--------------------------------------------------|-------------------------------------------------|--------------------------------------|
| `browser_research`        | Open multiple URLs in parallel and extract       | `urls: array`, `extract_mode`, `concurrency`    | `{results: array}`                   |
| `browser_batch_navigate`  | Navigate many URLs concurrently                  | `urls: array`, `wait_until`                     | `{tabs: array}`                      |
| `browser_deep_research`   | Autonomous multi-page research with memory       | `query`, `max_pages`, `depth`                   | `{summary, sources}`                 |

### 3.5 Session & Persistence Tools

| Tool Name                  | Description                                      | Parameters                                      | Returns                              |
|---------------------------|--------------------------------------------------|-------------------------------------------------|--------------------------------------|
| `browser_create_profile`  | Create a persistent browser profile              | `name`, `proxy`, `stealth_level`                | `{profile_id}`                       |
| `browser_load_profile`    | Load and restore a saved profile                 | `profile_id`                                    | `{session_id}`                       |
| `browser_persist_state`   | Save current cookies and storage                 | —                                               | `{profile_id}`                       |
| `browser_memory_search`   | Semantic search across visited pages             | `query`, `limit`                                | `{results: array}`                   |

### 3.6 Enterprise & Operations Tools

| Tool Name                  | Description                                      | Parameters                                      | Returns                              |
|---------------------------|--------------------------------------------------|-------------------------------------------------|--------------------------------------|
| `browser_smart_retry`     | Retry failed action with different stealth       | `action_id`                                     | `{success}`                          |
| `browser_job_create`      | Create background automation job                 | `task`, `schedule`, `profile_id`                | `{job_id}`                           |
| `browser_get_metrics`     | Get performance and success metrics              | `time_range`                                    | `{metrics: object}`                  |
| `browser_health_check`    | System health and status                         | —                                               | `{status, details}`                  |

---

## 4. Project Structure (Option C)

```
nexusmcp/
├── Cargo.toml
├── README.md
├── LICENSE
├── docs/
│   ├── architecture.md
│   ├── tool-reference.md
│   └── deployment.md
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── mcp/
│   │   ├── server.rs
│   │   ├── tools/
│   │   │   ├── mod.rs
│   │   │   ├── navigation.rs
│   │   │   ├── extraction.rs
│   │   │   ├── stealth.rs
│   │   │   ├── research.rs
│   │   │   └── session.rs
│   │   └── router.rs
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── obscura.rs
│   │   ├── stealth.rs
│   │   └── extractor.rs
│   ├── session/
│   │   ├── manager.rs
│   │   ├── profile.rs
│   │   └── persistence.rs
│   ├── observability/
│   │   ├── metrics.rs
│   │   ├── logging.rs
│   │   └── tracing.rs
│   └── utils/
├── examples/
│   ├── claude-desktop-config.json
│   └── cursor-config.json
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
└── tests/
```

---

## 5. Enterprise Features & Security

### Security
- Sandboxed execution
- Origin allowlisting for HTTP transport
- Request body size limits
- Audit logging for all actions
- Role-based access control (future)

### Observability
- OpenTelemetry tracing
- Prometheus metrics endpoint
- Structured JSON logging
- Performance dashboards

### Scalability
- Multi-tenant support
- Configurable concurrency limits
- Background job system
- Horizontal scaling via Kubernetes

### Compliance
- GDPR-ready data handling
- Configurable data retention
- Export capabilities

---

## 6. Roadmap

### Phase 1: MVP (Weeks 1-4)
- Core navigation + extraction tools
- Stealth rotation
- Basic profile persistence
- `browser_research` tool

### Phase 2: Intelligence (Weeks 5-8)
- Advanced extraction engine
- Memory search
- Human behavior simulation
- Metrics & health checks

### Phase 3: Enterprise (Weeks 9-12)
- Job system
- Multi-tenancy
- Full observability
- Webhook support

### Phase 4: Polish (Ongoing)
- Visual dashboard
- Cloud sync option
- Marketplace for custom tools

---

## 7. Comparison & Competitive Advantage (Option D)

See the table in the Executive Summary.

**Unique Selling Points**:
- Best-in-class performance-to-feature ratio
- Native parallel research capabilities
- Deep stealth integration from day one
- True enterprise readiness in an open-source project

---

## 8. Installation & Configuration

### Quick Start

```bash
# Build from source
git clone https://github.com/your-org/nexusmcp
cd nexusmcp
cargo build --release --features enterprise

# Run
./target/release/nexusmcp mcp --stealth
```

### Claude Desktop Configuration

```json
{
  "mcpServers": {
    "nexusmcp": {
      "command": "/path/to/nexusmcp",
      "args": ["mcp", "--stealth"],
      "env": {
        "NEXUS_STEALTH_LEVEL": "high",
        "NEXUS_PROXY": "http://proxy.example.com:8080"
      }
    }
  }
}
```

---

## 9. Deployment

### Docker (Recommended)

```dockerfile
FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/nexusmcp /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/nexusmcp"]
```

### Kubernetes Ready
- Horizontal Pod Autoscaler support
- ConfigMap for profiles
- Secret management for proxies

---

## 10. Next Steps

1. Approve this master document
2. Create GitHub repository
3. Begin Phase 1 development
4. Set up CI/CD pipeline
5. Create initial MVP release

---

**Document Status**: Complete Master Specification  
**Maintained By**: NexusMCP Team  
**Last Updated**: 2026-07-10

---

*This document serves as the single source of truth for the NexusMCP project.*