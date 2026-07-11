use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserSmartRetryTool;
impl BrowserSmartRetryTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserSmartRetryTool {
    fn name(&self) -> &str {
        "browser_smart_retry"
    }
    fn description(&self) -> &str {
        "Retry a failed navigation with escalating stealth levels (low → medium → high)."
    }
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

pub async fn handle_smart_retry(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let url = arguments
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing url — provide the URL to retry navigation for"))?;
    let max_retries = arguments
        .get("max_retries")
        .and_then(|v| v.as_u64())
        .unwrap_or(3);

    ToolRegistry::validate_fetch_url(url)?;

    let browser = registry.session_manager.get_or_create_browser()?;
    let stealth_levels = ["low", "medium", "high"];
    let mut last_error = String::new();

    for attempt in 0..max_retries {
        let level = stealth_levels.get(attempt as usize).unwrap_or(&"high");

        let session_id = registry.create_session(None)?;
        let session = registry
            .session_manager
            .get_session_mut(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        // Anti-Bot: Eagerly create target tab before applying stealth scripts
        if session.tab.is_none() {
            let tab = browser
                .new_tab()
                .map_err(|e| anyhow::anyhow!("Failed to create tab: {}", e))?;
            session.tab = Some(tab);
        }

        // Apply stealth for this attempt
        let stealth_config = registry.stealth_engine.apply_stealth(level);
        if let Some(script) = stealth_config["script"].as_str() {
            if let Some(tab) = &session.tab {
                let script_owned = script.to_string();
                let tab_clone = tab.clone();
                let _ = tokio::task::spawn_blocking(move || {
                    let _ = tab_clone.evaluate(&script_owned, false);
                })
                .await;
            }
        }

        match session.navigate(url, &browser).await {
            Ok(page) => {
                let html = match session.tab.clone() {
                    Some(tab) => ToolRegistry::html_from_tab(tab).await.unwrap_or_default(),
                    None => String::new(),
                };
                let detection = registry.crawl4ai.detect_protection(url, &html);
                let blocked = detection["protection_level"].as_str() == Some("high");

                if !blocked {
                    let response = json!({
                        "success": true,
                        "url": url,
                        "attempts": attempt + 1,
                        "stealth_level": level,
                        "page": {
                            "url": page.url,
                            "title": page.title,
                            "status": page.status,
                            "load_time_ms": page.load_time_ms
                        },
                        "message": format!("Navigation succeeded on attempt {} with {} stealth", attempt + 1, level)
                    });
                    return Ok(serde_json::to_string_pretty(&response)?);
                }
                last_error = "Page loaded but bot protection detected".to_string();
            }
            Err(e) => {
                last_error = e.to_string();
            }
        }

        registry.remove_session(&session_id);

        // Brief delay between retries
        tokio::time::sleep(std::time::Duration::from_millis(500 * (attempt + 1))).await;
    }

    let response = json!({
        "success": false,
        "url": url,
        "attempts": max_retries,
        "last_error": last_error,
        "message": "All retry attempts failed"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}
