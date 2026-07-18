#[derive(Clone)]
pub(crate) enum AdCaching {
    Disabled,
    #[cfg(feature = "adc-fs")]
    Filesystem {
        dir: std::path::PathBuf,
    },
}
