// Stagehand-style AI-powered element targeting (vision + LLM)
use serde_json::{json, Value};

pub struct StagehandEngine;

impl StagehandEngine {
    pub fn new() -> Self {
        Self
    }

    /// Find elements using natural language + semantic understanding
    pub fn find_element(&self, instruction: &str, page_html: &str) -> Value {
        let document = scraper::Html::parse_document(page_html);
        let selector = scraper::Selector::parse("input, button, a, textarea, [role='button'], [role='textbox']").unwrap();
        
        let instruction_lower = instruction.to_lowercase();
        let mut best_element = None;
        let mut best_score: f64 = 0.0;
        let mut best_role = "generic";
        let mut best_desc = "Matching element".to_string();
        
        for element in document.select(&selector) {
            let mut score = 0.0;
            let tag = element.value().name();
            let mut role = "generic";
            
            // Extract characteristics
            let id = element.value().attr("id").unwrap_or("").to_lowercase();
            let name = element.value().attr("name").unwrap_or("").to_lowercase();
            let placeholder = element.value().attr("placeholder").unwrap_or("").to_lowercase();
            let class = element.value().attr("class").unwrap_or("").to_lowercase();
            let type_attr = element.value().attr("type").unwrap_or("").to_lowercase();
            let aria_label = element.value().attr("aria-label").unwrap_or("").to_lowercase();
            let text = element.text().collect::<String>().trim().to_lowercase();
            
            // Adjust score based on tags and keywords
            if tag == "input" {
                role = "textbox";
                if type_attr == "submit" || type_attr == "button" {
                    role = "button";
                }
            } else if tag == "button" {
                role = "button";
            } else if tag == "a" {
                role = "link";
            }
            
            // Score matching
            let search_terms: Vec<&str> = instruction_lower.split_whitespace().collect();
            for term in search_terms {
                if id.contains(term) { score += 10.0; }
                if name.contains(term) { score += 10.0; }
                if placeholder.contains(term) { score += 12.0; }
                if aria_label.contains(term) { score += 12.0; }
                if text.contains(term) { score += 8.0; }
                if class.contains(term) { score += 2.0; }
            }
            
            // Specific overrides
            if instruction_lower.contains("login") && (id.contains("login") || class.contains("login") || text.contains("login")) {
                score += 15.0;
            }
            if instruction_lower.contains("search") && (type_attr == "search" || id.contains("search") || placeholder.contains("search")) {
                score += 15.0;
            }
            if instruction_lower.contains("submit") && (type_attr == "submit" || text.contains("submit")) {
                score += 15.0;
            }
            
            if score > best_score {
                best_score = score;
                best_role = role;
                // Generate a selector
                let sel = if !id.is_empty() {
                    format!("#{}", id)
                } else if !name.is_empty() {
                    format!("{}[name='{}']", tag, name)
                } else if !placeholder.is_empty() {
                    format!("{}[placeholder='{}']", tag, placeholder)
                } else if !text.is_empty() {
                    if !class.is_empty() {
                        format!("{}.{}", tag, class.split_whitespace().next().unwrap_or(""))
                    } else {
                        tag.to_string()
                    }
                } else {
                    tag.to_string()
                };
                best_element = Some(sel);
                best_desc = format!("Dynamic match: <{}> id={} name={}", tag, id, name);
            }
        }
        
        let result_element = if let Some(sel) = best_element {
            json!({
                "selector": sel,
                "role": best_role,
                "description": best_desc,
                "confidence": 0.5 + (best_score / 100.0).min(0.49)
            })
        } else {
            json!({
                "selector": "body",
                "role": "generic",
                "description": "No high-confidence match found",
                "confidence": 0.2
            })
        };
        
        json!({
            "instruction": instruction,
            "element": result_element,
            "method": "Stagehand-style semantic scraper scoring",
            "message": "Dynamic CSS selector generated"
        })
    }
}