use serde_json::{json, Value};
use std::collections::HashMap;
use anyhow::Result;
use std::sync::Arc;
use crate::engine::SessionManager;
use crate::extraction::AdvancedExtractor;
use crate::agent::AgentEnhancer;
use crate::hidden_gems::{StagehandEngine, Crawl4AIDetector, TrafilaturaExtractor, PlaywrightStealth, VectorMemory, FirecrawlExtractor};

pub mod navigation;
pub mod extraction;
pub mod stealth;
pub mod research;
pub mod session;
pub mod error_recovery;
pub mod profile_persistence;
pub mod agent_superpowers;
pub mod hidden_gem_tools;
pub mod enterprise;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
    pub session_manager: SessionManager,
    pub extractor: AdvancedExtractor,
    pub agent: AgentEnhancer,
    pub memory: Vec<String>,

    // Hidden Gems Integration
    pub stagehand: StagehandEngine,
    pub crawl4ai: Crawl4AIDetector,
    pub trafilatura: TrafilaturaExtractor,
    pub stealth_engine: PlaywrightStealth,
    pub vector_memory: VectorMemory,
    pub firecrawl: FirecrawlExtractor,
}

#[async_trait::async_trait]
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;

    async fn call(&self, arguments: Value) -> Result<String>;
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            session_manager: SessionManager::new(),
            extractor: AdvancedExtractor::new(),
            agent: AgentEnhancer::new(),
            memory: Vec::new(),

            // Hidden Gems
            stagehand: StagehandEngine::new(),
            crawl4ai: Crawl4AIDetector::new(),
            trafilatura: TrafilaturaExtractor::new(),
            stealth_engine: PlaywrightStealth::new(),
            vector_memory: VectorMemory::new(None),
            firecrawl: FirecrawlExtractor::new(),
        }
    }

    pub fn register_tools(&mut self, stealth_enabled: bool, _proxy: Option<String>) {
        // Navigation tools
        self.register(Box::new(navigation::BrowserNavigateTool::new()));
        self.register(Box::new(navigation::BrowserClickTool::new()));
        self.register(Box::new(navigation::BrowserEvaluateTool::new()));
        self.register(Box::new(navigation::BrowserFillFormTool::new()));
        self.register(Box::new(navigation::BrowserBackTool::new()));
        self.register(Box::new(navigation::BrowserReloadTool::new()));
        self.register(Box::new(navigation::BrowserWaitForTool::new()));

        // Tab management tools
        self.register(Box::new(navigation::BrowserTabNewTool::new()));
        self.register(Box::new(navigation::BrowserTabSwitchTool::new()));
        self.register(Box::new(navigation::BrowserTabCloseTool::new()));

        // Extraction tools
        self.register(Box::new(extraction::BrowserMarkdownTool::new()));
        self.register(Box::new(extraction::BrowserExtractTool::new()));
        self.register(Box::new(extraction::BrowserLinksTool::new()));
        self.register(Box::new(extraction::BrowserPdfTool::new()));
        self.register(Box::new(extraction::BrowserScreenshotTool::new()));

        // Stealth tools
        if stealth_enabled {
            self.register(Box::new(stealth::BrowserStealthRotateTool::new()));
        }

        // Research tools
        self.register(Box::new(research::BrowserResearchTool::new()));

        // Session tools
        self.register(Box::new(session::BrowserCreateProfileTool::new()));

        // Error Recovery
        self.register(Box::new(error_recovery::BrowserSmartRetryTool::new()));

        // Profile Persistence
        self.register(Box::new(profile_persistence::BrowserLoadProfileTool::new()));

        // AI Agent tools
        self.register(Box::new(agent_superpowers::BrowserObserveTool::new()));
        self.register(Box::new(agent_superpowers::BrowserActTool::new()));

        // Hidden Gems Tools
        self.register(Box::new(hidden_gem_tools::BrowserFindElementTool::new()));
        self.register(Box::new(hidden_gem_tools::BrowserTrafilaturaTool::new()));
        self.register(Box::new(hidden_gem_tools::BrowserFirecrawlExtractTool::new()));

        // Enterprise Tools
        self.register(Box::new(enterprise::BrowserHandleCaptchaTool::new()));
        self.register(Box::new(enterprise::BrowserHealthCheckTool::new()));

        tracing::info!("Registered {} tools (all wired to real browser engine)", self.tools.len());
    }

    fn register(&mut self, tool: Box<dyn Tool + Send + Sync>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    pub fn list_tools(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect()
    }

    // ==================== HELPER: Get Active Tab ====================

    /// Get a clone of the active browser tab from any session.
    /// Returns None if no session or tab is active.
    fn get_active_tab(&self) -> Option<Arc<headless_chrome::Tab>> {
        self.session_manager.sessions.values()
            .find_map(|session| session.tab.clone())
    }

    /// Get the session ID of the first active session.
    fn get_active_session_id(&self) -> Option<String> {
        self.session_manager.sessions.keys().next().cloned()
    }

    /// Get HTML from the active browser tab.
    fn get_active_html(&self) -> Result<String> {
        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        session.get_current_html()
            .ok_or_else(|| anyhow::anyhow!("No page loaded — navigate to a URL first"))
    }

    // ==================== DISPATCH ====================

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        match name {
            // Navigation
            "browser_navigate" => self.handle_navigate(arguments).await,
            "browser_evaluate" => self.handle_evaluate(arguments).await,
            "browser_click" => self.handle_click(arguments).await,
            "browser_fill_form" => self.handle_fill_form(arguments).await,
            "browser_wait_for" => self.handle_wait_for(arguments).await,
            "browser_back" => self.handle_back(arguments).await,
            "browser_reload" => self.handle_reload(arguments).await,

            // Tab management
            "browser_tab_new" => self.handle_tab_new(arguments).await,
            "browser_tab_switch" => self.handle_tab_switch(arguments).await,
            "browser_tab_close" => self.handle_tab_close(arguments).await,

            // Extraction
            "browser_markdown" => self.handle_markdown(arguments).await,
            "browser_screenshot" => self.handle_screenshot(arguments).await,
            "browser_pdf" => self.handle_pdf(arguments).await,
            "browser_links" => self.handle_links(arguments).await,
            "browser_extract" | "browser_firecrawl_extract" => self.handle_extract(arguments).await,

            // Hidden gems
            "browser_find_element" => self.handle_find_element(arguments).await,
            "browser_trafilatura" => self.handle_trafilatura(arguments).await,

            // Stealth
            "browser_stealth_rotate" => self.handle_stealth_rotate(arguments).await,

            // Profiles
            "browser_create_profile" => self.handle_create_profile(arguments).await,
            "browser_load_profile" => self.handle_load_profile(arguments).await,

            // Agent superpowers
            "browser_observe" => self.handle_observe(arguments).await,
            "browser_act" => self.handle_act(arguments).await,

            // Enterprise
            "browser_handle_captcha" => self.handle_captcha(arguments).await,
            "browser_health_check" => self.handle_health_check(arguments).await,
            "browser_smart_retry" => self.handle_smart_retry(arguments).await,

            // Research (uses reqwest, not browser)
            "browser_research" => {
                if let Some(tool) = self.tools.get(name) {
                    tool.call(arguments).await
                } else {
                    Err(anyhow::anyhow!("Tool '{}' not found", name))
                }
            }

            _ => Err(anyhow::anyhow!("Tool '{}' not found", name)),
        }
    }

    // ==================== REAL IMPLEMENTATIONS ====================

    async fn handle_navigate(&mut self, arguments: Value) -> Result<String> {
        let url = arguments
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing url"))?;

        let wait_until = arguments
            .get("wait_until")
            .and_then(|v| v.as_str())
            .unwrap_or("load");

        let profile_id = arguments.get("profile_id").and_then(|v| v.as_str());

        // Detect bot protection from previous visit or empty string for first visit
        let existing_html = self.get_active_html().unwrap_or_default();
        let detection = self.crawl4ai.detect_protection(url, &existing_html);
        let protection = detection["protection_level"].as_str().unwrap_or("none");
        let mut stealth_level = if protection == "high" { "high" } else { "medium" };

        // Create session with optional profile
        let session_id = if let Some(pid) = profile_id {
            self.memory.push(format!("Loaded profile: {}", pid));
            self.session_manager.create_session(Some(pid.to_string()))?
        } else {
            self.session_manager.create_session(None)?
        };

        let browser = self.session_manager.get_or_create_browser()?;
        let mut attempts = 0;
        let max_attempts = 2;
        let mut page_result = None;

        while attempts < max_attempts {
            attempts += 1;
            let session = self.session_manager.get_session_mut(&session_id)
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

            // Inject stealth scripts via CDP before navigation
            let stealth_config = self.stealth_engine.apply_stealth(stealth_level);
            if let Some(script) = stealth_config["script"].as_str() {
                if let Some(tab) = &session.tab {
                    let script_owned = script.to_string();
                    let _ = tokio::task::block_in_place(|| {
                        let _ = tab.call_method(headless_chrome::protocol::cdp::Page::AddScriptToEvaluateOnNewDocument {
                            source: script_owned.clone(),
                            world_name: None,
                            include_command_line_api: None,
                            run_immediately: None,
                        });
                        let _ = tab.evaluate(&script_owned, false);
                    });
                }
            }

            match session.navigate(url, &browser).await {
                Ok(page) => {
                    // Check if response page has bot protection
                    let html = session.get_current_html().unwrap_or_default();
                    let post_detection = self.crawl4ai.detect_protection(url, &html);
                    let post_protection = post_detection["protection_level"].as_str().unwrap_or("none");

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
                        return Err(anyhow::anyhow!("Navigation failed after {} attempts: {}", attempts, e));
                    }
                    tracing::warn!("Navigation failed: {}. Retrying with high stealth...", e);
                    stealth_level = "high";
                }
            }
        }

        let page = page_result.ok_or_else(|| anyhow::anyhow!("Navigation failed"))?;
        let session = self.session_manager.get_session(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found after navigation"))?;

        self.memory.push(format!("Navigated to: {}", url));
        self.vector_memory.store(url, &format!("Visited page: {}", page.title));

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
            "current_page_count": session.pages.len()
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_evaluate(&mut self, arguments: Value) -> Result<String> {
        let script = arguments
            .get("script")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing script"))?;

        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let result_val = tokio::task::block_in_place(|| -> Result<serde_json::Value> {
            let result_obj = tab.evaluate(script, false)
                .map_err(|e| anyhow::anyhow!("JS execution failed: {}", e))?;
            Ok(result_obj.value.unwrap_or(serde_json::Value::Null))
        })?;

        self.memory.push(format!("Executed JS: {}", script));

        let response = json!({
            "success": true,
            "result": result_val,
            "script": script,
            "executed": true,
            "engine": "headless_chrome CDP"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_markdown(&mut self, _arguments: Value) -> Result<String> {
        let html = self.get_active_html()?;
        let session_id = self.get_active_session_id().unwrap_or_default();
        let session = self.session_manager.get_session(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        let url = session.current_page_state().map(|p| p.url.clone()).unwrap_or_default();
        let title = session.current_page_state().map(|p| p.title.clone()).unwrap_or_default();

        let markdown = self.extractor.html_to_markdown(&html, &url)?;
        self.vector_memory.store(&url, &markdown);
        self.memory.push("Extracted Markdown from live page".to_string());

        let response = json!({
            "success": true,
            "markdown": markdown,
            "metadata": {
                "title": title,
                "url": url,
                "word_count": markdown.split_whitespace().count(),
                "extraction_method": "html2md from live browser content"
            }
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_click(&mut self, arguments: Value) -> Result<String> {
        let selector = arguments.get("selector").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing selector"))?;

        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        tokio::task::block_in_place(|| -> Result<()> {
            let element = tab.find_element(selector)
                .map_err(|e| anyhow::anyhow!("Element '{}' not found: {}", selector, e))?;
            element.click()
                .map_err(|e| anyhow::anyhow!("Click failed on '{}': {}", selector, e))?;
            Ok(())
        })?;

        let response = json!({
            "success": true,
            "selector": selector,
            "message": "Click executed on live browser element"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_fill_form(&mut self, arguments: Value) -> Result<String> {
        let form_data = arguments.get("form_data").and_then(|v| v.as_object())
            .ok_or_else(|| anyhow::anyhow!("Missing form_data"))?;

        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let fields: Vec<(String, String)> = form_data
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect();

        let filled_count = fields.len();
        tokio::task::block_in_place(|| -> Result<()> {
            for (selector, value) in fields {
                let element = tab.find_element(&selector)
                    .map_err(|e| anyhow::anyhow!("Element '{}' not found: {}", selector, e))?;
                element.click()
                    .map_err(|e| anyhow::anyhow!("Failed to focus '{}': {}", selector, e))?;
                element.type_into(&value)
                    .map_err(|e| anyhow::anyhow!("Failed to type into '{}': {}", selector, e))?;
            }
            Ok(())
        })?;

        let response = json!({
            "success": true,
            "fields_filled": filled_count,
            "message": "Form fields filled in live browser"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_wait_for(&mut self, arguments: Value) -> Result<String> {
        let selector = arguments.get("selector").and_then(|v| v.as_str()).map(String::from);
        let text = arguments.get("text").and_then(|v| v.as_str()).map(String::from);
        let timeout_ms = arguments.get("timeout").and_then(|v| v.as_u64()).unwrap_or(30000);

        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let condition = if selector.is_some() { "selector" } else if text.is_some() { "text" } else { "delay" };

        tokio::task::block_in_place(|| -> Result<()> {
            if let Some(sel) = selector {
                tab.wait_for_element_with_custom_timeout(&sel, std::time::Duration::from_millis(timeout_ms))
                    .map_err(|e| anyhow::anyhow!("Timeout waiting for element '{}': {}", sel, e))?;
            } else if let Some(txt) = text {
                let start = std::time::Instant::now();
                loop {
                    let res = tab.evaluate(&format!("document.body.innerText.includes('{}')", txt), false)?;
                    if res.value.and_then(|v| v.as_bool()).unwrap_or(false) {
                        break;
                    }
                    if start.elapsed().as_millis() > timeout_ms as u128 {
                        return Err(anyhow::anyhow!("Timeout waiting for text '{}'", txt));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
            Ok(())
        })?;

        let response = json!({
            "success": true,
            "condition": condition,
            "timeout_ms": timeout_ms,
            "message": "Wait condition satisfied on live browser"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_back(&mut self, _arguments: Value) -> Result<String> {
        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session_mut(&session_id)
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

    async fn handle_reload(&mut self, _arguments: Value) -> Result<String> {
        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session_mut(&session_id)
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

    async fn handle_tab_new(&mut self, arguments: Value) -> Result<String> {
        let url = arguments.get("url").and_then(|v| v.as_str());
        let browser = self.session_manager.get_or_create_browser()?;

        // Use existing session or create one
        let session_id = if let Some(sid) = self.get_active_session_id() {
            sid
        } else {
            self.session_manager.create_session(None)?
        };

        let session = self.session_manager.get_session_mut(&session_id)
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

    async fn handle_tab_switch(&mut self, arguments: Value) -> Result<String> {
        let tab_id = arguments.get("tab_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing tab_id"))?;

        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session_mut(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        session.switch_tab(tab_id)?;

        let response = json!({
            "success": true,
            "tab_id": tab_id,
            "message": "Switched to tab"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_tab_close(&mut self, _arguments: Value) -> Result<String> {
        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session_mut(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        session.close_current_tab()?;

        let response = json!({
            "success": true,
            "message": "Tab closed in live browser"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_screenshot(&mut self, _arguments: Value) -> Result<String> {
        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let png_bytes = tokio::task::block_in_place(|| -> Result<Vec<u8>> {
            tab.capture_screenshot(
                headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
                None,
                None,
                true
            ).map_err(|e| anyhow::anyhow!("Failed to capture screenshot: {}", e))
        })?;

        use base64::{Engine as _, engine::general_purpose};
        let b64 = general_purpose::STANDARD.encode(&png_bytes);

        let response = json!({
            "success": true,
            "screenshot_base64": b64,
            "format": "png",
            "size_bytes": png_bytes.len(),
            "message": "Screenshot captured from live browser"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_pdf(&mut self, _arguments: Value) -> Result<String> {
        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let pdf_bytes = tokio::task::block_in_place(|| -> Result<Vec<u8>> {
            tab.print_to_pdf(None)
                .map_err(|e| anyhow::anyhow!("Failed to print to PDF: {}", e))
        })?;

        use base64::{Engine as _, engine::general_purpose};
        let b64 = general_purpose::STANDARD.encode(&pdf_bytes);

        let response = json!({
            "success": true,
            "pdf_base64": b64,
            "size_bytes": pdf_bytes.len(),
            "message": "PDF generated from live browser"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_links(&mut self, _arguments: Value) -> Result<String> {
        let html = self.get_active_html()?;

        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("a").unwrap();
        let mut links = Vec::new();

        for element in document.select(&selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if let Some(href) = element.value().attr("href") {
                links.push(json!({
                    "url": href,
                    "text": text
                }));
            }
        }

        let response = json!({
            "success": true,
            "links": links,
            "count": links.len(),
            "engine": "scraper on live browser HTML"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_extract(&mut self, arguments: Value) -> Result<String> {
        let schema = arguments.get("schema").cloned().unwrap_or(json!({}));
        let html = self.get_active_html()?;

        // Use Firecrawl extractor if schema provided, otherwise basic extraction
        if schema.is_object() && !schema.as_object().map_or(true, |o| o.is_empty()) {
            let extraction = self.firecrawl.extract_with_schema(&html, schema);
            Ok(serde_json::to_string_pretty(&extraction)?)
        } else {
            let document = scraper::Html::parse_document(&html);
            let title_sel = scraper::Selector::parse("title").unwrap();
            let title = document.select(&title_sel).next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_else(|| "No Title".to_string());

            let body_sel = scraper::Selector::parse("body").unwrap();
            let body_text: String = document.select(&body_sel).next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            let word_count = body_text.split_whitespace().count();

            let response = json!({
                "success": true,
                "data": {
                    "title": title,
                    "word_count": word_count,
                    "extracted_content_snippet": body_text.chars().take(500).collect::<String>()
                }
            });
            Ok(serde_json::to_string_pretty(&response)?)
        }
    }

    async fn handle_find_element(&mut self, arguments: Value) -> Result<String> {
        let instruction = arguments.get("instruction").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing instruction"))?;

        let html = self.get_active_html()?;
        let res = self.stagehand.find_element(instruction, &html);
        Ok(serde_json::to_string_pretty(&res)?)
    }

    async fn handle_trafilatura(&mut self, _arguments: Value) -> Result<String> {
        let html = self.get_active_html()?;
        let session_id = self.get_active_session_id().unwrap_or_default();
        let url = self.session_manager.get_session(&session_id)
            .and_then(|s| s.current_page_state().map(|p| p.url.clone()))
            .unwrap_or_else(|| "unknown".to_string());

        let extraction = self.trafilatura.extract_content(&html, &url);
        Ok(serde_json::to_string_pretty(&extraction)?)
    }

    async fn handle_create_profile(&mut self, arguments: Value) -> Result<String> {
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing profile name"))?;
        let proxy = arguments.get("proxy").and_then(|v| v.as_str());
        let stealth_level = arguments.get("stealth_level").and_then(|v| v.as_str()).unwrap_or("high");

        let pm = crate::session::ProfileManager::new("nexusmcp_profiles.db")
            .map_err(|e| anyhow::anyhow!("Failed to initialize ProfileManager: {}", e))?;
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

    async fn handle_load_profile(&mut self, arguments: Value) -> Result<String> {
        let profile_id = arguments.get("profile_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing profile_id"))?;

        let pm = crate::session::ProfileManager::new("nexusmcp_profiles.db")
            .map_err(|e| anyhow::anyhow!("Failed to initialize ProfileManager: {}", e))?;
        let profile_opt = pm.get_profile(profile_id)
            .map_err(|e| anyhow::anyhow!("Failed to query profile: {}", e))?;

        if let Some(profile) = profile_opt {
            let session_id = self.session_manager.create_session(Some(profile.id.clone()))?;

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

    async fn handle_stealth_rotate(&mut self, arguments: Value) -> Result<String> {
        let level = arguments.get("level").and_then(|v| v.as_str()).unwrap_or("high");

        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        let stealth_config = self.stealth_engine.apply_stealth(level);
        let script = stealth_config["script"].as_str().unwrap_or("").to_string();
        let user_agent = stealth_config["user_agent"].as_str().unwrap_or("").to_string();

        tokio::task::block_in_place(|| -> Result<()> {
            tab.call_method(headless_chrome::protocol::cdp::Page::AddScriptToEvaluateOnNewDocument {
                source: script.clone(),
                world_name: None,
                include_command_line_api: None,
                run_immediately: None,
            })?;
            tab.evaluate(&script, false)?;
            Ok(())
        })?;

        let response = json!({
            "success": true,
            "level": level,
            "new_user_agent": user_agent,
            "techniques_applied": stealth_config["techniques_applied"],
            "message": "Stealth fingerprint rotated on live browser tab"
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_observe(&mut self, arguments: Value) -> Result<String> {
        let instruction = arguments.get("instruction").and_then(|v| v.as_str())
            .unwrap_or("analyze page");

        let html = self.get_active_html()?;
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

        // Also use stagehand if instruction is specific
        let stagehand_result = if instruction != "analyze page" {
            Some(self.stagehand.find_element(instruction, &html))
        } else {
            None
        };

        // Check for bot protection on this page
        let protection = self.crawl4ai.detect_protection("", &html);

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

    async fn handle_act(&mut self, arguments: Value) -> Result<String> {
        let goal = arguments.get("goal").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing goal"))?;

        let html = self.get_active_html()?;
        let tab = self.get_active_tab()
            .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

        // Use stagehand to find the target element for the goal
        let stagehand_result = self.stagehand.find_element(goal, &html);
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

        // Determine action: click for buttons/links, type for inputs
        let role = stagehand_result["element"]["role"].as_str().unwrap_or("generic");
        let action_taken;

        match role {
            "button" | "link" => {
                tokio::task::block_in_place(|| -> Result<()> {
                    let element = tab.find_element(target_selector)
                        .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                    element.click()
                        .map_err(|e| anyhow::anyhow!("Click failed: {}", e))?;
                    Ok(())
                })?;
                action_taken = "click";
            }
            "textbox" => {
                // If the goal contains text to type, extract it
                let text_to_type = if let Some(val) = arguments.get("value").and_then(|v| v.as_str()) {
                    val.to_string()
                } else {
                    String::new()
                };

                if !text_to_type.is_empty() {
                    tokio::task::block_in_place(|| -> Result<()> {
                        let element = tab.find_element(target_selector)
                            .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                        element.click()
                            .map_err(|e| anyhow::anyhow!("Focus failed: {}", e))?;
                        element.type_into(&text_to_type)
                            .map_err(|e| anyhow::anyhow!("Type failed: {}", e))?;
                        Ok(())
                    })?;
                    action_taken = "type";
                } else {
                    tokio::task::block_in_place(|| -> Result<()> {
                        let element = tab.find_element(target_selector)
                            .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                        element.click()
                            .map_err(|e| anyhow::anyhow!("Focus failed: {}", e))?;
                        Ok(())
                    })?;
                    action_taken = "focus";
                }
            }
            _ => {
                tokio::task::block_in_place(|| -> Result<()> {
                    let element = tab.find_element(target_selector)
                        .map_err(|e| anyhow::anyhow!("Element not found: {}", e))?;
                    element.click()
                        .map_err(|e| anyhow::anyhow!("Click failed: {}", e))?;
                    Ok(())
                })?;
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

    async fn handle_captcha(&mut self, arguments: Value) -> Result<String> {
        let action = arguments.get("action").and_then(|v| v.as_str()).unwrap_or("detect");

        let html = if let Ok(h) = self.get_active_html() { h } else { String::new() };
        let detection = self.crawl4ai.detect_protection("", &html);

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
                if let Some(tab) = self.get_active_tab() {
                    let stealth_config = self.stealth_engine.apply_stealth("high");
                    if let Some(script) = stealth_config["script"].as_str() {
                        let script_owned = script.to_string();
                        let _ = tokio::task::block_in_place(|| {
                            let _ = tab.evaluate(&script_owned, false);
                        });
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
            })
        };

        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_health_check(&mut self, _arguments: Value) -> Result<String> {
        let active_sessions = self.session_manager.sessions.len();
        let browser_launched = self.session_manager.browser.is_some();
        let has_active_tab = self.get_active_tab().is_some();

        let response = json!({
            "success": true,
            "status": "healthy",
            "details": {
                "active_sessions": active_sessions,
                "browser_launched": browser_launched,
                "has_active_tab": has_active_tab,
                "engine": "headless_chrome (Chromium CDP)",
                "memory_entries": self.memory.len(),
                "tools_registered": self.tools.len()
            }
        });
        Ok(serde_json::to_string_pretty(&response)?)
    }

    async fn handle_smart_retry(&mut self, arguments: Value) -> Result<String> {
        let url = arguments.get("url").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing url — provide the URL to retry navigation for"))?;
        let max_retries = arguments.get("max_retries").and_then(|v| v.as_u64()).unwrap_or(3);

        let browser = self.session_manager.get_or_create_browser()?;
        let stealth_levels = ["low", "medium", "high"];
        let mut last_error = String::new();

        for attempt in 0..max_retries {
            let level = stealth_levels.get(attempt as usize).unwrap_or(&"high");

            let session_id = self.session_manager.create_session(None)?;

            // Apply stealth for this attempt
            let stealth_config = self.stealth_engine.apply_stealth(level);
            if let Some(script) = stealth_config["script"].as_str() {
                let session = self.session_manager.get_session(&session_id)
                    .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
                if let Some(tab) = &session.tab {
                    let script_owned = script.to_string();
                    let _ = tokio::task::block_in_place(|| {
                        let _ = tab.evaluate(&script_owned, false);
                    });
                }
            }

            let session = self.session_manager.get_session_mut(&session_id)
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

            match session.navigate(url, &browser).await {
                Ok(page) => {
                    let html = session.get_current_html().unwrap_or_default();
                    let detection = self.crawl4ai.detect_protection(url, &html);
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
}