use crate::settings::SettingsNetwork;

pub(crate) async fn setup_listeners(settings: SettingsNetwork) -> Vec<tokio::net::TcpListener> {
    let mut port = settings.port;
    let mut listeners = Vec::new();
    let addrs = tokio::net::lookup_host((settings.address.as_str(), port))
        .await
        .unwrap();
    // Listen on every IP the address resolves to, skipping ones which cannot be bound
    for mut addr in addrs {
        addr.set_port(port);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                let local_addr = listener.local_addr().unwrap();
                // Process port of 0 (= allocate any unused port), so that it is consistent across
                // all addresses
                port = local_addr.port();
                tracing::debug!("listening on {local_addr}");
                listeners.push(listener);
            }
            Err(error) => tracing::warn!("unable to listen on {addr}: {error}"),
        }
    }
    assert!(
        !listeners.is_empty(),
        "unable to listen on any address of {}",
        settings.address
    );
    listeners
}
