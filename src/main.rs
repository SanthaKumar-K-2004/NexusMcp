use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use nexusmcp::{mcp, observability};

#[derive(Parser)]
#[command(name = "nexusmcp")]
#[command(about = "NexusMCP - Enterprise-grade lightweight browser MCP server")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the MCP server (stdio mode) - for Claude/Cursor
    Mcp {
        /// Enable stealth mode
        #[arg(long)]
        stealth: bool,

        /// Proxy URL (http:// or socks5://)
        #[arg(long)]
        proxy: Option<String>,
    },

    /// Start HTTP MCP server (for remote access / web clients)
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,

        #[arg(long)]
        stealth: bool,
    },

    /// Show version and build info
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nexusmcp=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize metrics
    observability::init_metrics();

    let cli = Cli::parse();

    match cli.command {
        Commands::Mcp { stealth, proxy } => {
            tracing::info!("Starting NexusMCP in MCP stdio mode");
            if stealth {
                tracing::info!("Stealth mode enabled");
            }
            if let Some(p) = &proxy {
                tracing::info!("Using proxy: {}", p);
            }

            // Start the MCP server
            mcp::start_mcp_server(stealth, proxy).await?;
        }
        Commands::Serve { port, stealth } => {
            tracing::info!("Starting HTTP MCP server on port {}", port);
            if stealth {
                tracing::info!("Stealth mode enabled");
            }
            mcp::http_server::start_http_server(port, stealth).await?;
        }
        Commands::Version => {
            println!("NexusMCP v0.1.0");
            println!("Built with Rust + Obscura (planned)");
        }
    }

    Ok(())
}
