use tokio_rayon::AsyncThreadPool;

pub(crate) struct ThreadPool {
    standard: tokio_rayon::rayon::ThreadPool,
    heavy: tokio_rayon::rayon::ThreadPool,
}
impl ThreadPool {
    pub(super) fn new(standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            standard: tokio_rayon::rayon::ThreadPoolBuilder::new()
                .num_threads(standard_threads)
                .build()
                .unwrap(),
            heavy: tokio_rayon::rayon::ThreadPoolBuilder::new()
                .num_threads(heavy_threads)
                .build()
                .unwrap(),
        }
    }
    pub(crate) async fn exec_standard<F, R>(&self, func: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let sync_span = tracing::trace_span!("sync");
        self.standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                func()
            })
            .await
    }
    pub(crate) async fn exec_heavy<F, R>(&self, func: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let sync_span = tracing::trace_span!("sync");
        self.heavy
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                func()
            })
            .await
    }
}
