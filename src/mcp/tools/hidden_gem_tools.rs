use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

/// Stagehand-style element targeting using natural language
pub struct BrowserFindElementTool;
impl BrowserFindElementTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserFindElementTool {
    fn name(&self) -> &str { "browser_find_element" }
    fn description(&self) -> &str { "Find elements using natural language. Returns CSS selectors ranked by confidence." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "instruction": { "type": "string", "description": "Natural language description of the element (e.g. 'the search input')" }
            },
            "required": ["instruction"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

/// Trafilatura-style article extraction
pub struct BrowserTrafilaturaTool;
impl BrowserTrafilaturaTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserTrafilaturaTool {
    fn name(&self) -> &str { "browser_trafilatura" }
    fn description(&self) -> &str { "Extract article content from the current page, stripping navigation/ads/boilerplate." }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

/// Firecrawl-style structured extraction with schema
pub struct BrowserFirecrawlExtractTool;
impl BrowserFirecrawlExtractTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserFirecrawlExtractTool {
    fn name(&self) -> &str { "browser_firecrawl_extract" }
    fn description(&self) -> &str { "Extract structured data using a field schema (title, emails, prices, links_count, or CSS selectors)." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "schema": { "type": "object", "description": "Schema defining fields to extract" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}