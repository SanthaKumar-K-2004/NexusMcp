// Crawl4AI-style anti-bot detection — real HTML-based implementation
// Analyzes actual response HTML for bot protection markers, not just URL patterns.

use serde_json::{json, Value};

pub struct Crawl4AIDetector;

impl Crawl4AIDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect bot protection by analyzing actual page HTML content.
    /// Checks for Cloudflare, reCAPTCHA, hCaptcha, Turnstile, Akamai, DataDome,
    /// and generic WAF/access-denied patterns.
    pub fn detect_protection(&self, url: &str, html: &str) -> Value {
        let mut detections: Vec<Value> = Vec::new();

        // Cloudflare challenge
        let cf_markers = [
            "cf-challenge-running",
            "cf-chl-widget",
            "__cf_chl_jschl_tk__",
            "cf-browser-verification",
            "cf_chl_opt",
            "challenge-platform",
            "Just a moment...",
        ];
        if cf_markers.iter().any(|m| html.contains(m)) {
            detections.push(json!({"type": "cloudflare", "severity": "high"}));
        }

        // Cloudflare Turnstile
        if html.contains("cf-turnstile") || html.contains("challenges.cloudflare.com/turnstile") {
            detections.push(json!({"type": "turnstile", "severity": "high"}));
        }

        // reCAPTCHA (Google)
        let recaptcha_markers = [
            "g-recaptcha",
            "recaptcha/api.js",
            "grecaptcha.execute",
            "recaptcha-token",
        ];
        if recaptcha_markers.iter().any(|m| html.contains(m)) {
            detections.push(json!({"type": "recaptcha", "severity": "high"}));
        }

        // hCaptcha
        let hcaptcha_markers = ["h-captcha", "hcaptcha.com/1/api.js", "hcaptcha-response"];
        if hcaptcha_markers.iter().any(|m| html.contains(m)) {
            detections.push(json!({"type": "hcaptcha", "severity": "high"}));
        }

        // Akamai Bot Manager
        let akamai_markers = ["_abck", "ak_bmsc", "bm_sz", "akamai"];
        if akamai_markers.iter().any(|m| html.contains(m)) {
            detections.push(json!({"type": "akamai", "severity": "medium"}));
        }

        // DataDome
        if html.contains("datadome") || html.contains("dd.js") {
            detections.push(json!({"type": "datadome", "severity": "medium"}));
        }

        // PerimeterX / HUMAN
        if html.contains("perimeterx") || html.contains("px-captcha") || html.contains("_pxhd") {
            detections.push(json!({"type": "perimeterx", "severity": "high"}));
        }

        // Generic WAF / Access Denied patterns
        let waf_markers = [
            "Access Denied",
            "403 Forbidden",
            "Request Blocked",
            "Sorry, you have been blocked",
            "Please verify you are a human",
            "Enable JavaScript and cookies to continue",
        ];
        if waf_markers.iter().any(|m| html.contains(m)) {
            detections.push(json!({"type": "waf_block", "severity": "high"}));
        }

        // Determine overall protection level
        let protection_level = if detections.iter().any(|d| d["severity"] == "high") {
            "high"
        } else if !detections.is_empty() {
            "medium"
        } else {
            "none"
        };

        let recommended_action = match protection_level {
            "high" => "stealth_high + proxy_rotation + delay",
            "medium" => "stealth_medium + user_agent_rotation",
            _ => "normal",
        };

        json!({
            "url": url,
            "protection_level": protection_level,
            "detections": detections,
            "detection_count": detections.len(),
            "recommended_action": recommended_action,
            "method": "Crawl4AI HTML content analysis"
        })
    }
}
