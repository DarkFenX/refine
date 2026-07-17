/// Adapted data caching mode.
///
/// When core library receives EVE data, it processes it into format which is more convenient to
/// work with. This processing can take some time (usually a few seconds); if it is undesirable,
/// result can be cached somewhere - the cache is much faster to load.
#[derive(Clone, Default)]
pub enum AdCaching {
    #[default]
    Disabled,
    /// Store cache on filesystem, with path to directory
    #[cfg(feature = "adc-fs")]
    Filesystem(std::path::PathBuf),
}
