use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserHandleCaptchaTool;
impl BrowserHandleCaptchaTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserHandleCaptchaTool {
    fn name(&self) -> &str {
        "browser_handle_captcha"
    }
    fn description(&self) -> &str {
        "Detect bot protection (Cloudflare, reCAPTCHA, hCaptcha, etc.) on the current page."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["detect", "bypass"], "default": "detect",
                    "description": "detect=check for protection, bypass=inject high stealth" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserHealthCheckTool;
impl BrowserHealthCheckTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserHealthCheckTool {
    fn name(&self) -> &str {
        "browser_health_check"
    }
    fn description(&self) -> &str {
        "Check health: active sessions, browser state, registered tools."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

// ==================== HANDLER IMPLEMENTATIONS ====================

pub async fn handle_captcha(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let action = arguments
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("detect");

    let html = match registry.get_active_tab() {
        Some(tab) => ToolRegistry::html_from_tab(tab).await.unwrap_or_default(),
        None => String::new(),
    };
    let detection = registry.crawl4ai.detect_protection("", &html);

    let response = match action {
        "detect" => {
            let has_captcha = detection["detection_count"].as_u64().unwrap_or(0) > 0;
            json!({
                "success": true,
                "captcha_detected": has_captcha,
                "protection_level": detection["protection_level"],
                "detections": detection["detections"],
                "recommended_action": detection["recommended_action"],
                "message": if has_captcha { "Bot protection detected on live page" } else { "No bot protection detected" }
            })
        }
        "bypass" => {
            // Apply high stealth and retry navigation
            if let Some(tab) = registry.get_active_tab() {
                let stealth_config = registry.stealth_engine.apply_stealth("high");
                if let Some(script) = stealth_config["script"].as_str() {
                    let script_owned = script.to_string();
                    let tab_clone = tab.clone();
                    let _ = tokio::task::spawn_blocking(move || {
                        let _ = tab_clone.evaluate(&script_owned, false);
                    })
                    .await;
                }
            }
            json!({
                "success": true,
                "action": "bypass",
                "stealth_applied": "high",
                "message": "High-stealth fingerprint injected. Re-navigate to attempt bypass."
            })
        }
        _ => json!({
            "success": false,
            "message": format!("Unknown action: {}", action)
        }),
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_health_check(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let active_sessions = registry.session_manager.sessions.len();
    let browser_launched = registry.session_manager.browser.is_some();
    let has_active_tab = registry.get_active_tab().is_some();

    let response = json!({
        "success": true,
        "status": "healthy",
        "details": {
            "active_sessions": active_sessions,
            "browser_launched": browser_launched,
            "has_active_tab": has_active_tab,
            "engine": "headless_chrome (Chromium CDP)",
            "memory_entries": registry.memory.len(),
            "tools_registered": registry.tools.len()
        }
    });
    Ok(serde_json::to_string_pretty(&response)?)
}
