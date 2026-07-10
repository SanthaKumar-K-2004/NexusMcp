use nexusmcp::engine::session::SessionManager;
use nexusmcp::hidden_gems::stagehand::StagehandEngine;
use nexusmcp::hidden_gems::firecrawl_extraction::FirecrawlExtractor;
use nexusmcp::extraction::AdvancedExtractor;
use serde_json::json;
use axum::{routing::get, response::Html, Router};

#[tokio::test(flavor = "multi_thread")]
async fn test_real_browser_and_hidden_gems_flow() -> Result<(), anyhow::Error> {
    // 1. Start a local HTTP server serving a test web page
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let local_port = listener.local_addr()?.port();
    let local_url = format!("http://127.0.0.1:{}", local_port);

    let app = Router::new().route("/", get(|| async {
        Html(r#"<!DOCTYPE html>
        <html>
        <head><title>NexusMCP Integration Test Page</title></head>
        <body>
            <h1 id="test-header">Real Browser E2E Rendering Working</h1>
            <p>Welcome to the NexusMCP test suite.</p>
            <input id="user-email" type="email" placeholder="enter email address" />
            <button id="submit-btn" type="submit" onclick="document.getElementById('test-header').innerText = 'Action Performed'">Submit Email</button>
            
            <div class="contact-details">
                <p>Support email: support@nexusmcp.dev</p>
                <p>Enterprise license price: $99.99</p>
            </div>
        </body>
        </html>"#)
    }));

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Wait for the local test server to start listening
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // 2. Initialize SessionManager and Browser
    let mut manager = SessionManager::new();
    let browser = manager.get_or_create_browser()?;
    
    let session_id = manager.create_session(None)?;
    let session = manager.get_session_mut(&session_id).unwrap();

    // 3. Test REAL Browser Navigation
    let page = session.navigate(&local_url, &browser).await?;
    println!("DEBUG: page.title='{}', page.url='{}', page.status='{}'", page.title, page.url, page.status);
    let tab = session.tab.clone().unwrap();
    println!("DEBUG: tab.get_url()='{}'", tab.get_url());
    println!("DEBUG: tab.get_title()='{}'", tab.get_title().unwrap_or_default());
    assert_eq!(page.title, "NexusMCP Integration Test Page");
    
    let html = session.get_current_html().unwrap();
    assert!(html.contains("Real Browser E2E Rendering Working"));

    // 4. Test REAL JS Evaluation via CDP
    let tab = session.tab.clone().unwrap();
    let eval_res = tokio::task::block_in_place(|| {
        tab.evaluate("document.getElementById('test-header').innerText", false)
    })?;
    assert_eq!(
        eval_res.value.unwrap().as_str().unwrap(),
        "Real Browser E2E Rendering Working"
    );

    // 5. Test DOM Interaction (clicking button via CDP)
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

    // 6. Test Stagehand Element Locator Scoring Engine
    let stagehand = StagehandEngine::new();
    let locator_res = stagehand.find_element("submit email button", &html);
    let matched_selector = locator_res["element"]["selector"].as_str().unwrap();
    assert_eq!(matched_selector, "#submit-btn"); // scores best on type="submit" and text "Submit"

    // 7. Test Firecrawl-style Structured Extraction (Regex + Scraper)
    let firecrawl = FirecrawlExtractor::new();
    let schema = json!({
        "title": {},
        "emails": {},
        "prices": {},
        "links_count": {}
    });
    let extraction = firecrawl.extract_with_schema(&html, schema);
    println!("DEBUG: extraction='{}'", extraction);
    let data = &extraction["extracted_data"];
    
    assert_eq!(data["title"], "NexusMCP Integration Test Page");
    assert!(data["emails"].as_array().unwrap().contains(&json!("support@nexusmcp.dev")));
    assert!(data["prices"].as_array().unwrap().contains(&json!("$99.99")));

    // 8. Test Markdown Extraction
    let extractor = AdvancedExtractor::new();
    let markdown = extractor.html_to_markdown(&html, &local_url)?;
    assert!(markdown.contains("Real Browser E2E Rendering Working"));
    assert!(markdown.contains("Support email: support@nexusmcp.dev"));

    Ok(())
}
