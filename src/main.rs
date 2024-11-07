mod config;
pub mod handlers;
mod models;

use axum::{routing::get, Router};
use models::Node;
use std::sync::{Arc, Mutex};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Set default logging to debug level for your crate
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::FULL), // This will log enter/exit of spans
        )
        .init();

    let config = config::load_config("config.yml");
    let nodes: Vec<Node> = config
        .nodes
        .into_iter()
        .map(|node_config| Node {
            id: node_config.id,
            ip: node_config.ip,
            port: node_config.port,
        })
        .collect();
    let nodes = Arc::new(Mutex::new(nodes));

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR));

    let app = Router::new()
        .layer(trace_layer)
        .route("/public", get(handlers::blockchain::public::index))
        .route(
            "/nodes",
            get(handlers::blockchain::node::get_all_nodes)
                .post(handlers::blockchain::node::register_node),
        )
        // query params will be used to filter the chain
        .route(
            "/chain",
            get(handlers::blockchain::chain::get_chain)
                .post(handlers::blockchain::chain::add_transaction),
        )
        /*
        .route("/peer/inventory", get(handlers::blockchain::peer::get_inventory))
        .route("/peer/getblocks", post(handlers::blockchain::peer::get_blocks))
        .route("/peer/getdata", post(handlers::blockchain::peer::get_data))
        .route("/peer/sync", post(handlers::blockchain::peer::sync_node))
        .route("/peer/status", get(handlers::blockchain::peer::get_node_status))
             */
        .with_state(nodes)
        .route_layer(TraceLayer::new_for_http());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.address, config.server.port))
            .await
            .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
