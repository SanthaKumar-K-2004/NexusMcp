use serde_json::{json, Value};
use std::collections::HashMap;
use anyhow::Result;
use std::sync::Arc;
use crate::engine::SessionManager;
use crate::extraction::AdvancedExtractor;
use crate::agent::AgentEnhancer;
use crate::session::ProfileManager;
use crate::hidden_gems::{StagehandEngine, Crawl4AIDetector, TrafilaturaExtractor, PlaywrightStealth, VectorMemory, FirecrawlExtractor};

pub mod navigation;
pub mod extraction;
pub mod stealth;
pub mod research;
pub mod session;
pub mod error_recovery;
pub mod profile_persistence;
pub mod agent_superpowers;
pub mod hidden_gem_tools;
pub mod enterprise;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
    pub session_manager: SessionManager,
    pub extractor: AdvancedExtractor,
    pub agent: AgentEnhancer,
    pub memory: Vec<String>,
    pub profile_manager: ProfileManager,

    // Hidden Gems Integration
    pub stagehand: StagehandEngine,
    pub crawl4ai: Crawl4AIDetector,
    pub trafilatura: TrafilaturaExtractor,
    pub stealth_engine: PlaywrightStealth,
    pub vector_memory: VectorMemory,
    pub firecrawl: FirecrawlExtractor,
}

#[async_trait::async_trait]
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;

    async fn call(&self, arguments: Value) -> Result<String>;
}

impl ToolRegistry {
    pub fn new() -> Self {
        let db_path = std::env::var("NEXUS_DB_PATH").unwrap_or_else(|_| "nexusmcp_profiles.db".to_string());
        let profile_manager = ProfileManager::new(&db_path).expect("Failed to initialize ProfileManager");
        Self {
            tools: HashMap::new(),
            session_manager: SessionManager::new(),
            extractor: AdvancedExtractor::new(),
            agent: AgentEnhancer::new(),
            memory: Vec::new(),
            profile_manager,

            // Hidden Gems
            stagehand: StagehandEngine::new(),
            crawl4ai: Crawl4AIDetector::new(),
            trafilatura: TrafilaturaExtractor::new(),
            stealth_engine: PlaywrightStealth::new(),
            vector_memory: VectorMemory::new(None),
            firecrawl: FirecrawlExtractor::new(),
        }
    }

    pub fn register_tools(&mut self, stealth_enabled: bool, _proxy: Option<String>) {
        // Navigation tools
        self.register(Box::new(navigation::BrowserNavigateTool::new()));
        self.register(Box::new(navigation::BrowserClickTool::new()));
        self.register(Box::new(navigation::BrowserEvaluateTool::new()));
        self.register(Box::new(navigation::BrowserFillFormTool::new()));
        self.register(Box::new(navigation::BrowserBackTool::new()));
        self.register(Box::new(navigation::BrowserReloadTool::new()));
        self.register(Box::new(navigation::BrowserWaitForTool::new()));

        // Tab management tools
        self.register(Box::new(navigation::BrowserTabNewTool::new()));
        self.register(Box::new(navigation::BrowserTabSwitchTool::new()));
        self.register(Box::new(navigation::BrowserTabCloseTool::new()));

        // Extraction tools
        self.register(Box::new(extraction::BrowserMarkdownTool::new()));
        self.register(Box::new(extraction::BrowserExtractTool::new()));
        self.register(Box::new(extraction::BrowserLinksTool::new()));
        self.register(Box::new(extraction::BrowserPdfTool::new()));
        self.register(Box::new(extraction::BrowserScreenshotTool::new()));

        // Stealth tools
        if stealth_enabled {
            self.register(Box::new(stealth::BrowserStealthRotateTool::new()));
        }

        // Research tools
        self.register(Box::new(research::BrowserResearchTool::new()));

        // Session tools
        self.register(Box::new(session::BrowserCreateProfileTool::new()));

        // Error Recovery
        self.register(Box::new(error_recovery::BrowserSmartRetryTool::new()));

        // Profile Persistence
        self.register(Box::new(profile_persistence::BrowserLoadProfileTool::new()));

        // AI Agent tools
        self.register(Box::new(agent_superpowers::BrowserObserveTool::new()));
        self.register(Box::new(agent_superpowers::BrowserActTool::new()));

        // Hidden Gems Tools
        self.register(Box::new(hidden_gem_tools::BrowserFindElementTool::new()));
        self.register(Box::new(hidden_gem_tools::BrowserTrafilaturaTool::new()));
        self.register(Box::new(hidden_gem_tools::BrowserFirecrawlExtractTool::new()));

        // Enterprise Tools
        self.register(Box::new(enterprise::BrowserHandleCaptchaTool::new()));
        self.register(Box::new(enterprise::BrowserHealthCheckTool::new()));

        tracing::info!("Registered {} tools (all modularized and real)", self.tools.len());
    }

