use super::{Tool, ToolRegistry};
use serde_json::{json, Value};
use anyhow::Result;

pub struct BrowserObserveTool;
impl BrowserObserveTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserObserveTool {
    fn name(&self) -> &str { "browser_observe" }
    fn description(&self) -> &str { "Observe the current page: inventory all interactive elements (forms, inputs, buttons, links)." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "instruction": { "type": "string", "description": "What to look for (e.g. 'find login form'). Defaults to full page analysis." }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserActTool;
impl BrowserActTool { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl Tool for BrowserActTool {
    fn name(&self) -> &str { "browser_act" }
    fn description(&self) -> &str { "Act on the page: find an element matching your goal and click/type on it." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "goal": { "type": "string", "description": "What to do (e.g. 'click the submit button', 'fill the email field')" },
                "value": { "type": "string", "description": "Value to type if the target is an input field" }
            },
            "required": ["goal"]
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

// ==================== HANDLER IMPLEMENTATIONS ====================

pub async fn handle_observe(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let instruction = arguments.get("instruction").and_then(|v| v.as_str())
        .unwrap_or("analyze page");

    let html = registry.get_active_html()?;
    let document = scraper::Html::parse_document(&html);

    // Collect all interactive elements on the page
    let interactive_sel = scraper::Selector::parse(
        "input, button, a, textarea, select, [role='button'], [role='textbox'], [role='link'], form"
    ).unwrap();

    let mut observations: Vec<Value> = Vec::new();
    let mut forms_count = 0;
    let mut inputs_count = 0;
    let mut buttons_count = 0;
    let mut links_count = 0;

    for element in document.select(&interactive_sel) {
        let tag = element.value().name();
        let id = element.value().attr("id").unwrap_or("");
        let text = element.text().collect::<String>().trim().to_string();
        let type_attr = element.value().attr("type").unwrap_or("");
        let placeholder = element.value().attr("placeholder").unwrap_or("");

        match tag {
            "form" => {
                forms_count += 1;
                let action = element.value().attr("action").unwrap_or("");
                observations.push(json!({
                    "type": "form",
                    "id": id,
                    "action": action
                }));
            }
            "input" | "textarea" | "select" => {
                inputs_count += 1;
                observations.push(json!({
                    "type": "input",
                    "tag": tag,
                    "id": id,
                    "input_type": type_attr,
                    "placeholder": placeholder
                }));
            }
            "button" => {
                buttons_count += 1;
                observations.push(json!({
                    "type": "button",
                    "id": id,
                    "text": if text.len() > 50 { &text[..50] } else { &text }
                }));
            }
            "a" => {
                links_count += 1;
                let href = element.value().attr("href").unwrap_or("");
                if links_count <= 20 { // Cap link observations
                    observations.push(json!({
                        "type": "link",
                        "text": if text.len() > 50 { &text[..50] } else { &text },
                        "href": href
                    }));
                }
            }
            _ => {}
        }
    }

    let stagehand_result = if instruction != "analyze page" {
        Some(registry.stagehand.find_element(instruction, &html))
    } else {
        None
    };

    let protection = registry.crawl4ai.detect_protection("", &html);

    let response = json!({
        "success": true,
        "instruction": instruction,
        "page_summary": {
            "forms": forms_count,
            "inputs": inputs_count,
            "buttons": buttons_count,
            "links": links_count,
            "total_interactive": observations.len()
        },
        "observations": observations,
        "stagehand_match": stagehand_result,
        "protection_detected": protection["protection_level"],
        "message": "Real DOM analysis of live browser page"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_act(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let goal = arguments.get("goal").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing goal"))?;

    let html = registry.get_active_html()?;
    let tab = registry.get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    // Use stagehand to find the target element for the goal
    let stagehand_result = registry.stagehand.find_element(goal, &html);
    let target_selector = stagehand_result["element"]["selector"].as_str()
        .unwrap_or("body");
    let confidence = stagehand_result["element"]["confidence"].as_f64().unwrap_or(0.0);

    if confidence < 0.3 {
        let response = json!({
            "success": false,
            "goal": goal,
            "reason": "No high-confidence element found for this goal",
            "stagehand_result": stagehand_result,
            "message": "Could not determine which element to act on"
        });
        return Ok(serde_json::to_string_pretty(&response)?);
    }

    let role = stagehand_result["element"]["role"].as_str().unwrap_or("generic");
    let action_taken;

    match role {
        "button" | "link" => {
            let tab_clone = tab.clone();
            let sel = target_selector.to_string();
            tokio::task::spawn_blocking(move || -> Result<()> {
                let element = tab_clone.find_element(&sel)
                    .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                element.click()
                    .map_err(|e| anyhow::anyhow!("Click failed: {}", e))?;
                Ok(())
            }).await??;
            action_taken = "click";
        }
        "textbox" => {
            let text_to_type = if let Some(val) = arguments.get("value").and_then(|v| v.as_str()) {
                val.to_string()
            } else {
                String::new()
            };

            if !text_to_type.is_empty() {
                let tab_clone = tab.clone();
                let sel = target_selector.to_string();
                tokio::task::spawn_blocking(move || -> Result<()> {
                    let element = tab_clone.find_element(&sel)
                        .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                    element.click()
                        .map_err(|e| anyhow::anyhow!("Focus failed: {}", e))?;
                    element.type_into(&text_to_type)
                        .map_err(|e| anyhow::anyhow!("Type failed: {}", e))?;
                    Ok(())
                }).await??;
                action_taken = "type";
            } else {
                let tab_clone = tab.clone();
                let sel = target_selector.to_string();
                tokio::task::spawn_blocking(move || -> Result<()> {
                    let element = tab_clone.find_element(&sel)
                        .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                    element.click()
                        .map_err(|e| anyhow::anyhow!("Focus failed: {}", e))?;
                    Ok(())
                }).await??;
                action_taken = "focus";
            }
        }
        _ => {
            let tab_clone = tab.clone();
            let sel = target_selector.to_string();
            tokio::task::spawn_blocking(move || -> Result<()> {
                let element = tab_clone.find_element(&sel)
                    .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                element.click()
                    .map_err(|e| anyhow::anyhow!("Click failed: {}", e))?;
                Ok(())
            }).await??;
            action_taken = "click";
        }
    }

    let response = json!({
        "success": true,
        "goal": goal,
        "action": action_taken,
        "target_selector": target_selector,
        "target_role": role,
        "confidence": confidence,
        "stagehand_result": stagehand_result,
        "message": format!("Action '{}' performed on '{}' in live browser", action_taken, target_selector)
    });
    Ok(serde_json::to_string_pretty(&response)?)
}