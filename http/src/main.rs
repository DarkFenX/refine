use std::{env, sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract, http, middleware,
    routing::{delete, get, patch, post},
    Router, ServiceExt,
};
use tower::Layer;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::Span;

use crate::{settings::HSettings, state::HInnerAppState};

mod bridge;
mod cmd;
mod handlers;
mod info;
mod logging;
mod settings;
mod shared;
mod state;
mod util;

#[tokio::main]
async fn main() {
    // Settings
    let config_path = env::args().nth(1);
    let settings = HSettings::new(config_path).unwrap();
    // Logging
    let _log_guard = logging::setup(settings.log.folder, &settings.log.level, settings.log.rotate);
    // Shared state
    let state = Arc::new(HInnerAppState::new(settings.cache.folder));

    // Cleanup task
    let state_cleanup = state.clone();
    tokio::spawn(async move {
        state_cleanup
            .sol_mgr
            .periodic_cleanup(settings.server.solsys_cleanup_interval, settings.server.solsys_lifetime)
            .await
    });

    // HTTP routing
    let router = Router::new()
        .route("/", get(handlers::root))
        .route("/src/:alias", post(handlers::create_source))
        .route("/src/:alias", delete(handlers::delete_source))
        .route("/sol", post(handlers::create_sol))
        .route("/sol/:sol_id", get(handlers::get_sol))
        .route("/sol/:sol_id", patch(handlers::change_sol))
        .route("/sol/:sol_id", delete(handlers::delete_sol))
        .route("/sol/:sol_id/src", patch(handlers::change_sol_src))
        .route("/sol/:sol_id/fit", post(handlers::create_fit))
        .route("/sol/:sol_id/fit/:fit_id", get(handlers::get_fit))
        .route("/sol/:sol_id/fit/:fit_id", patch(handlers::change_fit))
        .route("/sol/:sol_id/fit/:fit_id", delete(handlers::delete_fit))
        .route("/sol/:sol_id/item", post(handlers::create_item))
        .route("/sol/:sol_id/item/:item_id", get(handlers::get_item))
        .route("/sol/:sol_id/item/:item_id", patch(handlers::change_item))
        .route("/sol/:sol_id/item/:item_id", delete(handlers::delete_item))
        .route("/sol/:sol_id/fleet", post(handlers::create_fleet))
        .route("/sol/:sol_id/fleet/:fleet_id", get(handlers::get_fleet))
        .route("/sol/:sol_id/fleet/:fleet_id", patch(handlers::change_fleet))
        .route("/sol/:sol_id/fleet/:fleet_id", delete(handlers::delete_fleet))
        // Debug handlers
        .route("/sol/:sol_id/check", get(handlers::debug_check_sol))
        .with_state(state);
    // Middleware
    let url_mid = NormalizePathLayer::trim_trailing_slash();
    let general_mid = tower::ServiceBuilder::new()
        .layer(RequestIdLayer)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &http::Request<Body>| {
                    let request_id = request
                        .extensions()
                        .get::<RequestId>()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "unknown".into());
                    tracing::trace_span!("http", id = %request_id)
                })
                .on_request(|request: &http::Request<Body>, _span: &Span| {
                    tracing::info!(">>> rx {} {}", request.method(), request.uri())
                })
                .on_response(|response: &http::Response<Body>, latency: Duration, _span: &Span| {
                    tracing::info!("<<< tx {} generated in {:?}", response.status(), latency)
                }),
        )
        .layer(middleware::from_fn(util::ml_trace_reqresp::print_request_response));
    // App
    let app = url_mid.layer(router.layer(general_mid));

    // Run server
    let addr = format!("127.0.0.1:{}", settings.server.port);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    tracing::debug!("listening on {addr}");
    axum::serve(listener, ServiceExt::<extract::Request>::into_make_service(app))
        .await
        .unwrap();
}
