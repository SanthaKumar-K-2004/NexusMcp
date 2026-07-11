use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

// ==================== NAVIGATION TOOLS ====================
// These structs define tool schemas (name, description, inputSchema) for MCP registration.
// Actual execution is handled by ToolRegistry::handle_* methods in mod.rs.

pub struct BrowserNavigateTool;
impl BrowserNavigateTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserNavigateTool {
    fn name(&self) -> &str { "browser_navigate" }
    fn description(&self) -> &str { "Navigate to a URL using the real headless Chromium browser engine." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "The URL to navigate to" },
                "wait_until": { "type": "string", "enum": ["load", "domcontentloaded", "networkidle0"], "default": "load" },
                "timeout": { "type": "integer", "default": 30000, "description": "Navigation timeout in ms" },
                "profile_id": { "type": "string", "description": "Optional profile ID to use for this session" }
            },
            "required": ["url"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserClickTool;
impl BrowserClickTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserClickTool {
    fn name(&self) -> &str { "browser_click" }
    fn description(&self) -> &str { "Click on an element using a CSS selector." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector of the element to click" }
            },
            "required": ["selector"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserEvaluateTool;
impl BrowserEvaluateTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserEvaluateTool {
    fn name(&self) -> &str { "browser_evaluate" }
    fn description(&self) -> &str { "Execute JavaScript in the current page via Chrome DevTools Protocol." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "script": { "type": "string", "description": "JavaScript code to execute" }
            },
            "required": ["script"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserFillFormTool;
impl BrowserFillFormTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserFillFormTool {
    fn name(&self) -> &str { "browser_fill_form" }
    fn description(&self) -> &str { "Fill multiple form fields. Keys are CSS selectors, values are text to type." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "form_data": { "type": "object", "description": "Map of CSS selector -> value to type" }
            },
            "required": ["form_data"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserBackTool;
impl BrowserBackTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserBackTool {
    fn name(&self) -> &str { "browser_back" }
    fn description(&self) -> &str { "Navigate back in browser history." }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserReloadTool;
impl BrowserReloadTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserReloadTool {
    fn name(&self) -> &str { "browser_reload" }
    fn description(&self) -> &str { "Reload the current page." }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserWaitForTool;
impl BrowserWaitForTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserWaitForTool {
    fn name(&self) -> &str { "browser_wait_for" }
    fn description(&self) -> &str { "Wait for an element, text, or a timeout condition on the live page." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector to wait for" },
                "text": { "type": "string", "description": "Text to wait for in the page body" },
                "timeout": { "type": "integer", "default": 30000, "description": "Max wait time in ms" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

// ==================== TAB MANAGEMENT TOOLS ====================

pub struct BrowserTabNewTool;
impl BrowserTabNewTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserTabNewTool {
    fn name(&self) -> &str { "browser_tab_new" }
    fn description(&self) -> &str { "Open a new browser tab, optionally navigating to a URL." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "Optional URL to open in new tab" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserTabSwitchTool;
impl BrowserTabSwitchTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserTabSwitchTool {
    fn name(&self) -> &str { "browser_tab_switch" }
    fn description(&self) -> &str { "Switch to a different tab by its ID." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "tab_id": { "type": "string", "description": "Tab ID to switch to" }
            },
            "required": ["tab_id"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserTabCloseTool;
impl BrowserTabCloseTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserTabCloseTool {
    fn name(&self) -> &str { "browser_tab_close" }
    fn description(&self) -> &str { "Close the current browser tab." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "tab_id": { "type": "string", "description": "Optional tab ID to close" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}