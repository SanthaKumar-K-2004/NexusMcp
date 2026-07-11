use super::{Tool, ToolRegistry};
use serde_json::{json, Value};
use anyhow::Result;

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

// ==================== HANDLER IMPLEMENTATIONS ====================

pub async fn handle_find_element(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let instruction = arguments.get("instruction").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing instruction"))?;

    let html = registry.get_active_html()?;
    let res = registry.stagehand.find_element(instruction, &html);
    Ok(serde_json::to_string_pretty(&res)?)
}

pub async fn handle_trafilatura(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let html = registry.get_active_html()?;
    let session_id = registry.get_active_session_id().unwrap_or_default();
    let url = registry.session_manager.get_session(&session_id)
        .and_then(|s| s.current_page_state().map(|p| p.url.clone()))
        .unwrap_or_else(|| "unknown".to_string());

    let extraction = registry.trafilatura.extract_content(&html, &url);
    Ok(serde_json::to_string_pretty(&extraction)?)
}