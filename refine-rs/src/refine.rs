pub struct Refine {}

struct Settings {
    solsys_lifetime: u64,
    solsys_cleanup_interval: u64,
    standard_threads: usize,
    heavy_threads: usize,
    cache_folder: Option<String>,
}
