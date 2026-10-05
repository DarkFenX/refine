#[derive(Clone)]
pub(crate) struct ShutdownData(tokio::sync::watch::Receiver<()>);

pub(crate) fn setup_shutdown() -> ShutdownData {
    // Dropping the sender wakes shutdown futures
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(());
    tokio::spawn(async {
        shutdown_signal().await;
        drop(shutdown_tx);
    });
    ShutdownData(shutdown_rx)
}

pub(crate) async fn wait_shutdown(mut data: ShutdownData) {
    // Nothing is ever sent, so it returns (with an error) only when the sender is dropped
    data.0.changed().await.unwrap_err();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.unwrap();
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::debug!("shutting down");
}
