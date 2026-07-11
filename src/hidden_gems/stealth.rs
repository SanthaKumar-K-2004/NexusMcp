// Playwright-stealth CDP evasion scripts — real implementation
// Generates injectable JavaScript for navigator overrides, WebGL spoofing,
// plugin emulation, and other fingerprint evasion techniques.

use serde_json::{json, Value};

/// Pool of realistic user agents with matching platform metadata
const USER_AGENTS: &[(&str, &str, &str)] = &[
    // (user_agent, platform, vendor)
    ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36", "Win32", "Google Inc."),
    ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36", "Win32", "Google Inc."),
    ("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36", "MacIntel", "Google Inc."),
    ("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36", "MacIntel", "Google Inc."),
    ("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36", "Linux x86_64", "Google Inc."),
    ("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0", "Win32", ""),
    ("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:125.0) Gecko/20100101 Firefox/125.0", "MacIntel", ""),
    ("Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0", "Linux x86_64", ""),
    ("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15", "MacIntel", "Apple Computer, Inc."),
    ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0", "Win32", "Google Inc."),
];

pub struct PlaywrightStealth {
    ua_index: std::sync::atomic::AtomicUsize,
}

impl PlaywrightStealth {
    pub fn new() -> Self {
        Self {
            ua_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Get the next user agent from the rotation pool.
    pub fn next_user_agent(&self) -> (&'static str, &'static str, &'static str) {
        let idx = self
            .ua_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            % USER_AGENTS.len();
        USER_AGENTS[idx]
    }

    /// Generate CDP-injectable JavaScript for the given stealth level.
    /// Returns a JSON object with separate script strings that can be
    /// injected via `Page.addScriptToEvaluateOnNewDocument` or `tab.evaluate`.
    pub fn apply_stealth(&self, level: &str) -> Value {
        let (ua, platform, vendor) = self.next_user_agent();
        let mut scripts: Vec<String> = Vec::new();
        let mut techniques: Vec<&str> = Vec::new();

        // === Always applied ===

        // 1. Hide navigator.webdriver
        scripts.push(format!(
            "Object.defineProperty(navigator, 'webdriver', {{ get: () => undefined }});"
        ));
        techniques.push("webdriver_hide");

        if level == "medium" || level == "high" {
            // 2. Override navigator.userAgent, platform, vendor
            scripts.push(format!(
                r#"Object.defineProperty(navigator, 'userAgent', {{ get: () => '{}' }});
Object.defineProperty(navigator, 'platform', {{ get: () => '{}' }});
Object.defineProperty(navigator, 'vendor', {{ get: () => '{}' }});"#,
                ua, platform, vendor
            ));
            techniques.push("navigator_override");

            // 3. Override navigator.languages
            scripts.push(
                "Object.defineProperty(navigator, 'languages', { get: () => ['en-US', 'en'] });"
                    .to_string(),
            );
            techniques.push("languages");

            // 4. Override navigator.hardwareConcurrency
            scripts.push(
                "Object.defineProperty(navigator, 'hardwareConcurrency', { get: () => 8 });"
                    .to_string(),
            );
            techniques.push("hardware_concurrency");

            // 5. Override navigator.deviceMemory
            scripts.push(
                "Object.defineProperty(navigator, 'deviceMemory', { get: () => 8 });".to_string(),
            );
            techniques.push("device_memory");
        }

        if level == "high" {
            // 6. Fake chrome.runtime to look like a real Chrome extension context
            scripts.push(
                r#"window.chrome = { runtime: { connect: function(){}, sendMessage: function(){} }, loadTimes: function(){return {}}, csi: function(){return {}} };"#.to_string()
            );
            techniques.push("chrome_runtime");

            // 7. Override Permissions.query to return "prompt" for notifications
            scripts.push(
                r#"const originalQuery = window.navigator.permissions.query.bind(window.navigator.permissions);
window.navigator.permissions.query = (parameters) => (
    parameters.name === 'notifications' ?
        Promise.resolve({ state: Notification.permission }) :
        originalQuery(parameters)
);"#.to_string()
            );
            techniques.push("permissions_query");

            // 8. Spoof WebGL vendor/renderer
            scripts.push(
                r#"const getParameter = WebGLRenderingContext.prototype.getParameter;
WebGLRenderingContext.prototype.getParameter = function(parameter) {
    if (parameter === 37445) return 'Intel Inc.';
    if (parameter === 37446) return 'Intel Iris OpenGL Engine';
    return getParameter.call(this, parameter);
};"#
                .to_string(),
            );
            techniques.push("webgl_vendor");

            // 9. Fake plugins array (Chrome typically has 5 plugins)
            scripts.push(
                r#"Object.defineProperty(navigator, 'plugins', {
    get: () => {
        const plugins = [
            { name: 'Chrome PDF Plugin', filename: 'internal-pdf-viewer', description: 'Portable Document Format' },
            { name: 'Chrome PDF Viewer', filename: 'mhjfbmdgcfjbbpaeojofohoefgiehjai', description: '' },
            { name: 'Native Client', filename: 'internal-nacl-plugin', description: '' },
        ];
        plugins.length = 3;
        return plugins;
    }
});"#.to_string()
            );
            techniques.push("plugins");

            // 10. Override connection.rtt to a realistic value
            scripts.push(
                r#"Object.defineProperty(navigator, 'connection', {
    get: () => ({ effectiveType: '4g', rtt: 50, downlink: 10, saveData: false })
});"#
                    .to_string(),
            );
            techniques.push("connection_info");
        }

        let combined_script = scripts.join("\n");

        json!({
            "level": level,
            "user_agent": ua,
            "platform": platform,
            "techniques_applied": techniques,
            "script": combined_script,
            "method": "playwright-stealth (real CDP injection scripts)"
        })
    }
}
