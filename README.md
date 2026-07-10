# 🌐 NexusMCP

**The lightest, fastest, and most powerful enterprise-grade browser MCP server for AI agents.**

NexusMCP is a production-ready Rust-based Model Context Protocol (MCP) server that empowers AI models (Claude, Cursor, Roo Code, etc.) with real, stealth-guided, headless browser automation capabilities.

---

## ⚡ Quick Start: One-Command Auto-Configurator

Run this single command to automatically compile the server, detect your operating system, and configure your Claude Desktop / VS Code MCP settings safely:

```bash
# Clone the repository and run the auto-configurator
git clone https://github.com/SanthaKumar-K-2004/NexusMcp.git && cd NexusMcp && python3 setup.py
```

---

## 🚀 Key Features

*   **Real Headless Browser Automation**: Drives dynamic page rendering using Chromium via Chrome DevTools Protocol (`headless_chrome`).
*   **Self-Healing Navigation**: Automatically upgrades stealth levels and retries navigation when encountering CAPTCHAs, bot challenges, or page timeouts.
*   **Stealth & Anti-Bot Footprint Bypasses**: Spoofs `navigator.webdriver`, timezone settings, language parameters, and user-agent strings on document load to slip past Cloudflare, Akamai, and other detectors.
*   **Stagehand Semantic Element Locators**: Locate buttons, inputs, and anchors using natural language queries scored dynamically on the live DOM tree.
*   **Structured Firecrawl-style Extraction**: Extract emails, prices, and DOM content matching user-supplied JSON schemas using fast, compiled regular expressions.
*   **Persistent Profiles (SQLite)**: Create and load persistent user profiles with proxy and custom stealth settings stored securely in SQLite.
*   **Media Captures**: Capture base64 PNG screenshots and print pages to base64 PDFs dynamically.

---

## 🛠️ MCP Clients Configuration

### 1. Claude Desktop & VS Code (Cline / Roo Code)
The `setup.py` script automatically configures these for you. If you need to manually modify them, add this configuration block:

```json
{
  "mcpServers": {
    "nexusmcp": {
      "command": "/absolute/path/to/nexusmcp/target/release/nexusmcp",
      "args": ["mcp", "--stealth"]
    }
  }
}
```

### 2. Cursor Desktop
*   Go to **Settings** -> **Cursor Settings** -> **Features** -> **MCP**.
*   Click **+ Add New MCP Server**.
*   Enter:
    *   **Name**: `nexusmcp`
    *   **Type**: `command`
    *   **Command**: `/absolute/path/to/nexusmcp/target/release/nexusmcp`
    *   **Arguments**: `mcp --stealth`

---

## 💻 Manual Setup

```bash
# Compile optimized release binary
cargo build --release

# Run MCP server (stdio mode)
./target/release/nexusmcp mcp --stealth

# Run as standalone HTTP server
./target/release/nexusmcp serve --port 3000 --stealth
```

---

## 📄 License

Apache 2.0