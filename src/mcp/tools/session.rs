use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserCreateProfileTool;
impl BrowserCreateProfileTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserCreateProfileTool {
    fn name(&self) -> &str { "browser_create_profile" }
    fn description(&self) -> &str { "Create a persistent browser profile stored in SQLite." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "name": { "type": "string", "description": "Profile name" },
                "proxy": { "type": "string", "description": "Optional proxy URL" },
                "stealth_level": { "type": "string", "enum": ["low", "medium", "high"], "default": "high" }
            },
            "required": ["name"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}