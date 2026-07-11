pub(crate) struct ThreadPool {
    pub(crate) standard: tokio_rayon::rayon::ThreadPool,
    pub(crate) heavy: tokio_rayon::rayon::ThreadPool,
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
}
