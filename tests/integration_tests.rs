use nexusmcp::engine::session::SessionManager;
use nexusmcp::hidden_gems::stagehand::StagehandEngine;
use nexusmcp::hidden_gems::firecrawl_extraction::FirecrawlExtractor;
use nexusmcp::hidden_gems::trafilatura::TrafilaturaExtractor;
use nexusmcp::hidden_gems::crawl4ai::Crawl4AIDetector;
use nexusmcp::hidden_gems::stealth::PlaywrightStealth;
use nexusmcp::extraction::AdvancedExtractor;
use nexusmcp::session::ProfileManager;
use serde_json::json;
use axum::{routing::get, response::Html, Router};

/// Helper: spins up a local test HTTP server and returns the base URL.
async fn start_test_server() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_port = listener.local_addr().unwrap().port();
    let local_url = format!("http://127.0.0.1:{}", local_port);

    let app = Router::new()
        .route("/", get(|| async {
            Html(r#"<!DOCTYPE html>
            <html>
            <head>
                <title>NexusMCP Integration Test Page</title>
                <meta name="description" content="A test page for NexusMCP">
                <meta name="author" content="Test Author">
            </head>
            <body>
                <h1 id="test-header">Real Browser E2E Rendering Working</h1>
                <p>Welcome to the NexusMCP test suite.</p>
                <input id="user-email" type="email" placeholder="enter email address" />
                <button id="submit-btn" type="submit" onclick="document.getElementById('test-header').innerText = 'Action Performed'">Submit Email</button>
                
                <div class="contact-details">
                    <p>Support email: support@nexusmcp.dev</p>
                    <p>Enterprise license price: $99.99</p>
                </div>
                <a href="/about">About</a>
                <a href="/docs">Documentation</a>
            </body>
            </html>"#)
        }))
        .route("/about", get(|| async {
            Html(r#"<!DOCTYPE html>
            <html><head><title>About Page</title></head>
            <body><h1>About NexusMCP</h1><p>This is the about page.</p></body>
            </html>"#)
        }))
        .route("/protected", get(|| async {
            Html(r#"<!DOCTYPE html>
            <html><head><title>Protected</title></head>
            <body>
                <div class="cf-challenge-running">Checking your browser</div>
                <p>Just a moment...</p>
            </body>
            </html>"#)
        }));

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    local_url
}

// ==========================================================================
// Test 1: Real browser navigation + JS evaluation + DOM interaction
// ==========================================================================
#[tokio::test(flavor = "multi_thread")]
async fn test_real_browser_and_hidden_gems_flow() -> Result<(), anyhow::Error> {
    let local_url = start_test_server().await;

    // Initialize SessionManager and Browser
    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    
    let session_id = manager.create_session(None)?;
    let session = manager.get_session_mut(&session_id).unwrap();

    // Test REAL Browser Navigation
    let page = session.navigate(&local_url, &browser).await?;
    assert_eq!(page.title, "NexusMCP Integration Test Page");
    assert_eq!(page.status, "loaded");
    assert!(page.load_time_ms > 0);
    
    let html = session.get_current_html().unwrap();
    assert!(html.contains("Real Browser E2E Rendering Working"));

    // Test REAL JS Evaluation via CDP
    let tab = session.tab.clone().unwrap();
    let eval_res = tokio::task::block_in_place(|| {
        tab.evaluate("document.getElementById('test-header').innerText", false)
    })?;
    assert_eq!(
        eval_res.value.unwrap().as_str().unwrap(),
        "Real Browser E2E Rendering Working"
    );

    // Test DOM Interaction (clicking button via CDP)
    tokio::task::block_in_place(|| {
        let btn = tab.find_element("#submit-btn")?;
        btn.click()?;
        Ok::<(), anyhow::Error>(())
    })?;

    // Verify click event triggered JS modification on the page
    let eval_after_click = tokio::task::block_in_place(|| {
        tab.evaluate("document.getElementById('test-header').innerText", false)
    })?;
    assert_eq!(
        eval_after_click.value.unwrap().as_str().unwrap(),
        "Action Performed"
    );

    // Test Stagehand Element Locator Scoring Engine
    let stagehand = StagehandEngine::new();
    let locator_res = stagehand.find_element("submit email button", &html);
    let matched_selector = locator_res["element"]["selector"].as_str().unwrap();
    assert_eq!(matched_selector, "#submit-btn");
    // Verify top-3 candidates are returned
    assert!(locator_res["candidates"].as_array().unwrap().len() > 0);

    // Test Firecrawl-style Structured Extraction (Regex + Scraper)
    let firecrawl = FirecrawlExtractor::new();
    let schema = json!({
        "title": {},
        "emails": {},
        "prices": {},
        "links_count": {}
    });
    let extraction = firecrawl.extract_with_schema(&html, schema);
    let data = &extraction["extracted_data"];
    
    assert_eq!(data["title"], "NexusMCP Integration Test Page");
    assert!(data["emails"].as_array().unwrap().contains(&json!("support@nexusmcp.dev")));
    assert!(data["prices"].as_array().unwrap().contains(&json!("$99.99")));

    // Test Markdown Extraction
    let extractor = AdvancedExtractor::new();
    let markdown = extractor.html_to_markdown(&html, &local_url)?;
    assert!(markdown.contains("Real Browser E2E Rendering Working"));
    assert!(markdown.contains("support@nexusmcp.dev"));

    Ok(())
}

// ==========================================================================
// Test 2: Browser back + reload
// ==========================================================================
#[tokio::test(flavor = "multi_thread")]
async fn test_browser_back_and_reload() -> Result<(), anyhow::Error> {
    let local_url = start_test_server().await;

    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    let session_id = manager.create_session(None)?;

    // Navigate to page 1
    let session = manager.get_session_mut(&session_id).unwrap();
    let page1 = session.navigate(&local_url, &browser).await?;
    assert_eq!(page1.title, "NexusMCP Integration Test Page");

    // Navigate to page 2
    let about_url = format!("{}/about", local_url);
    let page2 = session.navigate(&about_url, &browser).await?;
    assert_eq!(page2.title, "About Page");

    // Go back to page 1
    let back_page = session.go_back().await?;
    assert!(back_page.url.contains(&local_url) || back_page.title.contains("NexusMCP") || back_page.title == "Loaded Page");

    // Reload
    let reloaded = session.reload().await?;
    assert_eq!(reloaded.status, "loaded");

    Ok(())
}

// ==========================================================================
// Test 3: Tab management
// ==========================================================================
#[tokio::test(flavor = "multi_thread")]
async fn test_tab_management() -> Result<(), anyhow::Error> {
    let local_url = start_test_server().await;

    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    let session_id = manager.create_session(None)?;

    let session = manager.get_session_mut(&session_id).unwrap();

    // Navigate to initial page
    let _ = session.navigate(&local_url, &browser).await?;
    assert_eq!(session.pages.len(), 1);

    // Open a new tab
    let new_tab_page = session.new_tab(Some(&format!("{}/about", local_url)), &browser).await?;
    assert_eq!(new_tab_page.title, "About Page");
    assert_eq!(session.pages.len(), 2);

    // Close current tab (leaves the first tab open)
    session.close_current_tab()?;
    assert_eq!(session.pages.len(), 1);
    assert!(session.tab.is_some());

    // Close the last tab (clears everything)
    session.close_current_tab()?;
    assert!(session.tab.is_none());
    assert!(session.pages.is_empty());

    Ok(())
}

// ==========================================================================
// Test 4: Screenshot produces real bytes
// ==========================================================================
#[tokio::test(flavor = "multi_thread")]
async fn test_screenshot_real_bytes() -> Result<(), anyhow::Error> {
    let local_url = start_test_server().await;

    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    let session_id = manager.create_session(None)?;
    let session = manager.get_session_mut(&session_id).unwrap();

    let _ = session.navigate(&local_url, &browser).await?;
    let tab = session.tab.clone().unwrap();

    let png_bytes = tokio::task::block_in_place(|| {
        tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None, None, true,
        )
    })?;

    // Real PNG file starts with the magic bytes 0x89 0x50 0x4E 0x47
    assert!(png_bytes.len() > 100, "Screenshot should be more than 100 bytes");
    assert_eq!(png_bytes[0], 0x89, "PNG magic byte 1");
    assert_eq!(png_bytes[1], 0x50, "PNG magic byte 2 (P)");
    assert_eq!(png_bytes[2], 0x4E, "PNG magic byte 3 (N)");
    assert_eq!(png_bytes[3], 0x47, "PNG magic byte 4 (G)");

    Ok(())
}

// ==========================================================================
// Test 5: Trafilatura article extraction
// ==========================================================================
#[tokio::test]
async fn test_trafilatura_extraction() {
    let trafilatura = TrafilaturaExtractor::new();
    let html = r#"<!DOCTYPE html>
    <html>
    <head>
        <title>Breaking News Article</title>
        <meta name="description" content="A major event happened">
        <meta name="author" content="John Doe">
    </head>
    <body>
        <nav><a href="/">Home</a> <a href="/news">News</a></nav>
        <article>
            <h1>Breaking News: Major Event</h1>
            <p>This is the first paragraph of a real article about something important that happened in the world today.</p>
            <p>The second paragraph provides additional context and detail about the event, including quotes from officials.</p>
        </article>
        <footer>&copy; 2024 News Corp</footer>
        <script>console.log('analytics');</script>
    </body>
    </html>"#;

    let result = trafilatura.extract_content(html, "https://example.com/article");

    assert_eq!(result["title"], "Breaking News Article");
    assert_eq!(result["author"], "John Doe");
    assert_eq!(result["description"], "A major event happened");

    let content = result["content"].as_str().unwrap();
    assert!(content.contains("Major Event"), "Should contain article heading");
    assert!(content.contains("first paragraph"), "Should contain article body");
    // Should NOT contain nav or footer
    assert!(!content.contains("analytics"), "Should strip script content");

    let word_count = result["word_count"].as_u64().unwrap();
    assert!(word_count > 10, "Should have substantial word count");
}

// ==========================================================================
// Test 6: Crawl4AI bot detection on HTML
// ==========================================================================
#[tokio::test]
async fn test_crawl4ai_html_detection() {
    let detector = Crawl4AIDetector::new();

    // Test Cloudflare detection
    let cf_html = r#"<html><body><div class="cf-challenge-running">Checking your browser</div></body></html>"#;
    let result = detector.detect_protection("https://example.com", cf_html);
    assert_eq!(result["protection_level"], "high");
    assert!(result["detections"].as_array().unwrap().iter().any(|d| d["type"] == "cloudflare"));

    // Test reCAPTCHA detection
    let recaptcha_html = r#"<html><body><div class="g-recaptcha" data-sitekey="key123"></div></body></html>"#;
    let result = detector.detect_protection("https://example.com", recaptcha_html);
    assert_eq!(result["protection_level"], "high");

    // Test clean page (no protection)
    let clean_html = r#"<html><body><h1>Hello World</h1></body></html>"#;
    let result = detector.detect_protection("https://example.com", clean_html);
    assert_eq!(result["protection_level"], "none");
    assert_eq!(result["detection_count"], 0);
}

// ==========================================================================
// Test 7: Stealth engine generates real scripts
// ==========================================================================
#[tokio::test]
async fn test_stealth_real_scripts() {
    let stealth = PlaywrightStealth::new();

    // High-level stealth should include all techniques
    let result = stealth.apply_stealth("high");
    let script = result["script"].as_str().unwrap();
    let techniques = result["techniques_applied"].as_array().unwrap();

    assert!(script.contains("navigator"), "Script should override navigator");
    assert!(script.contains("webdriver"), "Script should hide webdriver");
    assert!(script.contains("WebGLRenderingContext"), "High stealth should spoof WebGL");
    assert!(techniques.len() >= 7, "High stealth should apply 7+ techniques, got {}", techniques.len());

    // User agent should be a real-looking string
    let ua = result["user_agent"].as_str().unwrap();
    assert!(ua.contains("Mozilla/5.0"), "UA should look real");

    // Low-level stealth should be minimal
    let low_result = stealth.apply_stealth("low");
    let low_techniques = low_result["techniques_applied"].as_array().unwrap();
    assert_eq!(low_techniques.len(), 1, "Low stealth should only have webdriver_hide");
}

// ==========================================================================
// Test 8: Stagehand multi-candidate ranking
// ==========================================================================
#[tokio::test]
async fn test_stagehand_multi_candidates() {
    let stagehand = StagehandEngine::new();
    let html = r#"<html><body>
        <input id="search-input" type="search" placeholder="Search..." />
        <input id="query-box" type="text" placeholder="Type your query" />
        <button id="search-btn">Search</button>
        <a href="/search">Advanced Search</a>
    </body></html>"#;

    let result = stagehand.find_element("search", html);
    let candidates = result["candidates"].as_array().unwrap();

    // Should return multiple candidates
    assert!(candidates.len() >= 2, "Should find at least 2 search-related elements, got {}", candidates.len());
    // Best match should be the search input
    assert_eq!(result["element"]["selector"], "#search-input");
}

// ==========================================================================
// Test 9: Profile persistence roundtrip
// ==========================================================================
#[tokio::test]
async fn test_profile_persistence_roundtrip() -> Result<(), anyhow::Error> {
    let mut db_path = std::env::temp_dir();
    db_path.push("test_nexusmcp_profiles.db");
    let db_path_str = db_path.to_str().unwrap().to_string();
    
    // Clean up from previous runs
    let _ = std::fs::remove_file(&db_path_str);

    let pm = ProfileManager::new(&db_path_str)?;

    // Create a profile
    let profile = pm.create_profile("test-agent", Some("socks5://proxy:1080"), "high")?;
    assert_eq!(profile.name, "test-agent");
    assert_eq!(profile.stealth_level, "high");

    // Load it back
    let loaded = pm.get_profile(&profile.id)?;
    assert!(loaded.is_some(), "Profile should be loadable from SQLite");
    let loaded = loaded.unwrap();
    assert_eq!(loaded.name, "test-agent");
    assert_eq!(loaded.proxy, Some("socks5://proxy:1080".to_string()));

    // Clean up
    let _ = std::fs::remove_file(&db_path_str);
    Ok(())
}

// ==========================================================================
// Test 10: Wait for element on real page
// ==========================================================================
#[tokio::test(flavor = "multi_thread")]
async fn test_wait_for_element() -> Result<(), anyhow::Error> {
    let local_url = start_test_server().await;

    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    let session_id = manager.create_session(None)?;
    let session = manager.get_session_mut(&session_id).unwrap();

    let _ = session.navigate(&local_url, &browser).await?;
    let tab = session.tab.clone().unwrap();

    // Wait for an element that already exists
    tokio::task::block_in_place(|| {
        tab.wait_for_element_with_custom_timeout("#test-header", std::time::Duration::from_secs(5))
    })?;

    // Verify we can find it
    let el = tokio::task::block_in_place(|| tab.find_element("#test-header"))?;
    let text = tokio::task::block_in_place(|| el.get_inner_text())?;
    assert_eq!(text, "Real Browser E2E Rendering Working");

    Ok(())
}
