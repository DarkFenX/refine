mod err;
mod handlers;
mod logging;
mod middleware;
mod server;
mod settings;
mod state;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    server::setup_server().await;
}
