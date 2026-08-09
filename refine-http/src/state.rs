use std::sync::Arc;

use rs::Refine;

#[derive(Clone)]
pub(crate) struct AppState(Arc<InnerAppState>);
impl AppState {
    pub(crate) fn new(standard_threads: usize, heavy_threads: usize, cache_dir: Option<std::path::PathBuf>) -> Self {
        Self(Arc::new(InnerAppState::new(standard_threads, heavy_threads, cache_dir)))
    }
    pub(crate) fn get_refine(&self) -> &Refine {
        &self.0.refine
    }
    pub(crate) fn get_cache_dir(&self) -> Option<&std::path::Path> {
        self.0.cache_dir.as_deref()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner
////////////////////////////////////////////////////////////////////////////////////////////////////
struct InnerAppState {
    refine: Refine,
    cache_dir: Option<std::path::PathBuf>,
}
impl InnerAppState {
    fn new(standard_threads: usize, heavy_threads: usize, cache_dir: Option<std::path::PathBuf>) -> Self {
        InnerAppState {
            refine: Refine::new(standard_threads, heavy_threads),
            cache_dir,
        }
    }
}
