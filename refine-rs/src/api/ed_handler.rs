/// EVE data source type.
///
/// The handler is used to feed data into the core library - the data is needed to build
/// [`src::Src`](src::Src).
#[derive(Clone)]
pub enum EdSource {
    /// Phobos data dump, with directory on filesystem where it is stored.
    #[cfg(feature = "edh-phb-fs")]
    PhobosFilesystem { dir: std::path::PathBuf },
    /// Phobos data dump, with URL where it is served from.
    #[cfg(feature = "edh-phb-http")]
    PhobosHttp { data_version: String, base_url: String },
}
