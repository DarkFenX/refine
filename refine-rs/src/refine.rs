// TODO: remove pub from things which are not supposed to be public
pub struct Refine {
    pub tpool: ThreadPool,
}
impl Refine {
    pub fn new(standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
        }
    }
}

pub struct ThreadPool {
    pub standard: tokio_rayon::rayon::ThreadPool,
    pub heavy: tokio_rayon::rayon::ThreadPool,
}
impl ThreadPool {
    fn new(standard_threads: usize, heavy_threads: usize) -> Self {
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
