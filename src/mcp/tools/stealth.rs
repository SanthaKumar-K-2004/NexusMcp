use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserStealthRotateTool;
impl BrowserStealthRotateTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserStealthRotateTool {
    fn name(&self) -> &str {
        "browser_stealth_rotate"
    }
    fn description(&self) -> &str {
        "Rotate browser fingerprint (user-agent, WebGL, plugins) on the active tab using CDP."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "level": {
                    "type": "string",
                    "enum": ["low", "medium", "high"],
                    "default": "high",
                    "description": "Stealth level: low=webdriver only, medium=UA+languages, high=full fingerprint"
                }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub async fn handle_stealth_rotate(
    registry: &mut ToolRegistry,
    arguments: Value,
) -> Result<String> {
    let level = arguments
        .get("level")
        .and_then(|v| v.as_str())
        .unwrap_or("high");

    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let stealth_config = registry.stealth_engine.apply_stealth(level);
    let script = stealth_config["script"].as_str().unwrap_or("").to_string();
    let user_agent = stealth_config["user_agent"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let tab_clone = tab.clone();
    let script_clone = script.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        tab_clone.call_method(
            headless_chrome::protocol::cdp::Page::AddScriptToEvaluateOnNewDocument {
                source: script_clone.clone(),
                world_name: None,
                include_command_line_api: None,
                run_immediately: None,
            },
        )?;
        tab_clone.evaluate(&script_clone, false)?;
        Ok(())
    })
    .await??;

    let response = json!({
        "success": true,
        "level": level,
        "new_user_agent": user_agent,
        "techniques_applied": stealth_config["techniques_applied"],
        "message": "Stealth fingerprint rotated on live browser tab"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}
