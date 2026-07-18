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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner
////////////////////////////////////////////////////////////////////////////////////////////////////
struct InnerAppState {
    refine: Refine,
}
impl InnerAppState {
    fn new(standard_threads: usize, heavy_threads: usize, cache_dir: Option<std::path::PathBuf>) -> Self {
        let refine = match cache_dir {
            Some(cache_dir) => Refine::with_fs_adc(standard_threads, heavy_threads, cache_dir),
            None => Refine::new(standard_threads, heavy_threads),
        };
        InnerAppState { refine }
    }
}