    fn register(&mut self, tool: Box<dyn Tool + Send + Sync>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    pub fn list_tools(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect()
    }

    // ==================== HELPERS ====================

    pub fn get_active_tab(&self) -> Option<Arc<headless_chrome::Tab>> {
        self.session_manager.sessions.values()
            .find_map(|session| session.tab.clone())
    }

    pub fn get_active_session_id(&self) -> Option<String> {
        self.session_manager.sessions.keys().next().cloned()
    }

    pub fn get_active_html(&self) -> Result<String> {
        let session_id = self.get_active_session_id()
            .ok_or_else(|| anyhow::anyhow!("No active session"))?;
        let session = self.session_manager.get_session(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
        session.get_current_html()
            .ok_or_else(|| anyhow::anyhow!("No page loaded — navigate to a URL first"))
    }

    // ==================== DISPATCH ====================

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        match name {
            // Navigation
            "browser_navigate" => navigation::handle_navigate(self, arguments).await,
            "browser_evaluate" => navigation::handle_evaluate(self, arguments).await,
            "browser_click" => navigation::handle_click(self, arguments).await,
            "browser_fill_form" => navigation::handle_fill_form(self, arguments).await,
            "browser_wait_for" => navigation::handle_wait_for(self, arguments).await,
            "browser_back" => navigation::handle_back(self, arguments).await,
            "browser_reload" => navigation::handle_reload(self, arguments).await,

            // Tab management
            "browser_tab_new" => navigation::handle_tab_new(self, arguments).await,
            "browser_tab_switch" => navigation::handle_tab_switch(self, arguments).await,
            "browser_tab_close" => navigation::handle_tab_close(self, arguments).await,

            // Extraction
            "browser_markdown" => extraction::handle_markdown(self, arguments).await,
            "browser_screenshot" => extraction::handle_screenshot(self, arguments).await,
            "browser_pdf" => extraction::handle_pdf(self, arguments).await,
            "browser_links" => extraction::handle_links(self, arguments).await,
            "browser_extract" | "browser_firecrawl_extract" => extraction::handle_extract(self, arguments).await,

            // Hidden gems
            "browser_find_element" => hidden_gem_tools::handle_find_element(self, arguments).await,
            "browser_trafilatura" => hidden_gem_tools::handle_trafilatura(self, arguments).await,

            // Stealth
            "browser_stealth_rotate" => stealth::handle_stealth_rotate(self, arguments).await,

            // Profiles
            "browser_create_profile" => session::handle_create_profile(self, arguments).await,
            "browser_load_profile" => profile_persistence::handle_load_profile(self, arguments).await,

            // Agent superpowers
            "browser_observe" => agent_superpowers::handle_observe(self, arguments).await,
            "browser_act" => agent_superpowers::handle_act(self, arguments).await,

            // Enterprise
            "browser_handle_captcha" => enterprise::handle_captcha(self, arguments).await,
            "browser_health_check" => enterprise::handle_health_check(self, arguments).await,
            "browser_smart_retry" => error_recovery::handle_smart_retry(self, arguments).await,

            // Research
            "browser_research" => {
                if let Some(tool) = self.tools.get(name) {
                    tool.call(arguments).await
                } else {
                    Err(anyhow::anyhow!("Tool '{}' not found", name))
                }
            }

            _ => Err(anyhow::anyhow!("Tool '{}' not found", name)),
        }
    }
}