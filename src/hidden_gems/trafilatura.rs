// Trafilatura-style content extraction — real implementation using scraper
// Strips boilerplate (nav, footer, script, style, aside) and extracts article body.

use scraper::{Html, Selector};
use serde_json::{json, Value};

pub struct TrafilaturaExtractor;

impl TrafilaturaExtractor {
    pub fn new() -> Self {
        Self
    }

    /// High-quality article/content extraction from raw HTML.
    /// Strips boilerplate elements and extracts the main readable text.
    pub fn extract_content(&self, html: &str, url: &str) -> Value {
        let document = Html::parse_document(html);

        // 1. Extract title from <title> tag
        let title = Selector::parse("title")
            .ok()
            .and_then(|sel| document.select(&sel).next())
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // 2. Extract meta description
        let meta_description = Selector::parse("meta[name='description']")
            .ok()
            .and_then(|sel| document.select(&sel).next())
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()))
            .unwrap_or_default();

        // 3. Extract author from meta tags
        let author = Selector::parse("meta[name='author']")
            .ok()
            .and_then(|sel| document.select(&sel).next())
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()))
            .unwrap_or_default();

        // 4. Try to find main content in priority order:
        //    <article> > <main> > <div role="main"> > <body>
        let content_selectors = [
            "article",
            "main",
            "[role='main']",
            ".post-content",
            ".article-body",
            ".entry-content",
            "#content",
            "body",
        ];

        let mut content_text = String::new();
        for sel_str in &content_selectors {
            if let Ok(sel) = Selector::parse(sel_str) {
                if let Some(el) = document.select(&sel).next() {
                    content_text = self.extract_clean_text_from_element(&el);
                    if content_text.split_whitespace().count() > 20 {
                        break; // Found substantial content
                    }
                }
            }
        }

        // 5. If content is still too short, fallback to full body text
        if content_text.split_whitespace().count() < 20 {
            if let Ok(sel) = Selector::parse("body") {
                if let Some(el) = document.select(&sel).next() {
                    content_text = self.extract_clean_text_from_element(&el);
                }
            }
        }

        let word_count = content_text.split_whitespace().count();

        json!({
            "url": url,
            "title": title,
            "author": author,
            "description": meta_description,
            "content": content_text,
            "word_count": word_count,
            "method": "Trafilatura (scraper-based article extraction)",
            "quality": if word_count > 100 { "high" } else if word_count > 20 { "medium" } else { "low" }
        })
    }

    /// Extract clean text from an element, skipping boilerplate child elements.
    fn extract_clean_text_from_element(&self, element: &scraper::ElementRef) -> String {
        // Tags to skip entirely — these are boilerplate
        let skip_tags = ["script", "style", "nav", "footer", "header", "aside",
                         "noscript", "iframe", "svg", "form"];

        let mut lines: Vec<String> = Vec::new();
        self.collect_text_recursive(element, &skip_tags, &mut lines);

        // Clean up: collapse whitespace, remove empty lines, trim
        lines.iter()
            .map(|line| {
                line.split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Recursively collect text nodes, skipping boilerplate tags.
    fn collect_text_recursive(
        &self,
        element: &scraper::ElementRef,
        skip_tags: &[&str],
        out: &mut Vec<String>,
    ) {
        for child in element.children() {
            match child.value() {
                scraper::node::Node::Text(text) => {
                    let t = text.text.trim();
                    if !t.is_empty() {
                        out.push(t.to_string());
                    }
                }
                scraper::node::Node::Element(el) => {
                    if skip_tags.contains(&el.name()) {
                        continue; // Skip boilerplate
                    }
                    // Block-level elements get a line break
                    let is_block = matches!(el.name(),
                        "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
                        | "li" | "blockquote" | "pre" | "section" | "article"
                        | "tr" | "br" | "hr"
                    );
                    if is_block && !out.is_empty() {
                        out.push(String::new()); // line break
                    }
                    if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                        self.collect_text_recursive(&child_ref, skip_tags, out);
                    }
                    if is_block {
                        out.push(String::new());
                    }
                }
                _ => {}
            }
        }
    }
}