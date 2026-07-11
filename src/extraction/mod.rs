// Advanced Extraction Module - Hidden Gem Combination
// Combines: scraper + html2md + custom heuristics

use anyhow::Result;
use scraper::{Html, Selector};

pub struct AdvancedExtractor;

impl AdvancedExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Convert HTML to high-quality Markdown (Firecrawl-style)
    pub fn html_to_markdown(&self, html: &str, url: &str) -> Result<String> {
        // Use html2md for clean conversion
        let markdown = html2md::parse_html(html);

        // Post-process for better AI consumption
        let cleaned = self.clean_markdown(&markdown, url);
        Ok(cleaned)
    }

    /// Extract structured data using CSS selectors
    pub fn extract_structured(
        &self,
        html: &str,
        selectors: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let document = Html::parse_document(html);
        let mut result = serde_json::Map::new();

        for (key, selector_str) in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                let texts: Vec<String> = document
                    .select(&selector)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();

                if !texts.is_empty() {
                    result.insert(key.to_string(), serde_json::json!(texts));
                }
            }
        }

        Ok(serde_json::Value::Object(result))
    }

    fn clean_markdown(&self, md: &str, _url: &str) -> String {
        md.lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty()
                    && !trimmed.starts_with("var ")
                    && !trimmed.starts_with("function ")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }
}
