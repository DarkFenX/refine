use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    handler::Handler,
    routing::{delete, get, post},
    Router,
};
use tower::ServiceBuilder;

use crate::state::AppState;

mod handlers;
mod state;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState::new());

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route(
            "/source/:alias",
            post(
                handlers::create_source.layer(
                    ServiceBuilder::new()
                        .layer(HandleErrorLayer::new(handlers::handle_error))
                        .timeout(Duration::from_secs(30))
                        .buffer(5)
                        .concurrency_limit(2),
                ),
            ),
        )
        .route("/source/:alias", delete(handlers::delete_source))
        .with_state(shared_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
