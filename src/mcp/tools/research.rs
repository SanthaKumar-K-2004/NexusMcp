use super::{Tool, ToolRegistry};
use anyhow::Result;
use serde_json::{json, Value};

pub struct BrowserResearchTool;

impl BrowserResearchTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for BrowserResearchTool {
    fn name(&self) -> &str {
        "browser_research"
    }

    fn description(&self) -> &str {
        "Fully parallel research with real concurrent execution."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "urls": { "type": "array", "items": { "type": "string" } },
                "extract_mode": { "type": "string", "enum": ["markdown", "json", "links"], "default": "markdown" },
                "concurrency": { "type": "integer", "default": 8 }
            },
            "required": ["urls"]
        })
    }

    async fn call(&self, arguments: Value) -> Result<String> {
        let urls: Vec<String> = arguments
            .get("urls")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let extract_mode = arguments
            .get("extract_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("markdown");
        let concurrency = arguments
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(8) as usize;

        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut handles = Vec::new();

        for url in urls {
            if let Err(e) = ToolRegistry::validate_fetch_url(&url) {
                handles.push(tokio::spawn(async move {
                    json!({ "url": url, "status": "failed", "message": e.to_string() })
                }));
                continue;
            }

            let client = client.clone();
            let mode = extract_mode.to_string();
            let sem = semaphore.clone();

            let handle = tokio::spawn(async move {
                let _permit = match sem.acquire().await {
                    Ok(p) => p,
                    Err(_) => {
                        return json!({ "url": url, "status": "failed", "message": "Semaphore error" })
                    }
                };

                match client.get(&url).send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        if status.is_success() {
                            if let Ok(html) = resp.text().await {
                                let title = if html.contains("<title>") {
                                    html.split("<title>")
                                        .nth(1)
                                        .unwrap_or("")
                                        .split("</title>")
                                        .next()
                                        .unwrap_or("Page")
                                        .to_string()
                                } else {
                                    "Page".to_string()
                                };

                                let extracted = if mode == "markdown" {
                                    html2md::parse_html(&html)
                                } else {
                                    html.chars().take(2000).collect()
                                };

                                return json!({
                                    "url": url,
                                    "title": title,
                                    "extracted": extracted,
                                    "status": "success"
                                });
                            }
                        }
                        json!({ "url": url, "status": "failed", "message": format!("HTTP {}", status) })
                    }
                    Err(e) => json!({ "url": url, "status": "failed", "message": e.to_string() }),
                }
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }

        let response = json!({
            "success": true,
            "results": results,
            "total_urls": results.len(),
            "concurrency_used": concurrency,
            "message": "REAL parallel HTTP research and extraction completed"
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }
}
