use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

/// browser_observe — Real DOM analysis of the current live page.
/// Inventories all interactive elements (forms, inputs, buttons, links).
pub struct BrowserObserveTool;
impl BrowserObserveTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserObserveTool {
    fn name(&self) -> &str { "browser_observe" }
    fn description(&self) -> &str { "Observe the current page: inventory all interactive elements (forms, inputs, buttons, links)." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "instruction": { "type": "string", "description": "What to look for (e.g. 'find login form'). Defaults to full page analysis." }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

/// browser_act — Find an element matching the goal and interact with it.
/// Uses Stagehand scoring to identify the target, then clicks or types.
pub struct BrowserActTool;
impl BrowserActTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserActTool {
    fn name(&self) -> &str { "browser_act" }
    fn description(&self) -> &str { "Act on the page: find an element matching your goal and click/type on it." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "goal": { "type": "string", "description": "What to do (e.g. 'click the submit button', 'fill the email field')" },
                "value": { "type": "string", "description": "Value to type if the target is an input field" }
            },
            "required": ["goal"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}