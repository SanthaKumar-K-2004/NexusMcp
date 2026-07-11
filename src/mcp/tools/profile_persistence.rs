use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserLoadProfileTool;
impl BrowserLoadProfileTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserLoadProfileTool {
    fn name(&self) -> &str { "browser_load_profile" }
    fn description(&self) -> &str { "Load a persistent browser profile from SQLite by profile ID." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "profile_id": { "type": "string", "description": "The profile ID to load" }
            },
            "required": ["profile_id"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}