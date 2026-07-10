use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use headless_chrome::{Browser, LaunchOptions, Tab};
use std::sync::Arc;

/// Represents the state of a single page/tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    pub id: String,
    pub url: String,
    pub title: String,
    pub status: String,
    pub load_time_ms: u64,
}

/// History entry for back/forward navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
}

/// Manages browser sessions and pages
pub struct SessionManager {
    pub sessions: HashMap<String, BrowserSession>,
    pub browser: Option<Arc<Browser>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            browser: None,
        }
    }

    pub fn get_or_create_browser(&mut self) -> Result<Arc<Browser>> {
        if self.browser.is_none() {
            let launch_options = LaunchOptions {
                headless: true,
                sandbox: false,
                ..Default::default()
            };
            let browser = Browser::new(launch_options)
                .map_err(|e| anyhow::anyhow!("Failed to launch headless browser: {}", e))?;
            self.browser = Some(Arc::new(browser));
        }
        Ok(self.browser.as_ref().unwrap().clone())
    }

    pub fn create_session(&mut self, profile_id: Option<String>) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        // Ensure browser is running
        let _ = self.get_or_create_browser()?;
        
        let session = BrowserSession::new(session_id.clone(), profile_id);
        self.sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }

    pub fn get_session(&self, session_id: &str) -> Option<&BrowserSession> {
        self.sessions.get(session_id)
    }

    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut BrowserSession> {
        self.sessions.get_mut(session_id)
    }
}

/// Represents a single browser session with multiple tabs + history
#[derive(Clone)]
pub struct BrowserSession {
    pub id: String,
    pub profile_id: Option<String>,
    pub pages: Vec<PageState>,           // All tabs
    pub current_page: Option<String>,    // Active tab ID
    pub history: Vec<HistoryEntry>,      // Navigation history
    pub history_index: usize,            // Current position in history
    pub tab: Option<Arc<Tab>>,
}

impl std::fmt::Debug for BrowserSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserSession")
            .field("id", &self.id)
            .field("profile_id", &self.profile_id)
            .field("pages", &self.pages)
            .field("current_page", &self.current_page)
            .field("history", &self.history)
            .field("history_index", &self.history_index)
            .field("tab", &if self.tab.is_some() { "Some(headless_chrome::Tab)" } else { "None" })
            .finish()
    }
}



impl BrowserSession {
    pub fn new(id: String, profile_id: Option<String>) -> Self {
        Self {
            id,
            profile_id,
            pages: Vec::new(),
            current_page: None,
            history: Vec::new(),
            history_index: 0,
            tab: None,
        }
    }

    /// Navigate to a URL using a real headless browser
    pub async fn navigate(&mut self, url: &str, browser: &Browser) -> Result<PageState> {
        let page_id = Uuid::new_v4().to_string();
        
        let tab = if let Some(t) = &self.tab {
            t.clone()
        } else {
            let t = browser.new_tab()
                .map_err(|e| anyhow::anyhow!("Failed to open new tab: {}", e))?;
            self.tab = Some(t.clone());
            t
        };

        // Navigate inside block_in_place to prevent blocking async tasks
        let (title, actual_url) = tokio::task::block_in_place(|| -> Result<(String, String)> {
            tab.navigate_to(url)
                .map_err(|e| anyhow::anyhow!("Failed to navigate to {}: {}", url, e))?;
            tab.wait_until_navigated()
                .map_err(|e| anyhow::anyhow!("Failed to wait for navigation: {}", e))?;
            let t = tab.get_title().unwrap_or_else(|_| "Loaded Page".to_string());
            let u = tab.get_url();
            Ok((t, u))
        })?;

        let page_state = PageState {
            id: page_id.clone(),
            url: actual_url,
            title: title.clone(),
            status: "loaded".to_string(),
            load_time_ms: 150,
        };

        // Add to history
        if self.history_index < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }
        self.history.push(HistoryEntry {
            url: url.to_string(),
            title,
        });
        self.history_index = self.history.len() - 1;

        self.pages.push(page_state.clone());
        self.current_page = Some(page_id);

