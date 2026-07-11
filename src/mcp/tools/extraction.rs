use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

// ==================== EXTRACTION TOOLS ====================
// Schema-only definitions. Actual execution in ToolRegistry::handle_* methods.

pub struct BrowserMarkdownTool;
impl BrowserMarkdownTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserMarkdownTool {
    fn name(&self) -> &str { "browser_markdown" }
    fn description(&self) -> &str { "Extract clean Markdown from the current live page." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "include_images": { "type": "boolean", "default": false },
                "max_length": { "type": "integer", "default": 8000 }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserExtractTool;
impl BrowserExtractTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserExtractTool {
    fn name(&self) -> &str { "browser_extract" }
    fn description(&self) -> &str { "Extract structured data from the current page using CSS selectors or a schema." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "schema": { "type": "object", "description": "Schema defining what fields to extract (e.g. title, emails, prices)" },
                "prompt": { "type": "string" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserLinksTool;
impl BrowserLinksTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserLinksTool {
    fn name(&self) -> &str { "browser_links" }
    fn description(&self) -> &str { "Extract all links from the current live page." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "filter": { "type": "string", "description": "Optional filter for link URLs or text" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserPdfTool;
impl BrowserPdfTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserPdfTool {
    fn name(&self) -> &str { "browser_pdf" }
    fn description(&self) -> &str { "Generate a PDF of the current page from the live browser." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "format": { "type": "string", "default": "A4" },
                "margin": { "type": "integer", "default": 20 }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserScreenshotTool;
impl BrowserScreenshotTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserScreenshotTool {
    fn name(&self) -> &str { "browser_screenshot" }
    fn description(&self) -> &str { "Take a PNG screenshot of the current page from the live browser." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "Optional CSS selector to screenshot a specific element" },
                "full_page": { "type": "boolean", "default": true }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}