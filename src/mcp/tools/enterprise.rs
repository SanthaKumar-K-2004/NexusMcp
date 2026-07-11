use super::Tool;
use anyhow::Result;
use serde_json::{json, Value};

/// CAPTCHA and anti-bot detection using real HTML analysis (Crawl4AI-based)
pub struct BrowserHandleCaptchaTool;
impl BrowserHandleCaptchaTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserHandleCaptchaTool {
    fn name(&self) -> &str { "browser_handle_captcha" }
    fn description(&self) -> &str { "Detect bot protection (Cloudflare, reCAPTCHA, hCaptcha, etc.) on the current page." }
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

/// Health check returning real browser engine state
pub struct BrowserHealthCheckTool;
impl BrowserHealthCheckTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserHealthCheckTool {
    fn name(&self) -> &str { "browser_health_check" }
    fn description(&self) -> &str { "Check health: active sessions, browser state, registered tools." }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}