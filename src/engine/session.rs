use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use headless_chrome::{Browser, LaunchOptions, Tab};

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
            // Read sandbox settings from config (secure by default, opt-out via env vars)
            let no_sandbox =
                std::env::var("NEXUS_NO_SANDBOX").is_ok() || std::env::var("NO_SANDBOX").is_ok();

            let launch_options = LaunchOptions {
                headless: true,
                sandbox: !no_sandbox,
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

        let mut proxy = None;
        let mut stealth_level = "high".to_string();

        if let Some(pid) = &profile_id {
            let db_path = std::env::var("NEXUS_DB_PATH")
                .unwrap_or_else(|_| "nexusmcp_profiles.db".to_string());
            if let Ok(pm) = crate::session::ProfileManager::new(&db_path) {
                if let Ok(Some(profile)) = pm.get_profile(pid) {
                    proxy = profile.proxy;
                    stealth_level = profile.stealth_level;
                }
            }
        }

        let session = BrowserSession::new(session_id.clone(), profile_id, proxy, stealth_level);
        self.sessions.insert(session_id.clone(), session);
        crate::observability::set_active_sessions(self.sessions.len() as i64);
        Ok(session_id)
    }

    pub fn remove_session(&mut self, session_id: &str) -> Option<BrowserSession> {
        let removed = self.sessions.remove(session_id);
        if removed.is_some() {
            crate::observability::set_active_sessions(self.sessions.len() as i64);
        }
        removed
    }

    pub fn get_session(&self, session_id: &str) -> Option<&BrowserSession> {
        self.sessions.get(session_id)
    }

    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut BrowserSession> {
        self.sessions.get_mut(session_id)
    }

    pub fn shutdown(&mut self) {
        if let Some(browser) = self.browser.take() {
            // Drop browser which kills Chrome process
            drop(browser);
        }
        self.sessions.clear();
        crate::observability::set_active_sessions(0);
    }
}

/// Represents a single browser session with multiple tabs + history
pub struct BrowserSession {
    pub id: String,
    pub profile_id: Option<String>,
    pub proxy: Option<String>,
    pub stealth_level: String,
    pub pages: Vec<PageState>,           // All tabs metadata
    pub current_page: Option<String>,    // Active page metadata ID
    pub history: Vec<HistoryEntry>,      // Navigation history
    pub history_index: usize,            // Current position in history
    pub tab: Option<Arc<Tab>>,           // Currently active tab
    pub tabs: HashMap<String, Arc<Tab>>, // Map of page state ID -> Tab
}

impl std::fmt::Debug for BrowserSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserSession")
            .field("id", &self.id)
            .field("profile_id", &self.profile_id)
            .field("proxy", &self.proxy)
            .field("stealth_level", &self.stealth_level)
            .field("pages", &self.pages)
            .field("current_page", &self.current_page)
            .field("history", &self.history)
            .field("history_index", &self.history_index)
            .field(
                "tab",
                &if self.tab.is_some() {
                    "Some(headless_chrome::Tab)"
                } else {
                    "None"
                },
            )
            .field("tabs_count", &self.tabs.len())
            .finish()
    }
}

impl BrowserSession {
    pub fn new(
        id: String,
        profile_id: Option<String>,
        proxy: Option<String>,
        stealth_level: String,
    ) -> Self {
        Self {
            id,
            profile_id,
            proxy,
            stealth_level,
            pages: Vec::new(),
            current_page: None,
            history: Vec::new(),
            history_index: 0,
            tab: None,
            tabs: HashMap::new(),
        }
    }

