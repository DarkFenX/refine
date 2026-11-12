use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

pub(crate) struct ThreadPool {
    standard: rayon::ThreadPool,
    heavy: rayon::ThreadPool,
}
impl ThreadPool {
    pub(crate) fn new(standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            standard: rayon::ThreadPoolBuilder::new()
                .num_threads(standard_threads)
                .build()
                .unwrap(),
            heavy: rayon::ThreadPoolBuilder::new()
                .num_threads(heavy_threads)
                .build()
                .unwrap(),
        }
    }
    pub(crate) fn get_threads_standard(&self) -> usize {
        self.standard.current_num_threads()
    }
    pub(crate) fn get_threads_heavy(&self) -> usize {
        self.heavy.current_num_threads()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ThreadPool {
    pub(crate) async fn exec_standard<F, R>(&self, func: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        exec(&self.standard, func).await
    }
    pub(crate) async fn exec_heavy<F, R>(&self, func: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        exec(&self.heavy, func).await
    }
    pub(crate) fn free_standard<T>(&self, value: T)
    where
        T: Send + 'static,
    {
        let free_span = tracing::error_span!("free");
        self.standard.spawn_fifo(move || {
            let _sg = free_span.enter();
            // A panic escaping a spawned rayon job aborts the process
            if catch_unwind(AssertUnwindSafe(move || drop(value))).is_err() {
                tracing::error!("panic while freeing removed data");
            }
        });
    }
}

/// A job whose future is dropped before the job starts is skipped. Once started, a job runs to
/// completion.
async fn exec<F, R>(pool: &rayon::ThreadPool, func: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let sync_span = tracing::error_span!("sync");
    let (result_tx, result_rx) = tokio::sync::oneshot::channel();
    pool.spawn_fifo(move || {
        let _sg = sync_span.enter();
        // The future was dropped while the job was queued, so nothing waits for its result
        if result_tx.is_closed() {
            tracing::debug!("skipping job of a cancelled request");
            return;
        }
        let _ = result_tx.send(catch_unwind(AssertUnwindSafe(func)));
    });
    match result_rx.await.expect("threadpool job dropped without running") {
        Ok(result) => result,
        Err(panic) => resume_unwind(panic),
    }
}
