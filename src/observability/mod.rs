// Enterprise Observability Module
use prometheus::{IntCounter, IntGauge, Histogram, Registry, Opts};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NAVIGATION_COUNTER: IntCounter = IntCounter::new(
        "nexusmcp_navigations_total", 
        "Total number of navigations"
    ).unwrap();

    pub static ref ACTIVE_SESSIONS: IntGauge = IntGauge::new(
        "nexusmcp_active_sessions", 
        "Current active browser sessions"
    ).unwrap();

    pub static ref PAGE_LOAD_TIME: Histogram = Histogram::with_opts(
        Opts::new("nexusmcp_page_load_time_seconds", "Page load time in seconds")
            .into()
    ).unwrap();

    pub static ref REGISTRY: Registry = Registry::new();
}

pub fn init_metrics() {
    let _ = REGISTRY.register(Box::new(NAVIGATION_COUNTER.clone()));
    let _ = REGISTRY.register(Box::new(ACTIVE_SESSIONS.clone()));
    let _ = REGISTRY.register(Box::new(PAGE_LOAD_TIME.clone()));
}

pub fn record_navigation() {
    NAVIGATION_COUNTER.inc();
}

pub fn record_page_load_time(seconds: f64) {
    PAGE_LOAD_TIME.observe(seconds);
}

pub fn set_active_sessions(count: i64) {
    ACTIVE_SESSIONS.set(count);
}