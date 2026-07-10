# NexusMCP

**The lightest, fastest, and most powerful browser MCP server for AI agents.**

Built on Rust with plans to integrate **Obscura** as the core engine.

## Features (Current MVP)

- Full MCP protocol support (stdio)
- 7 working tools:
  - `browser_navigate`
  - `browser_click`
  - `browser_markdown`
  - `browser_stealth_rotate`
  - `browser_research` (parallel)
  - `browser_create_profile`
- Stealth mode support
- Proxy support (planned)
- Extremely lightweight architecture

## Quick Start

```bash
# Build
cargo build --release

# Run MCP server
./target/release/nexusmcp mcp --stealth
```

## Claude Desktop Configuration

Add this to your Claude config:

```json
{
  "mcpServers": {
    "nexusmcp": {
      "command": "/path/to/nexusmcp/target/release/nexusmcp",
      "args": ["mcp", "--stealth"]
    }
  }
}
```

## Roadmap

- [ ] Real Obscura integration
- [ ] More extraction tools
- [ ] Persistent profiles
- [ ] HTTP server mode
- [ ] Enterprise features

## License

Apache 2.0