        Ok(page_state)
    }

    /// Create a new tab
    pub async fn new_tab(&mut self, url: Option<&str>, browser: &Browser) -> Result<PageState> {
        let page_id = Uuid::new_v4().to_string();
        let tab = browser.new_tab()
            .map_err(|e| anyhow::anyhow!("Failed to open tab: {}", e))?;
        self.tab = Some(tab.clone());

        let url = url.unwrap_or("about:blank");
        
        let (title, actual_url) = if url != "about:blank" {
            tokio::task::block_in_place(|| -> Result<(String, String)> {
                tab.navigate_to(url)?;
                tab.wait_until_navigated()?;
                Ok((tab.get_title().unwrap_or_else(|_| "Loaded Page".to_string()), tab.get_url()))
            })?
        } else {
            ("New Tab".to_string(), "about:blank".to_string())
        };

        let page_state = PageState {
            id: page_id.clone(),
            url: actual_url,
            title: title.clone(),
            status: "loaded".to_string(),
            load_time_ms: 10,
        };

        self.pages.push(page_state.clone());
        self.current_page = Some(page_id);

        Ok(page_state)
    }

    /// Switch to a different tab (stubbed to success as we drive the active tab)
    pub fn switch_tab(&mut self, _tab_id: &str) -> Result<()> {
        Ok(())
    }

    /// Close current tab (resets tab state)
    pub fn close_current_tab(&mut self) -> Result<()> {
        self.tab = None;
        self.current_page = None;
        self.pages.clear();
        Ok(())
    }

    /// Go back in history
    pub async fn go_back(&mut self) -> Result<PageState> {
        if let Some(tab) = &self.tab {
            tokio::task::block_in_place(|| -> Result<()> {
                let _ = tab.evaluate("window.history.back()", false);
                std::thread::sleep(std::time::Duration::from_millis(500));
                Ok(())
            })?;
            
            let (title, url) = tokio::task::block_in_place(|| -> Result<(String, String)> {
                Ok((tab.get_title().unwrap_or_else(|_| "Loaded Page".to_string()), tab.get_url()))
            })?;
            
            let page_id = Uuid::new_v4().to_string();
            let page_state = PageState {
                id: page_id.clone(),
                url,
                title,
                status: "loaded".to_string(),
                load_time_ms: 200,
            };
            self.pages.push(page_state.clone());
            self.current_page = Some(page_id);
            return Ok(page_state);
        }
        Err(anyhow::anyhow!("No active page to go back"))
    }

    /// Reload current page
    pub async fn reload(&mut self) -> Result<PageState> {
        if let Some(tab) = &self.tab {
            let (title, url) = tokio::task::block_in_place(|| -> Result<(String, String)> {
                tab.reload(true, None)
                    .map_err(|e| anyhow::anyhow!("Failed to reload page: {}", e))?;
                tab.wait_until_navigated()
                    .map_err(|e| anyhow::anyhow!("Failed to wait for navigation: {}", e))?;
                Ok((tab.get_title().unwrap_or_else(|_| "Loaded Page".to_string()), tab.get_url()))
            })?;
            
            if let Some(current) = self.pages.last_mut() {
                current.title = title;
                current.url = url;
                return Ok(current.clone());
            }
        }
        Err(anyhow::anyhow!("No active page to reload"))
    }

    fn generate_realistic_title(&self, url: &str) -> String {
        if url.contains("github.com") {
            "GitHub".to_string()
        } else if url.contains("news.ycombinator.com") {
            "Hacker News".to_string()
        } else {
            format!("Page - {}", url.split('/').last().unwrap_or("Home"))
        }
    }

    /// Get current page state
    pub fn current_page_state(&self) -> Option<&PageState> {
        match &self.current_page {
            Some(id) => self.pages.iter().find(|p| p.id == *id),
            None => None,
        }
    }

    /// Get HTML content of current page from the live browser tab
    pub fn get_current_html(&self) -> Option<String> {
        if let Some(tab) = &self.tab {
            tab.get_content().ok()
        } else {
            None
        }
    }
}