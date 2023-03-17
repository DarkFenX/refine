#![feature(hash_drain_filter)]

use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{settings::Settings, state::AppState};

mod handlers;
mod settings;
mod src_mgr;
mod ss_mgr;
mod state;
mod util;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config_path = env::args().nth(1);
    let settings = Settings::new(config_path).unwrap();
    let state = Arc::new(AppState::new(settings.server.cache_folder));

    let state_cleanup = state.clone();
    tokio::spawn(async move {
        state_cleanup
            .ss_mgr
            .periodic_cleanup(settings.server.solsys_cleanup_interval, settings.server.solsys_lifetime)
            .await
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/source/:alias", post(handlers::create_source))
        .route("/source/:alias", delete(handlers::delete_source))
        .route("/solar_system", post(handlers::create_sol_sys))
        .route("/solar_system/:id", delete(handlers::delete_sol_sys))
        .route("/solar_system/:id", patch(handlers::change_sol_sys))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
