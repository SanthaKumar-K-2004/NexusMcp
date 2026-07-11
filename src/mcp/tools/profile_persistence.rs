use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserLoadProfileTool;
impl BrowserLoadProfileTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserLoadProfileTool {
    fn name(&self) -> &str {
        "browser_load_profile"
    }
    fn description(&self) -> &str {
        "Load a persistent browser profile from SQLite by profile ID."
    }
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

pub async fn handle_load_profile(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let profile_id = arguments
        .get("profile_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing profile_id"))?;

    let pm = &registry.profile_manager;
    let profile_opt = pm
        .get_profile(profile_id)
        .map_err(|e| anyhow::anyhow!("Failed to query profile: {}", e))?;

    if let Some(profile) = profile_opt {
        let session_id = registry
            .session_manager
            .create_session(Some(profile.id.clone()))?;

        let response = json!({
            "success": true,
            "session_id": session_id,
            "profile": {
                "id": profile.id,
                "name": profile.name,
                "proxy": profile.proxy,
                "stealth_level": profile.stealth_level
            },
            "message": "Profile loaded from SQLite"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    } else {
        Err(anyhow::anyhow!("Profile '{}' not found", profile_id))
    }
}
