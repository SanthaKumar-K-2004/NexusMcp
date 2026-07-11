use super::{Tool, ToolRegistry};
use serde_json::{json, Value};
use anyhow::Result;

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

pub async fn handle_create_profile(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let name = arguments.get("name").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing profile name"))?;
    let proxy = arguments.get("proxy").and_then(|v| v.as_str());
    let stealth_level = arguments.get("stealth_level").and_then(|v| v.as_str()).unwrap_or("high");

    let pm = &registry.profile_manager;
    let profile = pm.create_profile(name, proxy, stealth_level)
        .map_err(|e| anyhow::anyhow!("Failed to create profile: {}", e))?;

    let response = json!({
        "success": true,
        "profile_id": profile.id,
        "name": profile.name,
        "proxy": profile.proxy,
        "stealth_level": profile.stealth_level,
        "message": "Profile created and persisted in SQLite"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}