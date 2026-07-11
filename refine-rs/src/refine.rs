use crate::{src::SrcMgr, tpool::ThreadPool};

// TODO: remove pub from things which are not supposed to be public
pub struct Refine {
    pub tpool: ThreadPool,
    pub src_mgr: SrcMgr,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            src_mgr: SrcMgr::new(cache_folder),
        }
    }
}