    /// Navigate to a URL using a real headless browser
    pub async fn navigate(&mut self, url: &str, browser: &Browser) -> Result<PageState> {
        let tab = if let Some(t) = &self.tab {
            t.clone()
        } else {
            let t = browser
                .new_tab()
                .map_err(|e| anyhow::anyhow!("Failed to open new tab: {}", e))?;
            let tab_arc = t;
            self.tab = Some(tab_arc.clone());
            let page_id = Uuid::new_v4().to_string();
            self.tabs.insert(page_id.clone(), tab_arc.clone());
            self.current_page = Some(page_id);
            tab_arc
        };

        // Determine or initialize active page_id
        let page_id = self.current_page.clone().unwrap_or_else(|| {
            let id = Uuid::new_v4().to_string();
            self.tabs.insert(id.clone(), tab.clone());
            self.current_page = Some(id.clone());
            id
        });

        let start = std::time::Instant::now();
        let tab_clone = tab.clone();
        let url_clone = url.to_string();

        // Offload blocking navigation to threadpool
        tokio::task::spawn_blocking(move || -> Result<()> {
            tab_clone
                .navigate_to(&url_clone)
                .map_err(|e| anyhow::anyhow!("Failed to navigate to {}: {}", url_clone, e))?;
            tab_clone
                .wait_until_navigated()
                .map_err(|e| anyhow::anyhow!("Failed to wait for navigation: {}", e))?;
            Ok(())
        })
        .await??;

        // Allow Chromium V8 context to initialize and parse headers without blocking thread
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let tab_clone = tab.clone();
        let mut title =
            tokio::task::spawn_blocking(move || tab_clone.get_title().unwrap_or_default()).await?;

        if title.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            let tab_clone = tab.clone();
            title = tokio::task::spawn_blocking(move || {
                tab_clone
                    .get_title()
                    .unwrap_or_else(|_| "Loaded Page".to_string())
            })
            .await?;
        }

        let tab_clone = tab.clone();
        let actual_url = tokio::task::spawn_blocking(move || tab_clone.get_url()).await?;

        let duration = start.elapsed().as_secs_f64();
        crate::observability::record_navigation();
        crate::observability::record_page_load_time(duration);

        let page_state = PageState {
            id: page_id.clone(),
            url: actual_url,
            title: title.clone(),
            status: "loaded".to_string(),
            load_time_ms: (duration * 1000.0) as u64,
        };

