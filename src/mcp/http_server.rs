use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::mcp::tools::ToolRegistry;


pub struct AppState {
    registry: Arc<Mutex<ToolRegistry>>,
}

pub async fn start_http_server(port: u16, _stealth: bool) -> anyhow::Result<()> {
    let registry = ToolRegistry::new();

    let state = Arc::new(AppState {
        registry: Arc::new(Mutex::new(registry)),
    });

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(|origin, _| {
            let host = origin.to_str().unwrap_or("");
            host.starts_with("http://localhost:") 
                || host.starts_with("http://127.0.0.1:") 
                || host.starts_with("https://localhost:") 
                || host.starts_with("https://127.0.0.1:")
        }))
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))           // Prometheus metrics
        .route("/mcp/tools", get(list_tools))
        .route("/mcp/call", post(call_tool))
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("NexusMCP HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "nexusmcp",
        "version": "0.1.0"
    }))
}

// Prometheus metrics endpoint
async fn metrics_handler() -> String {
    use prometheus::Encoder;
    
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        return format!("# Error encoding metrics: {}\n", e);
    }
    
    String::from_utf8(buffer).unwrap_or_else(|_| "# Error converting metrics to string\n".to_string())
}

async fn list_tools(State(state): State<Arc<AppState>>) -> Json<Value> {
    let registry = state.registry.lock().await;
    let tools = registry.list_tools();
    Json(json!({ "tools": tools }))
}

async fn call_tool(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let arguments = payload.get("arguments").cloned().unwrap_or(json!({}));

    let mut registry = state.registry.lock().await;

    match registry.call_tool(name, arguments).await {
        Ok(result) => Json(json!({
            "success": true,
            "result": result
        })),
        Err(e) => Json(json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}