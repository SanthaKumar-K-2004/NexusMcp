// Stagehand-style AI-powered element targeting
// Real implementation using scraper with weighted attribute scoring,
// label association, ARIA attribute matching, and multi-candidate ranking.

use serde_json::{json, Value};

pub struct StagehandEngine;

impl StagehandEngine {
    pub fn new() -> Self {
        Self
    }

    /// Find elements using natural language instruction + semantic DOM scoring.
    /// Returns top 3 candidates ranked by confidence.
    pub fn find_element(&self, instruction: &str, page_html: &str) -> Value {
        let document = scraper::Html::parse_document(page_html);
        let interactive_sel = scraper::Selector::parse(
            "input, button, a, textarea, select, [role='button'], [role='textbox'], [role='link'], [role='checkbox'], [role='radio'], [role='tab'], [role='menuitem'], [contenteditable='true']"
        ).unwrap();

        let instruction_lower = instruction.to_lowercase();
        let search_terms: Vec<&str> = instruction_lower.split_whitespace()
            .filter(|w| w.len() > 1) // skip single-char words
            .collect();

        // Collect label text for label[for] → input association
        let label_sel = scraper::Selector::parse("label[for]").ok();
        let mut label_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        if let Some(ref sel) = label_sel {
            for label_el in document.select(sel) {
                if let Some(for_id) = label_el.value().attr("for") {
                    let label_text = label_el.text().collect::<String>().trim().to_lowercase();
                    label_map.insert(for_id.to_string(), label_text);
                }
            }
        }

        let mut candidates: Vec<(f64, String, String, String)> = Vec::new(); // (score, selector, role, description)

        for element in document.select(&interactive_sel) {
            let mut score: f64 = 0.0;
            let tag = element.value().name();

            // Extract attributes
            let id = element.value().attr("id").unwrap_or("").to_lowercase();
            let name = element.value().attr("name").unwrap_or("").to_lowercase();
            let placeholder = element.value().attr("placeholder").unwrap_or("").to_lowercase();
            let class = element.value().attr("class").unwrap_or("").to_lowercase();
            let type_attr = element.value().attr("type").unwrap_or("").to_lowercase();
            let aria_label = element.value().attr("aria-label").unwrap_or("").to_lowercase();
            let aria_role = element.value().attr("role").unwrap_or("").to_lowercase();
            let data_testid = element.value().attr("data-testid").unwrap_or("").to_lowercase();
            let data_cy = element.value().attr("data-cy").unwrap_or("").to_lowercase();
            let title_attr = element.value().attr("title").unwrap_or("").to_lowercase();
            let text = element.text().collect::<String>().trim().to_lowercase();

            // Determine semantic role
            let role = if !aria_role.is_empty() {
                aria_role.clone()
            } else {
                match tag {
                    "input" if type_attr == "submit" || type_attr == "button" => "button".to_string(),
                    "input" if type_attr == "checkbox" => "checkbox".to_string(),
                    "input" if type_attr == "radio" => "radio".to_string(),
                    "input" => "textbox".to_string(),
                    "button" => "button".to_string(),
                    "a" => "link".to_string(),
                    "textarea" => "textbox".to_string(),
                    "select" => "combobox".to_string(),
                    _ => "generic".to_string(),
                }
            };

            // Score matching — weighted by attribute relevance
            // Highest weight: aria-label, placeholder (explicitly user-facing)
            // Medium weight: id, name, data-testid
            // Lower weight: text content, class
            for term in &search_terms {
                if aria_label.contains(term) { score += 14.0; }
                if placeholder.contains(term) { score += 13.0; }
                if id.contains(term) { score += 11.0; }
                if name.contains(term) { score += 10.0; }
                if data_testid.contains(term) { score += 10.0; }
                if data_cy.contains(term) { score += 10.0; }
                if title_attr.contains(term) { score += 9.0; }
                if text.contains(term) { score += 8.0; }
                if class.contains(term) { score += 3.0; }
            }

            // Check label[for] association
            if !id.is_empty() {
                if let Some(label_text) = label_map.get(&id) {
                    for term in &search_terms {
                        if label_text.contains(term) {
                            score += 12.0;
                        }
                    }
                }
            }

            // Role matching bonus
            if instruction_lower.contains("button") && role == "button" { score += 8.0; }
            if instruction_lower.contains("link") && role == "link" { score += 8.0; }
            if instruction_lower.contains("input") && role == "textbox" { score += 8.0; }
            if instruction_lower.contains("checkbox") && role == "checkbox" { score += 8.0; }
            if instruction_lower.contains("search") && (type_attr == "search" || id.contains("search") || placeholder.contains("search")) { score += 12.0; }
            if instruction_lower.contains("submit") && (type_attr == "submit" || text.contains("submit")) { score += 12.0; }
            if instruction_lower.contains("login") && (id.contains("login") || class.contains("login") || text.contains("login") || text.contains("sign in")) { score += 12.0; }
            if instruction_lower.contains("email") && (type_attr == "email" || id.contains("email") || name.contains("email") || placeholder.contains("email")) { score += 12.0; }
            if instruction_lower.contains("password") && (type_attr == "password" || id.contains("password") || name.contains("password")) { score += 12.0; }

            if score > 0.0 {
                // Generate best CSS selector for this element
                let selector = if !id.is_empty() {
                    format!("#{}", id)
                } else if !data_testid.is_empty() {
                    format!("[data-testid='{}']", element.value().attr("data-testid").unwrap_or(""))
                } else if !data_cy.is_empty() {
                    format!("[data-cy='{}']", element.value().attr("data-cy").unwrap_or(""))
                } else if !name.is_empty() {
                    format!("{}[name='{}']", tag, element.value().attr("name").unwrap_or(""))
                } else if !placeholder.is_empty() {
                    format!("{}[placeholder='{}']", tag, element.value().attr("placeholder").unwrap_or(""))
                } else if !aria_label.is_empty() {
                    format!("{}[aria-label='{}']", tag, element.value().attr("aria-label").unwrap_or(""))
                } else {
                    // Fallback: tag + first class if available
                    if !class.is_empty() {
                        let first_class = class.split_whitespace().next().unwrap_or("");
                        format!("{}.{}", tag, first_class)
                    } else {
                        tag.to_string()
                    }
                };

                let desc = format!("<{}> role={} id={} text='{}'",
                    tag, role,
                    if id.is_empty() { "(none)" } else { &id },
                    if text.len() > 40 { &text[..40] } else { &text }
                );

                candidates.push((score, selector, role, desc));
            }
        }

        // Sort by score descending, take top 3
        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(3);

        let max_score = candidates.first().map(|c| c.0).unwrap_or(0.0);
        let result_elements: Vec<Value> = candidates.iter().map(|(score, sel, role, desc)| {
            json!({
                "selector": sel,
                "role": role,
                "description": desc,
                "confidence": (0.5 + (score / (max_score * 2.0)).min(0.49))
            })
        }).collect();

        if result_elements.is_empty() {
            json!({
                "instruction": instruction,
                "element": {
                    "selector": "body",
                    "role": "generic",
                    "description": "No matching interactive element found",
                    "confidence": 0.0
                },
                "candidates": [],
                "method": "Stagehand semantic DOM scoring"
            })
        } else {
            json!({
                "instruction": instruction,
                "element": result_elements[0].clone(),
                "candidates": result_elements,
                "method": "Stagehand semantic DOM scoring"
            })
        }
    }
}