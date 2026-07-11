use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserSmartRetryTool;
impl BrowserSmartRetryTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserSmartRetryTool {
    fn name(&self) -> &str { "browser_smart_retry" }
    fn description(&self) -> &str { "Retry a failed navigation with escalating stealth levels (low → medium → high)." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "URL to retry navigating to" },
                "max_retries": { "type": "integer", "default": 3, "description": "Maximum retry attempts" }
            },
            "required": ["url"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}