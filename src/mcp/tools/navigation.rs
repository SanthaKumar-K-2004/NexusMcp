use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

// ==================== NAVIGATION TOOLS ====================
// These structs define tool schemas (name, description, inputSchema) for MCP registration.
// Actual execution is handled by functions in this file.

pub struct BrowserNavigateTool;
impl BrowserNavigateTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserNavigateTool {
    fn name(&self) -> &str {
        "browser_navigate"
    }
    fn description(&self) -> &str {
        "Navigate to a URL using the real headless Chromium browser engine."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "The URL to navigate to" },
                "wait_until": { "type": "string", "enum": ["load", "domcontentloaded", "networkidle0"], "default": "load" },
                "timeout": { "type": "integer", "default": 30000, "description": "Navigation timeout in ms" },
                "profile_id": { "type": "string", "description": "Optional profile ID to use for this session" }
            },
            "required": ["url"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserClickTool;
impl BrowserClickTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserClickTool {
    fn name(&self) -> &str {
        "browser_click"
    }
    fn description(&self) -> &str {
        "Click on an element using a CSS selector."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector of the element to click" }
            },
            "required": ["selector"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserEvaluateTool;
impl BrowserEvaluateTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserEvaluateTool {
    fn name(&self) -> &str {
        "browser_evaluate"
    }
    fn description(&self) -> &str {
        "Execute JavaScript in the current page via Chrome DevTools Protocol."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "script": { "type": "string", "description": "JavaScript code to execute" }
            },
            "required": ["script"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserFillFormTool;
impl BrowserFillFormTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserFillFormTool {
    fn name(&self) -> &str {
        "browser_fill_form"
    }
    fn description(&self) -> &str {
        "Fill multiple form fields. Keys are CSS selectors, values are text to type."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "form_data": { "type": "object", "description": "Map of CSS selector -> value to type" }
            },
            "required": ["form_data"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserBackTool;
impl BrowserBackTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserBackTool {
    fn name(&self) -> &str {
        "browser_back"
    }
    fn description(&self) -> &str {
        "Navigate back in browser history."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserReloadTool;
impl BrowserReloadTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserReloadTool {
    fn name(&self) -> &str {
        "browser_reload"
    }
    fn description(&self) -> &str {
        "Reload the current page."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserWaitForTool;
impl BrowserWaitForTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserWaitForTool {
    fn name(&self) -> &str {
        "browser_wait_for"
    }
    fn description(&self) -> &str {
        "Wait for an element, text, or a timeout condition on the live page."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector to wait for" },
                "text": { "type": "string", "description": "Text to wait for in the page body" },
                "timeout": { "type": "integer", "default": 30000, "description": "Max wait time in ms" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserTabNewTool;
impl BrowserTabNewTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserTabNewTool {
    fn name(&self) -> &str {
        "browser_tab_new"
    }
    fn description(&self) -> &str {
        "Open a new browser tab, optionally navigating to a URL."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "Optional URL to open in new tab" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserTabSwitchTool;
impl BrowserTabSwitchTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserTabSwitchTool {
    fn name(&self) -> &str {
        "browser_tab_switch"
    }
    fn description(&self) -> &str {
        "Switch to a different tab by its ID."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "tab_id": { "type": "string", "description": "Tab ID to switch to" }
            },
            "required": ["tab_id"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserTabCloseTool;
impl BrowserTabCloseTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserTabCloseTool {
    fn name(&self) -> &str {
        "browser_tab_close"
    }
    fn description(&self) -> &str {
        "Close the current browser tab."
    }
    fn input_schema(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

// ==================== HANDLER IMPLEMENTATIONS ====================

pub async fn handle_navigate(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let url = arguments
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing url"))?;

    let wait_until = arguments
        .get("wait_until")
        .and_then(|v| v.as_str())
        .unwrap_or("load");

    let profile_id = arguments.get("profile_id").and_then(|v| v.as_str());

    ToolRegistry::validate_fetch_url(url)?;

    // Detect bot protection from previous visit or empty string for first visit
    let existing_html = match registry.get_active_tab() {
        Some(tab) => ToolRegistry::html_from_tab(tab).await.unwrap_or_default(),
        None => String::new(),
    };
    let detection = registry.crawl4ai.detect_protection(url, &existing_html);
    let protection = detection["protection_level"].as_str().unwrap_or("none");
    let mut stealth_level = if protection == "high" {
        "high"
    } else {
        "medium"
    };

    // Create session with optional profile
    let session_id = if let Some(pid) = profile_id {
        registry.push_memory(format!("Loaded profile: {}", pid));
        registry.create_session(Some(pid.to_string()))?
    } else {
        registry.create_session(None)?
    };

    let browser = registry.session_manager.get_or_create_browser()?;
    let mut attempts = 0;
    let max_attempts = 2;
    let mut page_result = None;

    while attempts < max_attempts {
        attempts += 1;

        let session = registry
            .session_manager
            .get_session_mut(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        // Anti-Bot Correction: Eagerly create the tab *before* applying stealth
        // so that scripts can be injected properly before navigation
        if session.tab.is_none() {
            let tab = browser
                .new_tab()
                .map_err(|e| anyhow::anyhow!("Failed to open tab: {}", e))?;
            session.tab = Some(tab);
        }

        // Inject stealth scripts via CDP before navigation
        let stealth_config = registry.stealth_engine.apply_stealth(stealth_level);
        if let Some(script) = stealth_config["script"].as_str() {
            if let Some(tab) = &session.tab {
                let script_owned = script.to_string();
                let tab_clone = tab.clone();
                let _ = tokio::task::spawn_blocking(move || {
                    let _ = tab_clone.call_method(
                        headless_chrome::protocol::cdp::Page::AddScriptToEvaluateOnNewDocument {
                            source: script_owned.clone(),
                            world_name: None,
                            include_command_line_api: None,
                            run_immediately: None,
                        },
                    );
                    let _ = tab_clone.evaluate(&script_owned, false);
                })
                .await;
            }
        }

        match session.navigate(url, &browser).await {
            Ok(page) => {
                // Check if response page has bot protection
                let html = match session.tab.clone() {
                    Some(tab) => ToolRegistry::html_from_tab(tab).await.unwrap_or_default(),
                    None => String::new(),
                };
                let post_detection = registry.crawl4ai.detect_protection(url, &html);
                let post_protection = post_detection["protection_level"]
                    .as_str()
                    .unwrap_or("none");

                if post_protection == "high" && stealth_level != "high" {
                    tracing::warn!("Bot protection detected after navigation. Upgrading stealth and retrying...");
                    stealth_level = "high";
                    continue;
                }
                page_result = Some(page);
                break;
            }
            Err(e) => {
                if attempts >= max_attempts {
                    return Err(anyhow::anyhow!(
                        "Navigation failed after {} attempts: {}",
                        attempts,
                        e
                    ));
                }
                tracing::warn!("Navigation failed: {}. Retrying with high stealth...", e);
                stealth_level = "high";
            }
        }
    }

    let page = page_result.ok_or_else(|| anyhow::anyhow!("Navigation failed"))?;
    let current_page_count = registry
        .session_manager
        .get_session(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found after navigation"))?
        .pages
        .len();

    registry.push_memory(format!("Navigated to: {}", url));
    registry
        .vector_memory
        .store(url, &format!("Visited page: {}", page.title));

    let response = json!({
        "success": true,
        "session_id": session_id,
        "page": {
            "id": page.id,
            "url": page.url,
            "title": page.title,
            "status": page.status,
            "load_time_ms": page.load_time_ms
        },
        "wait_until": wait_until,
        "protection_level": protection,
        "stealth_level": stealth_level,
        "profile_used": profile_id,
        "current_page_count": current_page_count
    });

    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_evaluate(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let script = arguments
        .get("script")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing script"))?;

    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let script_owned = script.to_string();
    let result_val = tokio::task::spawn_blocking(move || -> Result<serde_json::Value> {
        let result_obj = tab
            .evaluate(&script_owned, false)
            .map_err(|e| anyhow::anyhow!("JS execution failed: {}", e))?;
        Ok(result_obj.value.unwrap_or(serde_json::Value::Null))
    })
    .await??;

    registry.push_memory(format!("Executed JS: {}", script));

    let response = json!({
        "success": true,
        "result": result_val,
        "script": script,
        "executed": true,
        "engine": "headless_chrome CDP"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_click(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let selector = arguments
        .get("selector")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing selector"))?;

    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let selector_owned = selector.to_string();
    tokio::task::spawn_blocking(move || -> Result<()> {
        let element = tab
            .find_element(&selector_owned)
            .map_err(|e| anyhow::anyhow!("Element '{}' not found: {}", selector_owned, e))?;
        element
            .click()
            .map_err(|e| anyhow::anyhow!("Click failed on '{}': {}", selector_owned, e))?;
        Ok(())
    })
    .await??;

    let response = json!({
        "success": true,
        "selector": selector,
        "message": "Click executed on live browser element"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_fill_form(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let form_data = arguments
        .get("form_data")
        .and_then(|v| v.as_object())
        .ok_or_else(|| anyhow::anyhow!("Missing form_data"))?;

    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let fields: Vec<(String, String)> = form_data
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let filled_count = fields.len();
    tokio::task::spawn_blocking(move || -> Result<()> {
        for (selector, value) in fields {
            let element = tab
                .find_element(&selector)
                .map_err(|e| anyhow::anyhow!("Element '{}' not found: {}", selector, e))?;
            element
                .click()
                .map_err(|e| anyhow::anyhow!("Failed to focus '{}': {}", selector, e))?;
            element
                .type_into(&value)
                .map_err(|e| anyhow::anyhow!("Failed to type into '{}': {}", selector, e))?;
        }
        Ok(())
    })
    .await??;

    let response = json!({
        "success": true,
        "fields_filled": filled_count,
        "message": "Form fields filled in live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_wait_for(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let selector = arguments
        .get("selector")
        .and_then(|v| v.as_str())
        .map(String::from);
    let text = arguments
        .get("text")
        .and_then(|v| v.as_str())
        .map(String::from);
    let timeout_ms = arguments
        .get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(30000);

    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let condition = if selector.is_some() {
        "selector"
    } else if text.is_some() {
        "text"
    } else {
        "delay"
    };

    if let Some(sel) = selector {
        let tab_clone = tab.clone();
        tokio::task::spawn_blocking(move || -> Result<()> {
            tab_clone
                .wait_for_element_with_custom_timeout(
                    &sel,
                    std::time::Duration::from_millis(timeout_ms),
                )
                .map_err(|e| anyhow::anyhow!("Timeout waiting for element '{}': {}", sel, e))?;
            Ok(())
        })
        .await??;
    } else if let Some(txt) = text {
        let start = std::time::Instant::now();
        // Security: Escape user inputs to prevent injection into browser JS context
        let escaped_txt = serde_json::to_string(&txt).unwrap_or_else(|_| "null".to_string());
        let script = format!("document.body.innerText.includes({})", escaped_txt);
        loop {
            let tab_clone = tab.clone();
            let script_clone = script.clone();
            let res = tokio::task::spawn_blocking(move || tab_clone.evaluate(&script_clone, false))
                .await??;
            if res.value.and_then(|v| v.as_bool()).unwrap_or(false) {
                break;
            }
            if start.elapsed().as_millis() > timeout_ms as u128 {
                return Err(anyhow::anyhow!("Timeout waiting for text '{}'", txt));
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    } else {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    let response = json!({
        "success": true,
        "condition": condition,
        "timeout_ms": timeout_ms,
        "message": "Wait condition satisfied on live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_back(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let session_id = registry
        .get_active_session_id()
        .ok_or_else(|| anyhow::anyhow!("No active session"))?;
    let session = registry
        .session_manager
        .get_session_mut(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let page = session.go_back().await?;

    let response = json!({
        "success": true,
        "page": {
            "url": page.url,
            "title": page.title,
            "status": page.status
        },
        "message": "Navigated back in browser history"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_reload(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let session_id = registry
        .get_active_session_id()
        .ok_or_else(|| anyhow::anyhow!("No active session"))?;
    let session = registry
        .session_manager
        .get_session_mut(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let page = session.reload().await?;

    let response = json!({
        "success": true,
        "page": {
            "url": page.url,
            "title": page.title,
            "status": page.status
        },
        "message": "Page reloaded in live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_tab_new(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let url = arguments.get("url").and_then(|v| v.as_str());
    if let Some(url) = url {
        ToolRegistry::validate_fetch_url(url)?;
    }
    let browser = registry.session_manager.get_or_create_browser()?;

    // Use existing session or create one
    let session_id = if let Some(sid) = registry.get_active_session_id() {
        sid
    } else {
        registry.create_session(None)?
    };

    let session = registry
        .session_manager
        .get_session_mut(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let page = session.new_tab(url, &browser).await?;

    let response = json!({
        "success": true,
        "tab_id": page.id,
        "url": page.url,
        "title": page.title,
        "message": "New tab opened in live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_tab_switch(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let tab_id = arguments
        .get("tab_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing tab_id"))?;

    let session_id = registry
        .get_active_session_id()
        .ok_or_else(|| anyhow::anyhow!("No active session"))?;
    let session = registry
        .session_manager
        .get_session_mut(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    session.switch_tab(tab_id)?;
    registry.set_active_session(session_id);

    let response = json!({
        "success": true,
        "tab_id": tab_id,
        "message": "Switched to tab"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_tab_close(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let session_id = registry
        .get_active_session_id()
        .ok_or_else(|| anyhow::anyhow!("No active session"))?;
    let session = registry
        .session_manager
        .get_session_mut(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    session.close_current_tab().await?;
    let remove_empty_session = session.tab.is_none();
    if remove_empty_session {
        registry.remove_session(&session_id);
    }

    let response = json!({
        "success": true,
        "message": "Tab closed in live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}
