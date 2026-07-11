use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

// ==================== EXTRACTION TOOLS ====================
// Schema-only definitions. Actual execution in functions below.

pub struct BrowserMarkdownTool;
impl BrowserMarkdownTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserMarkdownTool {
    fn name(&self) -> &str {
        "browser_markdown"
    }
    fn description(&self) -> &str {
        "Extract clean Markdown from the current live page."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "include_images": { "type": "boolean", "default": false },
                "max_length": { "type": "integer", "default": 8000 }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserExtractTool;
impl BrowserExtractTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserExtractTool {
    fn name(&self) -> &str {
        "browser_extract"
    }
    fn description(&self) -> &str {
        "Extract structured data from the current page using CSS selectors or a schema."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "schema": { "type": "object", "description": "Schema defining what fields to extract (e.g. title, emails, prices)" },
                "prompt": { "type": "string" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserLinksTool;
impl BrowserLinksTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserLinksTool {
    fn name(&self) -> &str {
        "browser_links"
    }
    fn description(&self) -> &str {
        "Extract all links from the current live page."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "filter": { "type": "string", "description": "Optional filter for link URLs or text" }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserPdfTool;
impl BrowserPdfTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserPdfTool {
    fn name(&self) -> &str {
        "browser_pdf"
    }
    fn description(&self) -> &str {
        "Generate a PDF of the current page from the live browser."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "format": { "type": "string", "default": "A4" },
                "margin": { "type": "integer", "default": 20 }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

pub struct BrowserScreenshotTool;
impl BrowserScreenshotTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserScreenshotTool {
    fn name(&self) -> &str {
        "browser_screenshot"
    }
    fn description(&self) -> &str {
        "Take a PNG screenshot of the current page from the live browser."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "Optional CSS selector to screenshot a specific element" },
                "full_page": { "type": "boolean", "default": true }
            }
        })
    }
    async fn call(&self, _arguments: Value) -> Result<String> {
        Err(anyhow::anyhow!("Routed via ToolRegistry::call_tool"))
    }
}

// ==================== HANDLER IMPLEMENTATIONS ====================

pub async fn handle_markdown(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let html = registry.get_active_html()?;
    let session_id = registry.get_active_session_id().unwrap_or_default();
    let session = registry
        .session_manager
        .get_session(&session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let url = session
        .current_page_state()
        .map(|p| p.url.clone())
        .unwrap_or_default();
    let title = session
        .current_page_state()
        .map(|p| p.title.clone())
        .unwrap_or_default();

    let markdown = registry.extractor.html_to_markdown(&html, &url)?;
    registry.vector_memory.store(&url, &markdown);
    registry
        .memory
        .push("Extracted Markdown from live page".to_string());
    if registry.memory.len() > 100 {
        registry.memory.remove(0);
    }

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

pub async fn handle_screenshot(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let tab_clone = tab.clone();
    let png_bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        tab_clone
            .capture_screenshot(
                headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
                None,
                None,
                true,
            )
            .map_err(|e| anyhow::anyhow!("Failed to capture screenshot: {}", e))
    })
    .await??;

    use base64::{engine::general_purpose, Engine as _};
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

pub async fn handle_pdf(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let tab = registry
        .get_active_tab()
        .ok_or_else(|| anyhow::anyhow!("No active browser session — navigate first"))?;

    let tab_clone = tab.clone();
    let pdf_bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        tab_clone
            .print_to_pdf(None)
            .map_err(|e| anyhow::anyhow!("Failed to print to PDF: {}", e))
    })
    .await??;

    use base64::{engine::general_purpose, Engine as _};
    let b64 = general_purpose::STANDARD.encode(&pdf_bytes);

    let response = json!({
        "success": true,
        "pdf_base64": b64,
        "size_bytes": pdf_bytes.len(),
        "message": "PDF generated from live browser"
    });
    Ok(serde_json::to_string_pretty(&response)?)
}

pub async fn handle_links(registry: &mut ToolRegistry, _arguments: Value) -> Result<String> {
    let html = registry.get_active_html()?;

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

pub async fn handle_extract(registry: &mut ToolRegistry, arguments: Value) -> Result<String> {
    let schema = arguments.get("schema").cloned().unwrap_or(json!({}));
    let html = registry.get_active_html()?;

    if schema.is_object() && !schema.as_object().map_or(true, |o| o.is_empty()) {
        let extraction = registry.firecrawl.extract_with_schema(&html, schema);
        Ok(serde_json::to_string_pretty(&extraction)?)
    } else {
        let document = scraper::Html::parse_document(&html);
        let title_sel = scraper::Selector::parse("title").unwrap();
        let title = document
            .select(&title_sel)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No Title".to_string());

        let body_sel = scraper::Selector::parse("body").unwrap();
        let body_text: String = document
            .select(&body_sel)
            .next()
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
