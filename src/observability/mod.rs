// Enterprise Observability Module
use prometheus::{Histogram, IntCounter, IntGauge, Opts};
use std::sync::LazyLock;

pub static NAVIGATION_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::new("nexusmcp_navigations_total", "Total number of navigations").unwrap()
});

pub static ACTIVE_SESSIONS: LazyLock<IntGauge> = LazyLock::new(|| {
    IntGauge::new(
        "nexusmcp_active_sessions",
        "Current active browser sessions",
    )
    .unwrap()
});

pub static PAGE_LOAD_TIME: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::with_opts(
        Opts::new(
            "nexusmcp_page_load_time_seconds",
            "Page load time in seconds",
        )
        .into(),
    )
    .unwrap()
});

pub fn init_metrics() {
    let _ = prometheus::register(Box::new(NAVIGATION_COUNTER.clone()));
    let _ = prometheus::register(Box::new(ACTIVE_SESSIONS.clone()));
    let _ = prometheus::register(Box::new(PAGE_LOAD_TIME.clone()));
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