        // Add to navigation history
        if self.history_index < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }
        self.history.push(HistoryEntry {
            url: url.to_string(),
            title,
        });
        self.history_index = self.history.len() - 1;

        // Update pages metadata list
        if let Some(pos) = self.pages.iter().position(|p| p.id == page_id) {
            self.pages[pos] = page_state.clone();
        } else {
            self.pages.push(page_state.clone());
        }
        self.current_page = Some(page_id);

        Ok(page_state)
    }

    pub async fn new_tab(&mut self, url: Option<&str>, browser: &Browser) -> Result<PageState> {
        let page_id = Uuid::new_v4().to_string();
        let tab = browser
            .new_tab()
            .map_err(|e| anyhow::anyhow!("Failed to open tab: {}", e))?;
        let tab_arc = tab;
        self.tab = Some(tab_arc.clone());
        self.tabs.insert(page_id.clone(), tab_arc.clone());
        self.current_page = Some(page_id.clone());

        let url = url.unwrap_or("about:blank");

        let (title, actual_url) = if url != "about:blank" {
            let tab_clone = tab_arc.clone();
            let url_clone = url.to_string();
            tokio::task::spawn_blocking(move || -> Result<(String, String)> {
                tab_clone.navigate_to(&url_clone)?;
                tab_clone.wait_until_navigated()?;
                let t = tab_clone
                    .get_title()
                    .unwrap_or_else(|_| "Loaded Page".to_string());
                let u = tab_clone.get_url();
                Ok((t, u))
            })
            .await??
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

        Ok(page_state)
    }

    /// Switch to a different tab using its ID
    pub fn switch_tab(&mut self, tab_id: &str) -> Result<()> {
        if let Some(tab) = self.tabs.get(tab_id) {
            self.tab = Some(tab.clone());
            self.current_page = Some(tab_id.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Tab ID '{}' not found in session", tab_id))
        }
    }

    /// Close current tab using the real target close method
    pub async fn close_current_tab(&mut self) -> Result<()> {
        if let Some(page_id) = &self.current_page {
            if let Some(tab) = self.tabs.remove(page_id) {
                // Terminate target tab context to avoid resource leaks
                let _ = tokio::task::spawn_blocking(move || tab.close(true)).await;
            }
            if let Some(pos) = self.pages.iter().position(|p| &p.id == page_id) {
                self.pages.remove(pos);
            }
        }

        // Re-align active references
        if let Some((next_id, next_tab)) = self.tabs.iter().next() {
            self.current_page = Some(next_id.clone());
            self.tab = Some(next_tab.clone());
        } else {
            self.current_page = None;
            self.tab = None;
            self.pages.clear();
        }
        Ok(())
    }

    /// Go back in history (properly updating state and decrementing history index)
    pub async fn go_back(&mut self) -> Result<PageState> {
        if self.history_index > 0 {
            self.history_index -= 1;
            let entry = self.history[self.history_index].clone();
            tracing::info!("Navigating back to: {}", entry.url);
            if let Some(tab) = &self.tab {
                let tab_clone = tab.clone();
                tokio::task::spawn_blocking(move || -> Result<()> {
                    let _ = tab_clone.evaluate("window.history.back()", false);
                    Ok(())
                })
                .await??;

                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let tab_clone = tab.clone();
                let (title, url) =
                    tokio::task::spawn_blocking(move || -> Result<(String, String)> {
                        let t = tab_clone
                            .get_title()
                            .unwrap_or_else(|_| "Loaded Page".to_string());
                        let u = tab_clone.get_url();
                        Ok((t, u))
                    })
                    .await??;

                let page_id = self
                    .current_page
                    .clone()
                    .unwrap_or_else(|| Uuid::new_v4().to_string());
                let page_state = PageState {
                    id: page_id.clone(),
                    url,
                    title,
                    status: "loaded".to_string(),
                    load_time_ms: 200,
                };

                if let Some(pos) = self.pages.iter().position(|p| p.id == page_id) {
                    self.pages[pos] = page_state.clone();
                } else {
                    self.pages.push(page_state.clone());
                }
                self.current_page = Some(page_id);
                return Ok(page_state);
            }
        }
        Err(anyhow::anyhow!("No back history available"))
    }

    /// Go forward in history
    pub async fn go_forward(&mut self) -> Result<PageState> {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            let entry = self.history[self.history_index].clone();
            tracing::info!("Navigating forward to: {}", entry.url);
            if let Some(tab) = &self.tab {
                let tab_clone = tab.clone();
                tokio::task::spawn_blocking(move || -> Result<()> {
                    let _ = tab_clone.evaluate("window.history.forward()", false);
                    Ok(())
                })
                .await??;

                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let tab_clone = tab.clone();
                let (title, url) =
                    tokio::task::spawn_blocking(move || -> Result<(String, String)> {
                        let t = tab_clone
                            .get_title()
                            .unwrap_or_else(|_| "Loaded Page".to_string());
                        let u = tab_clone.get_url();
                        Ok((t, u))
                    })
                    .await??;

                let page_id = self
                    .current_page
                    .clone()
                    .unwrap_or_else(|| Uuid::new_v4().to_string());
                let page_state = PageState {
                    id: page_id.clone(),
                    url,
                    title,
                    status: "loaded".to_string(),
                    load_time_ms: 200,
                };

                if let Some(pos) = self.pages.iter().position(|p| p.id == page_id) {
                    self.pages[pos] = page_state.clone();
                } else {
                    self.pages.push(page_state.clone());
                }
                self.current_page = Some(page_id);
                return Ok(page_state);
            }
        }
        Err(anyhow::anyhow!("No forward history available"))
    }

    /// Reload current page
    pub async fn reload(&mut self) -> Result<PageState> {
        if let Some(tab) = &self.tab {
            let tab_clone = tab.clone();
            let (title, url) = tokio::task::spawn_blocking(move || -> Result<(String, String)> {
                tab_clone
                    .reload(true, None)
                    .map_err(|e| anyhow::anyhow!("Failed to reload page: {}", e))?;
                tab_clone
                    .wait_until_navigated()
                    .map_err(|e| anyhow::anyhow!("Failed to wait for navigation: {}", e))?;
                let t = tab_clone
                    .get_title()
                    .unwrap_or_else(|_| "Loaded Page".to_string());
                let u = tab_clone.get_url();
                Ok((t, u))
            })
            .await??;

            if let Some(current) = self.pages.last_mut() {
                current.title = title;
                current.url = url;
                return Ok(current.clone());
            }
        }
        Err(anyhow::anyhow!("No active page to reload"))
    }

    /// Get current page state
    pub fn current_page_state(&self) -> Option<&PageState> {
        match &self.current_page {
            Some(id) => self.pages.iter().find(|p| p.id == *id),
            None => None,
        }
    }

    /// Get HTML content of current page from the live browser tab
    pub async fn get_current_html(&self) -> Option<String> {
        if let Some(tab) = &self.tab {
            let tab = tab.clone();
            tokio::task::spawn_blocking(move || tab.get_content().ok())
                .await
                .ok()
                .flatten()
        } else {
            None
        }
    }
}
